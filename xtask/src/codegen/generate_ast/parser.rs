use anyhow::Result;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::convert::TryInto;
use ungrammar::{Grammar, Rule};

/// The maximum number of allowed SyntaxKind variants
const MAXIMUM_SYNTAX_KINDS: usize = u16::MAX as usize;

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
    ("->", "RIGHT_ARROW"),
    ("::", "DOUBLE_COLON"),
    ("=>", "RIGHT_ROCKET"),
    ("&=", "AMPERSAND_EQ"),
    ("string_literal", "STRING_LITERAL"),
    ("number_literal", "NUMBER_LITERAL"),
];

const FUNKY_CHARS: &[&str] = &["(", ")", "{", "}", "[", "]", "#[", "/*", "*/"];

// Sourced from https://doc.rust-lang.org/reference/keywords.html
const RUST_KEYWORDS: &[&str] = &[
    "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "union", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

type StructuredGrammar = (Vec<Struct>, Vec<Enum>, Vec<SyntaxKindEntry>, u16);

pub fn from_grammar(grammar: &Grammar) -> Result<StructuredGrammar> {
    assert_constants_are_unique();

    let nodes: Vec<_> = grammar.iter().map(|node| &grammar[node]).collect();

    let (mut structs, mut enums) = (Vec::new(), Vec::new());
    for token in grammar
        .tokens()
        .map(|token| &*grammar[token].name)
        .chain(EXTRA_TOKENS.iter().copied())
    {
        structs.push(if RUST_KEYWORDS.contains(&token) {
            let kw = format!("{}_kw", token);

            Struct {
                raw_name: token.to_owned(),
                camel_case_name: Ident::new(&kw.to_camel_case(), Span::mixed_site()),
                snake_case_name: Ident::new(&kw.to_snake_case(), Span::mixed_site()),
                screaming_snake_case_name: Ident::new(
                    &kw.to_shouty_snake_case(),
                    Span::mixed_site(),
                ),
                kind: NodeKind::Token,
            }
        } else {
            Struct {
                raw_name: token.to_owned(),
                camel_case_name: camel_case_name(token),
                snake_case_name: snake_case_name(token),
                screaming_snake_case_name: screaming_snake_case_name(token),
                kind: NodeKind::Token,
            }
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
            structs.push(Struct {
                raw_name: node.name.clone(),
                camel_case_name: camel_case_name(&node.name),
                snake_case_name: snake_case_name(&node.name),
                screaming_snake_case_name: screaming_snake_case_name(&node.name),
                kind,
            });
        }
    }

    structs.sort_unstable_by_key(|s| s.camel_case_name.clone());
    enums.sort_unstable_by_key(|e| e.camel_case_name.clone());

    let (syntax_kinds, maximum_syntax_discriminant) = collect_syntax_kinds(&structs, &enums)?;

    Ok((structs, enums, syntax_kinds, maximum_syntax_discriminant))
}

fn collect_syntax_kinds(structs: &[Struct], enums: &[Enum]) -> Result<(Vec<SyntaxKindEntry>, u16)> {
    let mut syntax_kinds: Vec<_> = structs
        .iter()
        .map(|s| (s.raw_name.clone(), s.screaming_snake_case_name.clone()))
        // TODO: This may be correct?
        // .chain(enums.iter().filter_map(|e| {
        //     if e.kind == EnumKind::NodeOfTokens {
        //         Some((e.raw_name.clone(), e.screaming_snake_case_name.clone()))
        //     } else {
        //         None
        //     }
        // }))
        .chain(
            enums
                .iter()
                .map(|e| (e.raw_name.clone(), e.screaming_snake_case_name.clone())),
        )
        .map(|(raw_name, screaming_snake_case)| {
            let debug_string = screaming_snake_case.to_string();

            let display_string = if RUST_KEYWORDS.contains(&&*raw_name)
                || NAMED_TOKENS
                    .iter()
                    .any(|&(named_token, _)| named_token == raw_name)
            {
                raw_name.clone()
            } else {
                debug_string.clone()
            };

            let macro_ident = if RUST_KEYWORDS.contains(&&*raw_name) {
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
            }
        })
        .collect();

    // Make sure we have less than MAXIMUM_SYNTAX_KINDS variants
    if syntax_kinds.len() > MAXIMUM_SYNTAX_KINDS as usize {
        anyhow::bail!(
            "tried to create more than {} SyntaxKind variants (total variants: {})",
            MAXIMUM_SYNTAX_KINDS,
            syntax_kinds.len(),
        );
    }

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

    // Give all the syntax kinds explicit discriminants
    for (idx, kind) in syntax_kinds.iter_mut().enumerate() {
        // We've already made sure that there's less than or equal to `MAXIMUM_SYNTAX_KINDS`
        // variants, so this should fit fine
        kind.discriminant = idx.try_into().unwrap();
    }

    let maximum_syntax_discriminant = (syntax_kinds.len() - 1).try_into().unwrap();

    Ok((syntax_kinds, maximum_syntax_discriminant))
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
    let (variant_name, variant_type, syntax_kind, snake_case) = match rule {
        &Rule::Node(node) => {
            let node = &grammar[node];
            let variant_name = camel_case_name(&node.name);
            let variant_type = quote![crate::ast::nodes::#variant_name];
            let syntax_kind = screaming_snake_case_name(&node.name);
            let snake_case = node.name.to_snake_case();

            (variant_name, variant_type, syntax_kind, snake_case)
        }

        &Rule::Token(token) => {
            let token = &grammar[token];
            let variant_name = camel_case_name(&token.name);
            let variant_type = quote![crate::ast::tokens::#variant_name];
            let syntax_kind = screaming_snake_case_name(&token.name);
            let snake_case = token.name.to_snake_case();

            (variant_name, variant_type, syntax_kind, snake_case)
        }

        Rule::Labeled { .. } | Rule::Seq(_) | Rule::Alt(_) | Rule::Opt(_) | Rule::Rep(_) => {
            todo!("error")
        }
    };

    EnumVariant {
        variant_name,
        variant_type,
        syntax_kind,
        snake_case,
    }
}

fn camel_case_name(token: &str) -> Ident {
    if let Some(&(_, symbol)) = NAMED_TOKENS.iter().find(|&&(symbol, _)| symbol == token) {
        Ident::new(&symbol.to_camel_case(), Span::mixed_site())
    } else if !token.chars().all(|char| char.is_alphabetic()) || token.is_empty() {
        // TODO: Bail w/ error
        panic!(
            "non-ascii token {:?}, consider adding to the `NAMED_TOKENS` constant in xtask",
            token,
        );
    } else {
        Ident::new(&token.to_camel_case(), Span::mixed_site())
    }
}

fn snake_case_name(token: &str) -> Ident {
    if let Some(&(_, symbol)) = NAMED_TOKENS.iter().find(|&&(symbol, _)| symbol == token) {
        Ident::new(&symbol.to_snake_case(), Span::mixed_site())
    } else if RUST_KEYWORDS.contains(&token) {
        Ident::new(
            &format!("{}_token", token.to_snake_case()),
            Span::mixed_site(),
        )
    } else if !token.chars().all(|char| char.is_alphabetic()) || token.is_empty() {
        // TODO: Bail w/ error
        panic!(
            "non-ascii token {:?}, consider adding to the `NAMED_TOKENS` constant in xtask",
            token,
        );
    } else {
        Ident::new(&token.to_snake_case(), Span::mixed_site())
    }
}

fn screaming_snake_case_name(token: &str) -> Ident {
    if let Some(&(_, symbol)) = NAMED_TOKENS.iter().find(|&&(symbol, _)| symbol == token) {
        Ident::new(&symbol.to_shouty_snake_case(), Span::mixed_site())
    } else if !token.chars().all(|char| char.is_alphabetic()) || token.is_empty() {
        // TODO: Bail w/ error
        panic!(
            "non-ascii token {:?}, consider adding to the `NAMED_TOKENS` constant in xtask",
            token,
        );
    } else {
        Ident::new(&token.to_shouty_snake_case(), Span::mixed_site())
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

    let keywords = RUST_KEYWORDS
        .iter()
        .all(|&kw| RUST_KEYWORDS.iter().filter(|&&kw2| kw2 == kw).count() == 1);
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
}

#[derive(Debug)]
pub struct Struct {
    pub raw_name: String,
    pub camel_case_name: Ident,
    pub snake_case_name: Ident,
    pub screaming_snake_case_name: Ident,
    pub kind: NodeKind,
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

#[derive(Debug)]
pub enum NodeKind {
    Syntax,
    Token,
}
