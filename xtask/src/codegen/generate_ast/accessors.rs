use crate::codegen::generate_ast::{
    parser::{ChildKind, NodeKind, StructChild},
    utils::KEYWORDS,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn node_accessors(parent_struct: &Ident, children: &[StructChild]) -> TokenStream {
    let methods = children.iter().map(|child| {
        let index = child.index;
        let child_type = &child.child_type;
        let mut snake_case_name = child.snake_case_name.to_string();

        match child.kind {
            ChildKind::Normal | ChildKind::Optional => {
                let method = if KEYWORDS.contains(&&*snake_case_name) {
                    format_ident!("_{}", snake_case_name)
                } else {
                    child.snake_case_name.clone()
                };

                if index == 0 {
                    let support_func = match child.node_kind {
                        NodeKind::Syntax => quote![child],
                        NodeKind::Token => quote![token],
                    };

                    quote! {
                        #[inline]
                        pub fn #method(&self) -> ::core::option::Option<::std::borrow::Cow<'_, #child_type>> {
                            crate::ast::support::#support_func(&self.syntax)
                        }
                    }
                } else {
                    let support_func = match child.node_kind {
                        NodeKind::Syntax => quote![nth_child],
                        NodeKind::Token => quote![nth_token],
                    };

                    quote! {
                        #[inline]
                        pub fn #method(&self) -> ::core::option::Option<::std::borrow::Cow<'_, #child_type>> {
                            crate::ast::support::#support_func::<_, #index>(&self.syntax)
                        }
                    }
                }
            }

            ChildKind::Repeated => {
                if !snake_case_name.ends_with('s') {
                    snake_case_name.push('s');
                }

                let method = if KEYWORDS.contains(&&*snake_case_name) {
                    format_ident!("_{}", snake_case_name)
                } else {
                    format_ident!("{}", snake_case_name)
                };
                let (support_func, support_type) = match child.node_kind {
                    NodeKind::Syntax => (quote![children], quote![AstChildren]),
                    NodeKind::Token => (quote![token_children], quote![TokenChildren]),
                };

                quote! {
                    #[inline]
                    pub fn #method(&self) -> crate::ast::support::#support_type<'_, #child_type> {
                        crate::ast::support::#support_func(&self.syntax)
                    }
                }
            }
        }
    });

    quote! {
        impl #parent_struct {
            #(#methods)*
        }
    }
}
