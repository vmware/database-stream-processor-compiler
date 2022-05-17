use crate::codegen::generate_ast::parser::NodeKind;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Implements the `AstNode` trait for the given node struct
pub fn ast_node_for_struct(camel_case_name: &Ident, syntax_kind: &Ident) -> TokenStream {
    let (can_cast_doc, cast_doc) = node_docs(NodeKind::Syntax, syntax_kind);

    quote! {
        impl crate::ast::AstNode for #camel_case_name {
            #[doc = #can_cast_doc]
            #[inline]
            fn can_cast_from(kind: crate::SyntaxKind) -> bool {
                kind == crate::SyntaxKind::#syntax_kind
            }

            #[doc = #cast_doc]
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

            #[doc = "Returns a reference to the inner [`SyntaxNode`][crate::SyntaxNode]"]
            #[inline]
            fn syntax(&self) -> &crate::SyntaxNode {
                &self.syntax
            }
        }
    }
}

/// Implements the `AstToken` trait for the given token struct
pub fn ast_token_for_struct(camel_case_name: &Ident, syntax_kind: &Ident) -> TokenStream {
    let (can_cast_doc, cast_doc) = node_docs(NodeKind::Token, syntax_kind);

    quote! {
        impl crate::ast::AstToken for #camel_case_name {
            #[doc = #can_cast_doc]
            #[inline]
            fn can_cast_from(kind: crate::SyntaxKind) -> bool {
                kind == crate::SyntaxKind::#syntax_kind
            }

            #[doc = #cast_doc]
            #[inline]
            fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
                if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
                    // Safety: `Self` is `#[repr(transparent)]` around a `SyntaxToken`
                    //         so `&SyntaxToken` can be transmuted into an `&Self`
                    let node = unsafe {
                        ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax)
                    };

                    ::core::option::Option::Some(
                        ::std::borrow::Cow::Borrowed(node),
                    )
                } else {
                    ::core::option::Option::None
                }
            }

            #[doc = "Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]"]
            #[inline]
            fn syntax(&self) -> &crate::SyntaxToken {
                &self.syntax
            }
        }
    }
}

/// Generates the doc strings for the `can_cast_from()` and `cast()` methods
fn node_docs(kind: NodeKind, syntax_kind: &Ident) -> (String, String) {
    let kind = match kind {
        NodeKind::Syntax => "SyntaxNode",
        NodeKind::Token => "SyntaxToken",
    };

    let can_cast_doc = format!(
        " Returns `true` if the given [`SyntaxKind`] is a [`{syntax_kind}`]\n \
        [`SyntaxKind`]: crate::SyntaxKind\n \
        [`{syntax_kind}`]: crate::SyntaxKind::{syntax_kind}",
        syntax_kind = syntax_kind,
    );
    let cast_doc = format!(
        " Returns [`Some`] if the given [`{kind}`] has the [`{syntax_kind}`] [`SyntaxKind`]\n \
        [`Some`]: std::option::Option::Some\n \
        [`{kind}`]: crate::{kind}\n \
        [`{syntax_kind}`]: crate::SyntaxKind::{syntax_kind}\n \
        [`SyntaxKind`]: crate::SyntaxKind",
        syntax_kind = syntax_kind,
        kind = kind,
    );

    (can_cast_doc, cast_doc)
}
