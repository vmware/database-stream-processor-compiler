use crate::codegen::generate_ast::utils::KEYWORDS;
use anyhow::Result;
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{collections::HashMap, panic};
use ungrammar::{Grammar, Rule};

const EXTRA_TOKENS: &[&str] = &["comment", "whitespace", "eof", "tombstone", "error"];

const NAMED_TOKENS: &[(&str, &str)] = &[
    ("=", "EQ"),
    (".", "DOT"),
    ("!", "BANG"),
    ("|", "PIPE"),
    ("+", "PLUS"),
    ("*", "STAR"),
    ("!=", "NEQ"),
    ("<<", "SHL"),
    (">>", "SHR"),
    ("^", "CARET"),
    (",", "COMMA"),
    (":", "COLON"),
    ("-", "MINUS"),
    ("/", "SLASH"),
    ("==", "EQEQ"),
    ("<", "L_ANGLE"),
    (">", "R_ANGLE"),
    ("[", "L_BRACK"),
    ("]", "R_BRACK"),
    ("{", "L_CURLY"),
    ("}", "R_CURLY"),
    ("(", "L_PAREN"),
    (")", "R_PAREN"),
    ("%", "PERCENT"),
    ("<<=", "SHL_EQ"),
    (">>=", "SHR_EQ"),
    ("+=", "PLUS_EQ"),
    ("*=", "STAR_EQ"),
    ("|=", "PIPE_EQ"),
    ("-=", "MINUS_EQ"),
    ("/=", "SLASH_EQ"),
    ("^=", "CARET_EQ"),
    ("&", "AMPERSAND"),
    (";", "SEMICOLON"),
    ("%=", "PERCENT_EQ"),
    ("<=", "L_ANGLE_EQ"),
    (">=", "R_ANGLE_EQ"),
    ("#[", "HASH_BRACK"),
    ("..", "DOUBLE_DOT"),
    ("->", "RIGHT_ARROW"),
    ("::", "DOUBLE_COLON"),
    ("=>", "RIGHT_ROCKET"),
    ("&=", "AMPERSAND_EQ"),
    ("..=", "DOUBLE_DOT_EQ"),
    (":-", "HORN_IMPLICATION"),
];

const FUNKY_CHARS: &[&str] = &["(", ")", "{", "}", "[", "]", "#[", "/*", "*/"];

type StructuredGrammar = (Vec<Struct>, Vec<Enum>, Vec<SyntaxKindEntry>);

pub fn from_grammar(grammar: &Grammar) -> Result<StructuredGrammar> {
    assert_constants_are_unique();

    let nodes: Vec<_> = grammar.iter().map(|node| &grammar[node]).collect();

    let (mut structs, mut enums) = (Vec::new(), Vec::new());
    for token in grammar
        .tokens()
        .map(|token| &*grammar[token].name)
        .chain(EXTRA_TOKENS.iter().copied())
    {
        structs.push(Struct {
            raw_name: token.to_owned(),
            camel_case_name: camel_case_name(token),
            snake_case_name: snake_case_name(token),
            screaming_snake_case_name: screaming_snake_case_name(token),
            kind: NodeKind::Token,
            // Direct tokens have no children
            children: Vec::new(),
        });
    }

    for node in nodes {
        let is_syntax_node = rule_is_syntax_node(&node.rule, true);
        let is_enum = rule_is_enum(&node.rule);

        let kind = if is_syntax_node {
            NodeKind::Syntax
        } else {
            NodeKind::Token
        };

        if is_enum {
            let kind = if all_variants_are_tokens(&node.rule) {
                EnumKind::NodeOfTokens
            } else {
                EnumKind::NodeOfNodes
            };

            let mut variants: Vec<_> = if let Rule::Alt(variants) = &node.rule {
                variants
                    .iter()
                    .map(|rule| rule_to_enum_variant(grammar, rule))
                    .collect()
            } else {
                panic!();
            };
            variants.sort_unstable_by_key(|variant| variant.variant_name.to_string());

            enums.push(Enum {
                raw_name: node.name.clone(),
                camel_case_name: camel_case_name(&node.name),
                snake_case_name: snake_case_name(&node.name),
                screaming_snake_case_name: screaming_snake_case_name(&node.name),
                variants,
                kind,
            });
        } else {
            fn children_inner(
                grammar: &Grammar,
                rule: &Rule,
                kind: ChildKind,
                children: &mut Vec<StructChild>,
            ) {
                match rule {
                    Rule::Labeled { label, rule } => {
                        let mut labeled = Vec::with_capacity(1);
                        children_inner(grammar, rule, kind, &mut labeled);
                        assert_eq!(labeled.len(), 1);

                        let mut child = labeled.remove(0);
                        child.snake_case_name = snake_case_name(label);
                        children.push(child);
                    }

                    &Rule::Node(node) => {
                        let node = &grammar[node];
                        let node_type = camel_case_name(&node.name);

                        let (node_kind, child_type) = if rule_is_syntax_node(rule, true) {
                            (NodeKind::Syntax, quote![crate::ast::nodes::#node_type])
                        } else {
                            (NodeKind::Token, quote![crate::ast::tokens::#node_type])
                        };

                        children.push(StructChild {
                            snake_case_name: snake_case_name(&node.name),
                            child_type,
                            node_kind,
                            kind,
                            index: 0,
                        });
                    }

                    &Rule::Token(token) => {
                        let token = &grammar[token];
                        let token_type = camel_case_name(&token.name);

                        children.push(StructChild {
                            snake_case_name: snake_case_name(&token.name),
                            child_type: quote![crate::ast::tokens::#token_type],
                            node_kind: NodeKind::Token,
                            kind,
                            index: 0,
                        });
                    }

                    Rule::Opt(rule) => {
                        children_inner(
                            grammar,
                            rule,
                            if kind == ChildKind::Repeated {
                                ChildKind::Repeated
                            } else {
                                ChildKind::Optional
                            },
                            children,
                        );
                    }

                    Rule::Rep(rule) => {
                        children_inner(grammar, rule, ChildKind::Repeated, children);
                    }

                    Rule::Seq(rules) => {
                        for rule in rules {
                            children_inner(grammar, rule, kind, children);
                        }
                    }

                    Rule::Alt(_) => {}
                }
            }

            let mut children = Vec::new();
            match &node.rule {
                &Rule::Node(node) => {
                    let node = &grammar[node];
                    let node_type = camel_case_name(&node.name);

                    let (kind, child_type) = if rule_is_syntax_node(&node.rule, true) {
                        (NodeKind::Syntax, quote![crate::ast::nodes::#node_type])
                    } else {
                        (NodeKind::Token, quote![crate::ast::tokens::#node_type])
                    };

                    children.push(StructChild {
                        snake_case_name: snake_case_name(&node.name),
                        child_type,
                        node_kind: kind,
                        kind: ChildKind::Normal,
                        index: 0,
                    });
                }

                &Rule::Token(token) => {
                    let token = &grammar[token];
                    let token_type = camel_case_name(&token.name);

                    children.push(StructChild {
                        snake_case_name: snake_case_name(&token.name),
                        child_type: quote![crate::ast::tokens::#token_type],
                        node_kind: NodeKind::Token,
                        kind: ChildKind::Normal,
                        index: 0,
                    });
                }

                Rule::Seq(sequence) => {
                    children.reserve(sequence.len());
                    for rule in sequence {
                        children_inner(grammar, rule, ChildKind::Normal, &mut children);
                    }
                }

                Rule::Opt(rule) => {
                    children_inner(grammar, rule, ChildKind::Optional, &mut children);
                }

                Rule::Rep(rule) => {
                    children_inner(grammar, rule, ChildKind::Repeated, &mut children);
                }

                Rule::Alt(_) => println!("top level alternative"),

                Rule::Labeled { rule, .. } => {
                    children_inner(grammar, rule, ChildKind::Normal, &mut children)
                }
            }

            // Iterate through children and generate their indexes
            // This will make the 1st occurrence of a given type have the index zero
            // and all others will have an incremented offset to use `nth_{child, token}` with
            let mut child_indices = HashMap::new();
            for child in &mut children {
                let index = *child_indices
                    .entry(child.child_type.to_string())
                    .and_modify(|index| *index += 1)
                    .or_default();

                child.index = index;
            }

            structs.push(Struct {
                raw_name: node.name.clone(),
                camel_case_name: camel_case_name(&node.name),
                snake_case_name: snake_case_name(&node.name),
                screaming_snake_case_name: screaming_snake_case_name(&node.name),
                kind,
                children,
            });
        }
    }

    structs.sort_unstable_by_key(|s| s.camel_case_name.clone());
    enums.sort_unstable_by_key(|e| e.camel_case_name.clone());

    let syntax_kinds = collect_syntax_kinds(&structs, &enums)?;

    Ok((structs, enums, syntax_kinds))
}

fn collect_syntax_kinds(structs: &[Struct], enums: &[Enum]) -> Result<Vec<SyntaxKindEntry>> {
    let mut syntax_kinds: Vec<_> = structs
        .iter()
        .map(|s| {
            (
                s.raw_name.clone(),
                s.screaming_snake_case_name.clone(),
                s.kind == NodeKind::Token,
            )
        })
        .chain(enums.iter().map(|e| {
            (
                e.raw_name.clone(),
                e.screaming_snake_case_name.clone(),
                false,
            )
        }))
        .map(|(raw_name, screaming_snake_case, is_token)| {
            let debug_string = screaming_snake_case.to_string();

            let display_string = if KEYWORDS.contains(&&*raw_name)
                || NAMED_TOKENS
                    .iter()
                    .any(|&(named_token, _)| named_token == raw_name)
            {
                raw_name.clone()
            } else {
                debug_string.clone()
            };

            let macro_ident = if KEYWORDS.contains(&&*raw_name) {
                let ident = format_ident!("{}", raw_name);
                Some(quote! { #ident })
            } else if FUNKY_CHARS.contains(&&*raw_name) {
                let char_count = raw_name.chars().count();
                assert!(char_count >= 1);

                Some(if char_count == 1 {
                    let char = raw_name.chars().next().unwrap();

                    quote! { #char }
                } else {
                    quote! { #raw_name }
                })
            } else if NAMED_TOKENS
                .iter()
                .any(|&(named_token, _)| named_token == raw_name)
            {
                Some(raw_name.parse().unwrap())
            } else {
                None
            };

            SyntaxKindEntry {
                raw_name,
                screaming_snake_case,
                debug_string,
                display_string,
                // We initialize everything with a zero discriminant until we've sorted the variants
                discriminant: 0,
                macro_ident,
                is_token,
            }
        })
        .collect();

    // Sort the syntax kinds lexicographically
    syntax_kinds.sort_unstable_by_key(|kind| kind.screaming_snake_case.clone());

    // Make sure there's no duplicates
    if let Some(duplicates) = syntax_kinds
        .windows(2)
        .find(|kinds| kinds[0].screaming_snake_case == kinds[1].screaming_snake_case)
    {
        anyhow::bail!(
            "found two occurrences of the '{}' SyntaxKind",
            duplicates[0].screaming_snake_case,
        );
    }

    Ok(syntax_kinds)
}

fn rule_is_syntax_node(rule: &Rule, top_level: bool) -> bool {
    match rule {
        Rule::Token(_) => top_level,

        // Labeled nodes always require a node
        Rule::Labeled { rule, .. } => rule_is_syntax_node(&**rule, false),

        // All of these rules imply the need for a `SyntaxNode`
        Rule::Node(_) | Rule::Seq(_) | Rule::Opt(_) | Rule::Rep(_) => true,

        // All alternatives are syntax nodes
        Rule::Alt(_) => true,
    }
}

fn all_variants_are_tokens(rule: &Rule) -> bool {
    if let Rule::Alt(variants) = rule {
        // TODO: Do we need to account for labeled variants?
        variants.iter().any(|rule| matches!(rule, Rule::Token(_)))
    } else {
        panic!("called `all_variants_are_tokens()` on a non-alt rule")
    }
}

fn rule_is_enum(rule: &Rule) -> bool {
    matches!(rule, Rule::Alt(_))
}

fn rule_to_enum_variant(grammar: &Grammar, rule: &Rule) -> EnumVariant {
    let (raw_name, variant_name, variant_type, syntax_kind, snake_case) = match rule {
        &Rule::Node(node) => {
            let node = &grammar[node];
            let variant_name = camel_case_name(&node.name);
            let variant_type = quote![crate::ast::nodes::#variant_name];
            let syntax_kind = screaming_snake_case_name(&node.name);
            let snake_case = node.name.to_snake_case();

            (
                raw_name(&node.name),
                variant_name,
                variant_type,
                syntax_kind,
                snake_case,
            )
        }

        &Rule::Token(token) => {
            let token = &grammar[token];
            let raw_name = raw_name(&token.name);
            let raw_name_str = raw_name.to_string();
            let variant_name = camel_case_name(&raw_name_str);
            let variant_type = quote![crate::ast::tokens::#variant_name];
            let syntax_kind = screaming_snake_case_name(&raw_name_str);
            let snake_case = raw_name_str.to_snake_case();

            (
                raw_name,
                variant_name,
                variant_type,
                syntax_kind,
                snake_case,
            )
        }

        Rule::Labeled { .. } | Rule::Seq(_) | Rule::Alt(_) | Rule::Opt(_) | Rule::Rep(_) => {
            todo!("error")
        }
    };

    EnumVariant {
        raw_name,
        variant_name,
        variant_type,
        syntax_kind,
        snake_case,
    }
}

fn raw_name(token: &str) -> Ident {
    if let Some(&(_, symbol)) = NAMED_TOKENS.iter().find(|&&(symbol, _)| symbol == token) {
        format_ident!("{}", symbol)
    } else if !token
        .chars()
        .all(|char| char.is_alphabetic() || char == '_')
        || token.is_empty()
    {
        // TODO: Bail w/ error
        panic!(
            "non-ascii token {:?}, consider adding to the `NAMED_TOKENS` constant in xtask",
            token,
        );
    } else {
        format_ident!("{}", token)
    }
}

fn camel_case_name(token: &str) -> Ident {
    let camel_case = token.to_upper_camel_case();

    if let Some(&(_, symbol)) = NAMED_TOKENS.iter().find(|&&(symbol, _)| symbol == token) {
        format_ident!("{}", symbol.to_upper_camel_case())
    } else if KEYWORDS.contains(&&*camel_case) {
        format_ident!("{}Token", camel_case)
    } else if !token
        .chars()
        .all(|char| char.is_alphabetic() || char == '_')
        || token.is_empty()
    {
        // TODO: Bail w/ error
        panic!(
            "non-ascii token {:?}, consider adding to the `NAMED_TOKENS` constant in xtask",
            token,
        );
    } else {
        format_ident!("{}", camel_case)
    }
}

fn snake_case_name(token: &str) -> Ident {
    let snake = token.to_snake_case();

    if let Some(&(_, symbol)) = NAMED_TOKENS.iter().find(|&&(symbol, _)| symbol == token) {
        format_ident!("{}", symbol.to_snake_case())
    } else if KEYWORDS.contains(&&*snake) {
        format_ident!("{}_token", snake)
    } else if !token
        .chars()
        .all(|char| char.is_alphabetic() || char == '_')
        || token.is_empty()
    {
        // TODO: Bail w/ error
        panic!(
            "non-ascii token {:?}, consider adding to the `NAMED_TOKENS` constant in xtask",
            token,
        );
    } else {
        format_ident!("{}", snake)
    }
}

fn screaming_snake_case_name(token: &str) -> Ident {
    let screaming = token.to_shouty_snake_case();

    if let Some(&(_, symbol)) = NAMED_TOKENS.iter().find(|&&(symbol, _)| symbol == token) {
        format_ident!("{}", symbol.to_shouty_snake_case())
    } else if KEYWORDS.contains(&token) {
        format_ident!("{}_TOKEN", screaming)
    } else if !token
        .chars()
        .all(|char| char.is_alphabetic() || char == '_')
        || token.is_empty()
    {
        // TODO: Bail w/ error
        panic!(
            "non-ascii token {:?}, consider adding to the `NAMED_TOKENS` constant in xtask",
            token,
        );
    } else {
        format_ident!("{}", screaming)
    }
}

// TODO: Bail on error
fn assert_constants_are_unique() {
    let extra_tokens = EXTRA_TOKENS
        .iter()
        .all(|&t| EXTRA_TOKENS.iter().filter(|&&t2| t2 == t).count() == 1);
    assert!(extra_tokens);

    let funky_chars = FUNKY_CHARS
        .iter()
        .all(|&c| FUNKY_CHARS.iter().filter(|&&c2| c2 == c).count() == 1);
    assert!(funky_chars);

    let keywords = KEYWORDS
        .iter()
        .all(|&kw| KEYWORDS.iter().filter(|&&kw2| kw2 == kw).count() == 1);
    assert!(keywords);

    let named_tokens = NAMED_TOKENS.iter().all(|&(token, _)| {
        NAMED_TOKENS
            .iter()
            .filter(|&&(token2, _)| token == token2)
            .count()
            == 1
    });
    assert!(named_tokens);

    let named_tokens = NAMED_TOKENS.iter().all(|&(_, symbol)| {
        symbol.chars().all(|c| c.is_uppercase() || c == '_')
            && NAMED_TOKENS
                .iter()
                .filter(|&&(_, symbol2)| symbol == symbol2)
                .count()
                == 1
    });
    assert!(named_tokens);
}

#[derive(Debug)]
pub struct SyntaxKindEntry {
    pub raw_name: String,
    /// The variant's name in SCREAMING_SNAKE_CASE
    pub screaming_snake_case: Ident,
    pub debug_string: String,
    pub display_string: String,
    /// The variant's discriminant within the enum
    pub discriminant: u16,
    pub macro_ident: Option<TokenStream>,
    pub is_token: bool,
}

#[derive(Debug)]
pub struct Struct {
    pub raw_name: String,
    pub camel_case_name: Ident,
    pub snake_case_name: Ident,
    pub screaming_snake_case_name: Ident,
    pub kind: NodeKind,
    pub children: Vec<StructChild>,
}

#[derive(Debug)]
pub struct StructChild {
    pub snake_case_name: Ident,
    pub child_type: TokenStream,
    pub node_kind: NodeKind,
    pub kind: ChildKind,
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildKind {
    Normal,
    Repeated,
    Optional,
}

#[derive(Debug)]
pub struct Enum {
    pub raw_name: String,
    pub camel_case_name: Ident,
    pub snake_case_name: Ident,
    pub screaming_snake_case_name: Ident,
    pub variants: Vec<EnumVariant>,
    pub kind: EnumKind,
}

#[derive(Debug)]
pub struct EnumVariant {
    pub raw_name: Ident,
    pub variant_name: Ident,
    pub variant_type: TokenStream,
    pub syntax_kind: Ident,
    pub snake_case: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumKind {
    /// This is an enum where all variants are tokens
    NodeOfTokens,
    /// This is an enum where all variants are other syntax nodes
    NodeOfNodes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Syntax,
    Token,
}
