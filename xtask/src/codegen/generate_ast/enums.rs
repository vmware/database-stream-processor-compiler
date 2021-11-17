use crate::codegen::generate_ast::parser::{EnumVariant, NodeKind};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

pub fn ast_node_of_nodes(camel_case_name: &Ident, variants: &[EnumVariant]) -> TokenStream {
    let syntax_kinds = get_syntax_kinds(variants);
    let cast_arms = generate_cast_arms(NodeKind::Syntax, variants);
    let syntax_arms = generate_syntax_arms(NodeKind::Syntax, variants);
    let user_casts = generate_ast_user_casts(variants);

    quote! {
        impl #camel_case_name {
            #(#user_casts)*
        }

        impl crate::ast::AstNode for #camel_case_name {
            #[inline]
            fn can_cast_from(kind: crate::SyntaxKind) -> bool {
                ::core::matches!(kind, #(crate::SyntaxKind::#syntax_kinds)|*)
            }

            #[inline]
            fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
                match crate::SyntaxNode::kind(syntax) {
                    #(#cast_arms)*
                    _ => ::core::option::Option::None,
                }
            }

            #[inline]
            fn syntax(&self) -> &crate::SyntaxNode {
                match self {
                    #(#syntax_arms)*
                }
            }
        }
    }
}

fn generate_ast_user_casts(variants: &[EnumVariant]) -> impl Iterator<Item = TokenStream> + '_ {
    let is_checks = variants.iter().map(|variant| {
        let method_name = format_ident!("is_{}", variant.snake_case);
        let variant = &variant.variant_name;

        quote! {
            pub fn #method_name(&self) -> bool {
                ::core::matches!(self, Self::#variant(..))
            }
        }
    });

    let as_casts = variants.iter().map(|variant| {
        let method_name = format_ident!("as_{}", variant.snake_case);
        let (variant, variant_type) = (&variant.variant_name, &variant.variant_type);

        quote! {
            pub fn #method_name(&self) -> ::core::option::Option<&#variant_type> {
                if let Self::#variant(syntax) = self {
                    ::core::option::Option::Some(syntax)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    });

    let into_casts = variants.iter().map(|variant| {
        let method_name = format_ident!("into_{}", variant.snake_case);
        let (variant, variant_type) = (&variant.variant_name, &variant.variant_type);

        quote! {
            pub fn #method_name(self) -> ::core::result::Result<#variant_type, Self> {
                if let Self::#variant(syntax) = self {
                    ::core::result::Result::Ok(syntax)
                } else {
                    ::core::result::Result::Err(self)
                }
            }
        }
    });

    is_checks.chain(as_casts).chain(into_casts)
}

pub fn ast_node_of_tokens(camel_case_name: &Ident, syntax_kind_name: &Ident) -> TokenStream {
    quote! {
        impl crate::ast::AstNode for #camel_case_name {
            #[inline]
            fn can_cast_from(kind: crate::SyntaxKind) -> bool {
                kind == crate::SyntaxKind::#syntax_kind_name
            }

            #[inline]
            fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
                if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
                    // Safety: `Self` is `#[repr(transparent)]` around a `SyntaxNode`
                    //         so `&SyntaxNode` can be transmuted into an `&Self`
                    let node = unsafe {
                        ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax)
                    };

                    ::core::option::Option::Some(
                        ::std::borrow::Cow::Borrowed(node),
                    )
                } else {
                    ::core::option::Option::None
                }
            }

            #[inline]
            fn syntax(&self) -> &crate::SyntaxNode {
                &self.syntax
            }
        }
    }
}

fn get_syntax_kinds(variants: &[EnumVariant]) -> impl Iterator<Item = &Ident> + '_ {
    variants.iter().map(|variant| &variant.syntax_kind)
}

fn generate_cast_arms(
    parent_kind: NodeKind,
    variants: &[EnumVariant],
) -> impl Iterator<Item = TokenStream> + '_ {
    let (parent_cast_trait, kind_ident) = match parent_kind {
        NodeKind::Syntax => (
            quote![crate::ast::AstNode],
            Ident::new("SyntaxNode", Span::mixed_site()),
        ),
        NodeKind::Token => (
            quote![crate::ast::AstToken],
            Ident::new("SyntaxToken", Span::mixed_site()),
        ),
    };

    variants.iter().map(move |variant| {
        let EnumVariant {
            variant_name,
            variant_type,
            syntax_kind,
            ..
        } = variant;

        // let cast_trait = match kind {
        //     NodeKind::Syntax => quote![crate::ast::AstNode],
        //     NodeKind::Token => quote![crate::ast::AstToken],
        // };
        let cast_trait = quote![crate::ast::AstNode];

        quote! {
            crate::SyntaxKind::#syntax_kind => {
                ::std::debug_assert!(<Self as #parent_cast_trait>::can_cast_from(
                    crate::#kind_ident::kind(syntax),
                ));

                let node = match <#variant_type as #cast_trait>::cast(syntax) {
                    ::core::option::Option::Some(node) => ::std::borrow::Cow::into_owned(node),
                    ::core::option::Option::None => if ::core::cfg!(debug_assertions) {
                        ::core::unreachable!()
                    } else {
                        // Safety: We've already checked that the target node is castable
                        unsafe { ::core::hint::unreachable_unchecked() }
                    },
                };

                ::core::option::Option::Some(
                    ::std::borrow::Cow::Owned(
                        Self::#variant_name(node),
                    ),
                )
            }
        }
    })
}

fn generate_syntax_arms(
    kind: NodeKind,
    variants: &[EnumVariant],
) -> impl Iterator<Item = TokenStream> + '_ {
    let syntax_trait = match kind {
        NodeKind::Syntax => quote![crate::ast::AstNode],
        NodeKind::Token => quote![crate::ast::AstToken],
    };

    variants.iter().map(move |variant| {
        let EnumVariant {
            variant_type,
            variant_name,
            ..
        } = variant;

        quote! {
            Self::#variant_name(syntax) => {
                <#variant_type as #syntax_trait>::syntax(syntax)
            }
        }
    })
}
