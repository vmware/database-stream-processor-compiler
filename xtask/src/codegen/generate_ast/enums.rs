use crate::codegen::generate_ast::parser::{EnumVariant, NodeKind};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn ast_node_of_nodes(camel_case_name: &Ident, variants: &[EnumVariant]) -> TokenStream {
    let can_cast_from = generate_can_cast_from(variants);
    let cast_arms = generate_cast_arms(variants);
    let syntax_arms = generate_syntax_arms(NodeKind::Syntax, variants);
    let user_casts = generate_ast_user_casts(camel_case_name, variants);
    let from_impls = generate_enum_from_impls(camel_case_name, variants);
    let try_from_impls = generate_enum_try_from_impls(camel_case_name, variants);

    quote! {
        impl #camel_case_name {
            #(#user_casts)*
        }

        impl crate::ast::AstNode for #camel_case_name {
            #[inline]
            fn can_cast_from(kind: crate::SyntaxKind) -> bool {
                #(#can_cast_from)||*
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

        #(#from_impls)*
        #(#try_from_impls)*
    }
}

/// Generate utility checking and casting methods for enums
fn generate_ast_user_casts<'a>(
    parent_enum: &Ident,
    variants: &'a [EnumVariant],
) -> impl Iterator<Item = TokenStream> + 'a {
    // Generate `.is_variant() -> bool` methods
    let is_checks = variants.iter().map(|variant| {
        let method_name = format_ident!("is_{}", variant.snake_case);
        let variant = &variant.variant_name;

        quote! {
            pub fn #method_name(&self) -> bool {
                ::core::matches!(self, Self::#variant(..))
            }
        }
    });

    // Generate `.as_variant() -> Option<&Variant>` methods
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

    // Generate `.into_variant() -> Result<Variant, Parent>` methods
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

    // Generate `.to_variant() -> Variant` methods
    let parent_enum = parent_enum.to_string();
    let to_casts = variants.iter().map(move |variant| {
        let method_name = format_ident!("to_{}", variant.snake_case);
        let (variant, variant_type) = (&variant.variant_name, &variant.variant_type);
        let variant_str = format!("{variant}");

        quote! {
            #[track_caller]
            pub fn #method_name(self) -> #variant_type {
                if let Self::#variant(syntax) = self {
                    syntax
                } else {
                    crate::ast::support::failed_enum_to_node_cast(
                        #parent_enum,
                        #variant_str,
                        crate::SyntaxNode::kind(<Self as crate::ast::AstNode>::syntax(&self)),
                    )
                }
            }
        }
    });

    is_checks.chain(as_casts).chain(into_casts).chain(to_casts)
}

/// Generate `From` implementations for each variant type to the parent enum (`From<Child> for Parent`)
fn generate_enum_try_from_impls<'a>(
    parent_enum: &'a Ident,
    variants: &'a [EnumVariant],
) -> impl Iterator<Item = TokenStream> + 'a {
    variants.iter().map(move |variant| {
        let (variant, variant_type) = (&variant.variant_name, &variant.variant_type);

        quote! {
            impl ::core::convert::From<#variant_type> for #parent_enum {
                #[inline]
                fn from(value: #variant_type) -> Self {
                    Self::#variant(value)
                }
            }
        }
    })
}

/// Generate `TryFrom` implementations for each variant type from the parent enum (`TryFrom<Parent> for Child`)
fn generate_enum_from_impls<'a>(
    parent_enum: &'a Ident,
    variants: &'a [EnumVariant],
) -> impl Iterator<Item = TokenStream> + 'a {
    variants.iter().map(move |variant| {
        let (variant, variant_type) = (&variant.variant_name, &variant.variant_type);

        quote! {
            impl ::core::convert::TryFrom<#parent_enum> for #variant_type {
                type Error = #parent_enum;

                #[inline]
                fn try_from(value: #parent_enum) -> ::core::result::Result<Self, Self::Error> {
                    if let #parent_enum::#variant(this) = value {
                        ::core::result::Result::Ok(this)
                    } else {
                        ::core::result::Result::Err(value)
                    }
                }
            }
        }
    })
}

pub fn ast_node_of_tokens(
    camel_case_name: &Ident,
    syntax_kind_name: &Ident,
    variants: &[EnumVariant],
) -> TokenStream {
    let user_casts = generate_token_user_casts(variants);

    quote! {
        impl #camel_case_name {
            #(#user_casts)*
        }

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

fn generate_token_user_casts(variants: &[EnumVariant]) -> impl Iterator<Item = TokenStream> + '_ {
    // Generate `.is_variant() -> bool` and `.as_variant() -> Option<Cow<'_, Variant>>` methods
    variants.iter().map(|variant| {
        let snake_case_name = variant.raw_name.to_string().to_snake_case();
        let is_method_name = format_ident!("is_{}", snake_case_name);
        let as_method_name = format_ident!("as_{}", variant.raw_name.to_string().to_snake_case());
        let variant_type = &variant.variant_type;

        quote! {
            pub fn #is_method_name(&self) -> bool {
                crate::ast::support::token_exists::<#variant_type>(
                    <Self as crate::ast::AstNode>::syntax(self),
                )
            }

            pub fn #as_method_name(&self) -> ::core::option::Option<::std::borrow::Cow<'_, #variant_type>> {
                crate::ast::support::token::<#variant_type>(
                    <Self as crate::ast::AstNode>::syntax(self),
                )
            }
        }
    })
}

fn generate_can_cast_from(variants: &[EnumVariant]) -> impl Iterator<Item = TokenStream> + '_ {
    variants.iter().map(|variant| {
        let variant_type = &variant.variant_type;

        quote! {
            <#variant_type as crate::ast::AstNode>::can_cast_from(kind)
        }
    })
}

/// Generate the arms of the match statement within `AstNode::cast()`
fn generate_cast_arms(variants: &[EnumVariant]) -> impl Iterator<Item = TokenStream> + '_ {
    variants.iter().map(move |variant| {
        let EnumVariant {
            variant_name,
            variant_type,
            ..
        } = variant;

        let cast_trait = quote![crate::ast::AstNode];

        quote! {
            kind if <#variant_type as #cast_trait>::can_cast_from(kind) => {
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

/// Generate the arms of the match statement to get the inner syntax nodes from variants
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
