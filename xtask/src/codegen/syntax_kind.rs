use crate::utils::{fs2, project_root, CodegenMode};
use anyhow::{Context, Result};
use heck::ShoutySnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use ungrammar::Grammar;

const GRAMMAR: &str = include_str!("ddlog.ungram");
const EXTRA_TOKENS: &[&str] = &["comment", "whitespace", "eof", "tombstone"];

fn grammar() -> Result<Grammar> {
    GRAMMAR.parse().context("failed to parse ddlog grammar")
}

pub fn syntax_kind(mode: CodegenMode) -> Result<()> {
    match mode {
        CodegenMode::Run => eprintln!("running code generation..."),
        CodegenMode::Check => eprintln!("checking generated code..."),
    }

    let grammar = grammar()?;
    let target_path = project_root().join("../crates/ddlog-syntax/src/syntax_kind/generated.rs");

    // Sort the tokens so that their ordering is consistent across runs
    let mut tokens: Vec<_> = grammar
        .iter()
        .map(|node| &*grammar[node].name)
        .chain(grammar.tokens().map(|token| &*grammar[token].name))
        .chain(EXTRA_TOKENS.iter().copied())
        .collect();
    tokens.sort_unstable();

    if tokens.len() + 1 > u16::MAX as usize {
        anyhow::bail!("created more than {} SyntaxKind variants", u16::MAX);
    }

    let variants = tokens.iter().enumerate().map(|(idx, &token)| {
        let idx = idx as u16;
        let variant = token_variant(token);
        let attr = token_attr(token);

        quote! {
            #attr
            #variant = #idx,
        }
    });
    let error_idx = tokens.len() as u16;

    let debug_impl = generate_debug(&tokens);
    let display_impl = generate_display(&tokens);
    let trait_impls = trait_implementations();
    let token_macro = token_macro(&tokens);

    let code = quote! {
        #[derive(logos::Logos)]
        #[allow(
            non_camel_case_types,
            clippy::upper_case_acronyms,
            clippy::manual_non_exhaustive,
        )]
        #[repr(u16)]
        pub enum SyntaxKind {
            #(#variants)*

            /// An error within the syntax tree
            // Note: This must be the last variant of the enum so that it has
            //       the highest discriminant
            #[error]
            ERROR = #error_idx,
        }

        impl SyntaxKind {
            #[doc(hidden)]
            #[inline]
            pub const fn highest() -> Self {
                Self::ERROR
            }
        }

        #debug_impl
        #display_impl
        #trait_impls
        #token_macro
    }
    .to_string();

    fs2::update(target_path, &code, mode)?;

    match mode {
        CodegenMode::Run => eprintln!("finished running code generation"),
        CodegenMode::Check => eprintln!("finished checking generated code"),
    }
    Ok(())
}

const FUNKY_CHARS: &[&str] = &["(", ")", "{", "}", "[", "]"];

fn token_macro(tokens: &[&str]) -> TokenStream {
    let arms = tokens.iter().filter_map(|&token| {
        let ident = if KEYWORDS.contains(&token) {
            let ident = format_ident!("{}", token);
            quote! { #ident }
        } else if FUNKY_CHARS.contains(&token) {
            assert_eq!(token.chars().count(), 1);
            let char = token.chars().next().unwrap();

            quote! { #char }
        } else if NAMED_TOKENS
            .iter()
            .any(|&(named_token, _)| named_token == token)
        {
            token.parse().unwrap()
        } else {
            return None;
        };

        let variant = token_variant(token);
        Some(quote! {
            (#ident) => { $crate::SyntaxKind::#variant };
        })
    });

    quote! {
        #[macro_export]
        macro_rules! T {
            #(#arms)*
        }
    }
}

fn generate_display(tokens: &[&str]) -> TokenStream {
    let arms = tokens.iter().map(|&token| token_display(token));

    quote! {
        impl ::core::fmt::Display for SyntaxKind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    #(#arms)*
                    Self::ERROR => ::core::fmt::Formatter::write_str(f, "???"),
                }
            }
        }
    }
}

fn generate_debug(tokens: &[&str]) -> TokenStream {
    let arms = tokens.iter().map(|&token| token_debug(token));

    quote! {
        impl ::core::fmt::Debug for SyntaxKind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    #(#arms)*
                    Self::ERROR => ::core::fmt::Formatter::write_str(f, "ERROR"),
                }
            }
        }
    }
}

fn trait_implementations() -> TokenStream {
    quote! {
        impl ::core::convert::From<SyntaxKind> for ::cstree::SyntaxKind {
            #[inline]
            fn from(kind: SyntaxKind) -> Self {
                Self(kind as u16)
            }
        }

        impl ::core::convert::From<SyntaxKind> for u16 {
            #[inline]
            fn from(kind: SyntaxKind) -> Self {
                kind as u16
            }
        }

        const _: () = {
            #[cold]
            #[track_caller]
            #[inline(never)]
            fn invalid_syntax_kind(kind: u16) -> ! {
                ::core::panic!(
                    "invalid SyntaxKind '{}', must be within the range of 0..={}",
                    kind,
                    SyntaxKind::ERROR as u16,
                )
            }

            impl ::core::convert::From<u16> for SyntaxKind {
                #[inline]
                #[track_caller]
                fn from(kind: u16) -> Self {
                    if kind > Self::ERROR as u16 {
                        invalid_syntax_kind(kind)
                    } else {
                        // Safety: `kind` is a valid `SyntaxKind`
                        unsafe { ::core::mem::transmute::<u16, Self>(kind) }
                    }
                }
            }

            impl ::core::convert::From<::cstree::SyntaxKind> for SyntaxKind {
                #[inline]
                #[track_caller]
                fn from(::cstree::SyntaxKind(kind): ::cstree::SyntaxKind) -> Self {
                    if kind > Self::ERROR as u16 {
                        invalid_syntax_kind(kind)
                    } else {
                        // Safety: `kind` is a valid `SyntaxKind`
                        unsafe { ::core::mem::transmute::<u16, Self>(kind) }
                    }
                }
            }
        };

        impl ::core::clone::Clone for SyntaxKind {
            #[inline]
            #[must_use]
            fn clone(&self) -> Self {
                *self
            }

            #[inline]
            fn clone_from(&mut self, source: &Self) {
                *self = *source;
            }
        }
        impl ::core::marker::Copy for SyntaxKind {}

        impl ::core::hash::Hash for SyntaxKind {
            #[inline]
            fn hash<H>(&self, state: &mut H)
            where
                H: ::core::hash::Hasher,
            {
                ::core::hash::Hasher::write_u16(state, *self as u16);
            }
        }

        impl ::core::cmp::PartialEq<SyntaxKind> for SyntaxKind {
            #[inline]
            #[must_use]
            fn eq(&self, other: &SyntaxKind) -> bool {
                *self as u16 == *other as u16
            }
        }
        impl ::core::cmp::Eq for SyntaxKind {}

        impl ::core::cmp::PartialOrd<SyntaxKind> for SyntaxKind {
            #[inline]
            #[must_use]
            fn partial_cmp(&self, other: &SyntaxKind)
                -> ::core::option::Option<::core::cmp::Ordering>
            {
                (*self as u16).partial_cmp(&(*other as u16))
            }
        }

        impl ::core::cmp::Ord for SyntaxKind {
            #[inline]
            #[must_use]
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                (*self as u16).cmp(&(*other as u16))
            }
        }
    }
}

const NAMED_TOKENS: &[(&str, &str)] = &[
    ("=", "EQ"),
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
    ("&", "AMPERSAND"),
    (";", "SEMICOLON"),
    ("<=", "L_ANGLE_EQ"),
    (">=", "R_ANGLE_EQ"),
    ("=>", "RIGHT_ROCKET"),
];

const KEYWORDS: &[&str] = &[
    "function", "var", "match", "input", "output", "relation", "typedef", "for", "while", "loop",
    "apply", "extern", "and", "or", "if", "else", "return", "break", "true", "false",
];

const TOKEN_LOGOS: &[(&str, &[&str])] = &[
    ("ident", &["regex(\"[A-Za-z_][A-Za-z0-9_]*\")"]),
    ("whitespace", &["regex(\"[\\n\\t\\r ]+\")"]),
    ("comment", &["regex(\"//.*\")", "regex(\"///.*\")"]),
    ("bool", &["token(\"true\")", "token(\"false\")"]),
    (
        "number",
        &[
            "regex(\"[0-9][0-9_]*\")",
            "regex(\"0b[0-1][0-1_]*\")",
            "regex(\"0x[0-9a-fA-F][0-9a-fA-F_]*\")",
        ],
    ),
];

fn token_variant_string(data: &str) -> String {
    if let Some(&(_, name)) = NAMED_TOKENS.iter().find(|&&(token, _)| token == data) {
        name.to_owned()
    } else if KEYWORDS.contains(&data) {
        data.to_shouty_snake_case()
    } else if let Some(&(token, _)) = TOKEN_LOGOS.iter().find(|&&(token, _)| token == data) {
        token.to_shouty_snake_case()
    } else {
        data.to_shouty_snake_case()
    }
}

fn token_variant(token: &str) -> Ident {
    format_ident!("{}", token_variant_string(token))
}

fn token_attr(token: &str) -> TokenStream {
    if let Some(&(token, _)) = NAMED_TOKENS
        .iter()
        .find(|&&(named_token, _)| named_token == token)
    {
        quote! { #[token(#token)] }
    } else if KEYWORDS.contains(&token) {
        quote! { #[token(#token)] }
    } else if let Some(&(_, logos)) = TOKEN_LOGOS
        .iter()
        .find(|&&(logos_token, _)| logos_token == token)
    {
        let attrs = logos.iter().map(|&attr| {
            let attr: TokenStream = attr.parse().unwrap();
            quote! { #[#attr] }
        });

        quote! { #(#attrs)* }
    } else {
        TokenStream::new()
    }
}

fn token_debug(token: &str) -> TokenStream {
    let variant = token_variant(token);
    let debug = token_variant_string(token);

    quote! {
        Self::#variant => ::core::fmt::Formatter::write_str(f, #debug),
    }
}

fn token_display(data: &str) -> TokenStream {
    let display = if KEYWORDS.contains(&data) {
        quote! {
            ::core::fmt::Formatter::write_str(f, #data)
        }
    } else if NAMED_TOKENS
        .iter()
        .any(|&(named_token, _)| named_token == data)
    {
        if data.chars().count() == 1 {
            let name = data
                .chars()
                .next()
                .expect("there's exactly one char in the string");

            quote! {
                <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, #name)
            }
        } else {
            quote! {
                ::core::fmt::Formatter::write_str(f, #data)
            }
        }
    } else {
        let name = data.to_shouty_snake_case();
        quote! {
            ::core::fmt::Formatter::write_str(f, #name)
        }
    };

    let variant = token_variant(data);
    quote! {
        Self::#variant => #display,
    }
}
