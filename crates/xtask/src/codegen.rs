use anyhow::{Context, Result};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::{
    fs::{self},
    path::Path,
};
use ungrammar::{Grammar, Token, TokenData};

use crate::utils;

const GRAMMAR: &str = include_str!("ddlog.ungram");
const TARGET_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../ddlog-syntax/src/parser/generated.rs",
);

fn grammar() -> Result<Grammar> {
    GRAMMAR.parse().context("failed to parse ddlog grammar")
}

pub fn codegen() -> Result<()> {
    let grammar = grammar()?;

    // Sort the tokens so that their ordering is consistent across runs
    let mut tokens: Vec<_> = grammar.tokens().collect();
    tokens.sort();

    if tokens.len() + 1 > u16::MAX as usize {
        anyhow::bail!("created more than {} SyntaxKind variants", u16::MAX);
    }

    let variants = tokens.iter().enumerate().map(|(idx, &token)| {
        let idx = idx as u16;
        let data = &grammar[token];
        let variant = token_variant(data);
        let attr = token_attr(data);

        quote! {
            #attr
            #variant = #idx,
        }
    });
    let error_idx = tokens.len() as u16;

    let debug_impl = generate_debug(&grammar, &tokens);
    let display_impl = generate_display(&grammar, &tokens);
    let trait_impls = trait_implementations();
    let token_macro = token_macro(&grammar, &tokens);

    let mut code = quote! {
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

        #debug_impl
        #display_impl
        #trait_impls
        #token_macro
    }
    .to_string();
    utils::normalize(&mut code);

    let target_path = Path::new(TARGET_PATH);

    // The current value of the codegen'd file, we don't want to mess with it if we have
    // no changes since that'll trigger rustc's recompilation machinery
    let mut current = if !target_path.exists() {
        String::new()
    } else {
        fs::read_to_string(&target_path)
            .with_context(|| format!("failed to read current value of {}", target_path.display()))?
    };
    utils::normalize(&mut current);

    if code != current {
        fs::write(&target_path, code)
            .with_context(|| format!("failed to write new code to {}", target_path.display()))?;
    }

    Ok(())
}

const FUNKY_CHARS: &[&str] = &["(", ")", "{", "}", "[", "]"];

fn token_macro(grammar: &Grammar, tokens: &[Token]) -> TokenStream {
    let arms = tokens.iter().filter_map(|&token| {
        let data = &grammar[token];

        let ident = if KEYWORDS.contains(&&*data.name) {
            let ident = format_ident!("{}", data.name);
            quote! { #ident }
        } else if FUNKY_CHARS.contains(&&*data.name) {
            assert_eq!(data.name.chars().count(), 1);
            let char = data.name.chars().next().unwrap();

            quote! { #char }
        } else if NAMED_TOKENS.iter().any(|&(token, _)| token == data.name) {
            data.name.parse().unwrap()
        } else {
            return None;
        };

        let variant = token_variant(data);
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

fn generate_display(grammar: &Grammar, tokens: &[Token]) -> TokenStream {
    let arms = tokens.iter().map(|&token| {
        let data = &grammar[token];
        token_display(data)
    });

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

fn generate_debug(grammar: &Grammar, tokens: &[Token]) -> TokenStream {
    let arms = tokens.iter().map(|&token| {
        let data = &grammar[token];
        token_debug(data)
    });

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
    ("(", "L_PAREN"),
    (")", "R_PAREN"),
    ("{", "L_CURLY"),
    ("}", "R_CURLY"),
    ("[", "L_BRACK"),
    ("]", "R_BRACK"),
    (",", "COMMA"),
    (":", "COLON"),
    ("!", "BANG"),
    ("+", "PLUS"),
    ("-", "MINUS"),
    ("*", "STAR"),
    ("/", "SLASH"),
    ("=", "EQ"),
    ("!=", "NEQ"),
    ("==", "EQEQ"),
];

const KEYWORDS: &[&str] = &[
    "function", "var", "match", "input", "output", "relation", "typedef", "for", "while", "loop",
    "apply", "extern",
];

const TOKEN_LOGOS: &[(&str, &str)] = &[("ident", "regex(\"[A-Za-z_][A-Za-z0-9_]*\")")];

fn token_variant_string(data: &TokenData) -> String {
    if let Some(&(_, name)) = NAMED_TOKENS.iter().find(|&&(token, _)| token == data.name) {
        name.to_owned()
    } else if KEYWORDS.contains(&&*data.name) {
        data.name.to_uppercase()
    } else if let Some(&(token, _)) = TOKEN_LOGOS.iter().find(|&&(token, _)| token == data.name) {
        token.to_uppercase()
    } else {
        data.name.to_uppercase()
    }
}

fn token_variant(data: &TokenData) -> Ident {
    format_ident!("{}", token_variant_string(data))
}

fn token_attr(data: &TokenData) -> TokenStream {
    if let Some(&(token, _)) = NAMED_TOKENS.iter().find(|&&(token, _)| token == data.name) {
        quote! { #[token(#token)] }
    } else if KEYWORDS.contains(&&*data.name) {
        let keyword = &data.name;
        quote! { #[token(#keyword)] }
    } else if let Some(&(_, logos)) = TOKEN_LOGOS.iter().find(|&&(token, _)| token == data.name) {
        let logos: TokenStream = logos.parse().unwrap();
        quote! { #[#logos] }
    } else {
        TokenStream::new()
    }
}

fn token_debug(data: &TokenData) -> TokenStream {
    let variant = token_variant(data);
    let debug = token_variant_string(data);

    quote! {
        Self::#variant => ::core::fmt::Formatter::write_str(f, #debug),
    }
}

fn token_display(data: &TokenData) -> TokenStream {
    let display = if KEYWORDS.contains(&&*data.name) {
        let keyword = &data.name;
        quote! {
            ::core::fmt::Formatter::write_str(f, #keyword)
        }
    } else {
        let name = data.name.to_uppercase();

        if name.chars().count() == 1 {
            let name = name.chars().next().unwrap();
            quote! {
                <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, #name)
            }
        } else {
            quote! {
                ::core::fmt::Formatter::write_str(f, #name)
            }
        }
    };

    let variant = token_variant(data);
    quote! {
        Self::#variant => #display,
    }
}
