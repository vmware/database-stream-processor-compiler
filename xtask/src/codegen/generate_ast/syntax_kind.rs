use crate::codegen::generate_ast::parser::SyntaxKindEntry;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

macro_rules! special_logos {
    ($($name:literal => { $($attr:literal),+ $(,)? }),* $(,)?) => {
        const SPECIAL_LOGOS: &[(&str, &[&str])] = &[$(($name, &[$($attr,)+]),)*];
    };
}

// These are pairs of tokens and special logos attributes that they need
special_logos! {
    "ident" => { "regex(\"[A-Za-z_'][A-Za-z0-9_']*\")" },
    "whitespace" => { "regex(\"[\\n\\t\\r ]+\")" },

    // Note that these regexae don't include the trailing newlines, this
    // is on purpose. If they keep ahold of the newline then we'll create
    // lsp spans that cross line boundaries which is bad
    "comment" => {
        "regex(\"//.*\")",
        // TODO: Make this a separate doc comment
        "regex(\"///.*\")",
        // Block comments
        "token(\"/*\", lex_block_comment)",
    },

    // TODO: May not need this??
    "bool" => {
        "token(\"true\")",
        "token(\"false\")",
    },

    "number_literal" => {
        "regex(\"[0-9][0-9_]*\")",
        "regex(\"0b[0-1][0-1_]*\")",
        "regex(\"0x[0-9a-fA-F][0-9a-fA-F_]*\")",
    },

    "string_literal" => {
        // FIXME: Handle string escapes
        // FIXME: Raw strings
        // FIXME: Inline expressions
        // FIXME: Interned strings
        r##"regex(r#"b?"(\\.|[^\\"])*""#)"##,
    },

    "char_literal" => {
        "regex(\"b?'[^']*'\")",
    },

    "error" => { "error" },
}

macro_rules! special_displays {
    ($($name:literal => $display:literal),* $(,)?) => {
        const SPECIAL_DISPLAYS: &[(&str, &str)] = &[$(($name, $display),)*];
    };
}

special_displays! {
    "error" => "???",
}

pub fn generate_syntax_kind(
    kinds: &[SyntaxKindEntry],
    maximum_discriminant: u16,
) -> Result<TokenStream> {
    let variants = kinds.iter().map(|kind| {
        let (variant, discriminant) = (&kind.screaming_snake_case, kind.discriminant);

        if let Some((_, attrs)) = SPECIAL_LOGOS
            .iter()
            .find(|(name, _)| name == &kind.raw_name)
        {
            let attrs = attrs
                .iter()
                .map(|attr| attr.parse::<TokenStream>().unwrap());

            quote! {
                #(#[#attrs])*
                #variant = #discriminant,
            }
        } else if kind.is_token {
            let token = &kind.raw_name;

            quote! {
                #[token(#token)]
                #variant = #discriminant,
            }
        } else {
            quote! {
                #variant = #discriminant,
            }
        }
    });

    let debug_impl = generate_debug(kinds);
    let display_impl = generate_display(kinds);
    let trait_impl = trait_implementations(maximum_discriminant);
    let token_macro = token_macro(kinds);

    Ok(quote! {
        #[derive(logos::Logos)]
        #[allow(
            non_camel_case_types,
            clippy::upper_case_acronyms,
        )]
        #[repr(u16)]
        pub enum SyntaxKind {
            #(#variants)*
        }

        impl SyntaxKind {
            /// The maximum discriminant of the [`SyntaxKind`] enum
            pub const MAXIMUM_DISCRIMINANT: u16 = #maximum_discriminant;
        }

        fn lex_block_comment(lexer: &mut logos::Lexer<'_, SyntaxKind>) -> bool {
            let remainder = lexer.remainder();
            let mut nesting = 0;

            for (idx, _) in remainder.char_indices() {
                match remainder.get(idx..idx + 2) {
                    Some("*/") if nesting == 0 => {
                        lexer.bump(idx + 2);

                        return true;
                    }
                    Some("*/") => nesting -= 1,
                    Some("/*") => nesting += 1,

                    Some(_) => continue,
                    None => break,
                }
            }

            false
        }

        #debug_impl
        #display_impl
        #trait_impl
        #token_macro
    })
}

fn token_macro(tokens: &[SyntaxKindEntry]) -> TokenStream {
    let arms = tokens.iter().filter_map(|kind| {
        let (macro_ident, variant) = (kind.macro_ident.as_ref()?, &kind.screaming_snake_case);

        Some(quote! {
            (#macro_ident) => { $crate::SyntaxKind::#variant };
        })
    });

    quote! {
        /// A convenience macro to create tokens with
        #[macro_export]
        macro_rules! T {
            #(#arms)*
        }
    }
}

fn generate_display(kinds: &[SyntaxKindEntry]) -> TokenStream {
    let arms = kinds.iter().map(|kind| {
        let variant = &kind.screaming_snake_case;
        let display = if let Some(&(_, display)) = SPECIAL_DISPLAYS
            .iter()
            .find(|(name, _)| name == &kind.raw_name)
        {
            display
        } else {
            &kind.display_string
        };

        if display.chars().count() == 1 {
            let only_char = display.chars().next().unwrap();

            quote! {
                Self::#variant => {
                    <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, #only_char)
                }
            }
        } else {
            quote! {
                Self::#variant => ::core::fmt::Formatter::write_str(f, #display),
            }
        }
    });

    quote! {
        impl ::core::fmt::Display for SyntaxKind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    #(#arms)*
                }
            }
        }
    }
}

fn generate_debug(kinds: &[SyntaxKindEntry]) -> TokenStream {
    let arms = kinds.iter().map(|kind| {
        let (variant, debug) = (&kind.screaming_snake_case, &kind.debug_string);

        quote! {
            Self::#variant => ::core::fmt::Formatter::write_str(f, #debug),
        }
    });

    quote! {
        impl ::core::fmt::Debug for SyntaxKind {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    #(#arms)*
                }
            }
        }
    }
}

fn trait_implementations(maximum_discriminant: u16) -> TokenStream {
    let invalid_syntax_kind_message = format!(
        "invalid SyntaxKind '{{}}', must be within the range of 0..={}",
        maximum_discriminant,
    );

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
                    #invalid_syntax_kind_message,
                    kind,
                )
            }

            impl ::core::convert::From<u16> for SyntaxKind {
                #[inline]
                #[track_caller]
                fn from(kind: u16) -> Self {
                    if kind > #maximum_discriminant {
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
                    if kind > #maximum_discriminant {
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
