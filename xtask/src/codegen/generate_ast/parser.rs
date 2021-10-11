use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use proc_macro2::{Ident, Span};
use ungrammar::{Grammar, Rule};

const EXTRA_TOKENS: &[&str] = &["comment", "whitespace", "eof", "tombstone"];

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
];

const FUNKY_CHARS: &[&str] = &["(", ")", "{", "}", "[", "]", "#[", "/*", "*/"];

const KEYWORDS: &[&str] = &[
    "function", "let", "match", "input", "output", "relation", "struct", "for", "while", "loop",
    "apply", "extern", "and", "or", "if", "else", "return", "break", "true", "false", "multiset",
    "stream", "enum", "const", "as", "type", "abstract", "final", "async", "await", "unsafe",
    "static",
];

pub fn from_grammar(grammar: &Grammar) -> (Vec<Struct>, Vec<Enum>) {
    assert_constants_are_unique();

    let nodes: Vec<_> = grammar.iter().map(|node| &grammar[node]).collect();
    dbg!(&nodes);

    let (mut structs, mut enums) = (Vec::new(), Vec::new());

    for token in grammar
        .tokens()
        .map(|token| &*grammar[token].name)
        .chain(EXTRA_TOKENS.iter().copied())
    {
        structs.push(Struct {
            camel_case_name: camel_case_name(token),
            snake_case_name: snake_case_name(token),
            screaming_snake_case_name: screaming_snake_case_name(token),
            kind: NodeKind::Token,
        });
    }

    for node in nodes {
        let is_syntax_node = rule_is_syntax_node(&node.rule);
        let is_enum = rule_is_enum(&node.rule);

        println!(
            "{:?} = syntax node: {}, enum: {}",
            node, is_syntax_node, is_enum,
        );

        let kind = if is_syntax_node {
            NodeKind::Syntax
        } else {
            NodeKind::Token
        };

        if is_enum {
            let variants = if let Rule::Alt(variants) = &node.rule {
                variants
                    .iter()
                    .map(|rule| rule_to_enum_variant(grammar, rule))
                    .collect()
            } else {
                panic!();
            };

            enums.push(Enum {
                camel_case_name: camel_case_name(&node.name),
                snake_case_name: snake_case_name(&node.name),
                screaming_snake_case_name: screaming_snake_case_name(&node.name),
                kind,
                variants,
            });
        } else {
            structs.push(Struct {
                camel_case_name: camel_case_name(&node.name),
                snake_case_name: snake_case_name(&node.name),
                screaming_snake_case_name: screaming_snake_case_name(&node.name),
                kind,
            });
        }
    }

    structs.sort_unstable_by_key(|s| s.camel_case_name.clone());
    enums.sort_unstable_by_key(|e| e.camel_case_name.clone());
    dbg!(structs, enums)
}

fn rule_is_syntax_node(rule: &Rule) -> bool {
    fn rule_is_syntax_node_inner(rule: &Rule, top_level: bool) -> bool {
        match rule {
            Rule::Token(_) => false,

            // Labeled nodes always require a node
            Rule::Labeled { .. } => true, // rule_is_syntax_node_inner(&**rule, false),

            // All of these rules imply the need for a `SyntaxNode`
            Rule::Node(_) | Rule::Seq(_) | Rule::Opt(_) | Rule::Rep(_) => true,

            // Alternatives are only valid as a `SyntaxToken` if it's the top-level item
            // and all alternatives are tokens
            Rule::Alt(alternatives) if top_level => alternatives
                .iter()
                .any(|rule| rule_is_syntax_node_inner(rule, false)),

            // Non top-leveled alternatives are always syntax nodes
            Rule::Alt(_) => true,
        }
    }

    rule_is_syntax_node_inner(rule, true)
}

fn rule_is_enum(rule: &Rule) -> bool {
    matches!(rule, Rule::Alt(_))
}

fn rule_to_enum_variant(grammar: &Grammar, rule: &Rule) -> EnumVariant {
    match rule {
        &Rule::Node(node) => {
            let node = &grammar[node];
            let name = camel_case_name(&node.name);

            EnumVariant {
                variant_name: name.clone(),
                // TODO: Do a lookup to figure out the type's path and store
                // that instead of an ident
                variant_type: name,
            }
        }

        &Rule::Token(token) => {
            let token = &grammar[token];
            let name = camel_case_name(&token.name);

            EnumVariant {
                variant_name: name.clone(),
                // TODO: Do a lookup to figure out the type's path and store
                // that instead of an ident
                variant_type: name,
            }
        }

        Rule::Labeled { .. } | Rule::Seq(_) | Rule::Alt(_) | Rule::Opt(_) | Rule::Rep(_) => {
            todo!("error")
        }
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
    } else if KEYWORDS.contains(&token) {
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
pub struct Struct {
    pub camel_case_name: Ident,
    pub snake_case_name: Ident,
    pub screaming_snake_case_name: Ident,
    pub kind: NodeKind,
}

#[derive(Debug)]
pub struct Enum {
    pub camel_case_name: Ident,
    pub snake_case_name: Ident,
    pub screaming_snake_case_name: Ident,
    pub variants: Vec<EnumVariant>,
    pub kind: NodeKind,
}

#[derive(Debug)]
pub struct EnumVariant {
    pub variant_name: Ident,
    pub variant_type: Ident,
}

#[derive(Debug)]
pub enum NodeKind {
    Syntax,
    Token,
}
