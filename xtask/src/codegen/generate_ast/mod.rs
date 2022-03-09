use crate::{
    codegen::{
        generate_ast::parser::{Enum, EnumKind, EnumVariant, NodeKind, Struct},
        grammar,
    },
    utils::{fs2, project_root, CodegenMode},
};
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod accessors;
mod enums;
mod parser;
mod structs;
mod syntax_kind;
mod utils;

pub fn generate_ast(mode: CodegenMode) -> Result<()> {
    match mode {
        CodegenMode::Run => println!("running code generation..."),
        CodegenMode::Check => println!("checking generated code..."),
    }

    let grammar = grammar()?;
    let (structs, enums, mut syntax_kinds) = parser::from_grammar(&grammar)?;
    let derives = default_derives();

    let (mut token_ast, mut node_ast) = (TokenStream::new(), TokenStream::new());

    // Create all cst structs
    for Struct {
        camel_case_name,
        screaming_snake_case_name,
        kind,
        children,
        ..
    } in structs
    {
        let syntax = match kind {
            NodeKind::Syntax => format_ident!("SyntaxNode"),
            NodeKind::Token => format_ident!("SyntaxToken"),
        };

        let decl = quote! {
            #derives
            #[repr(transparent)]
            pub struct #camel_case_name {
                syntax: crate::#syntax,
            }
        };

        match kind {
            NodeKind::Syntax => {
                node_ast.extend(decl);

                let accessors = accessors::node_accessors(&camel_case_name, &children);
                node_ast.extend(accessors);

                let ast_node_impl =
                    structs::ast_node_for_struct(&camel_case_name, &screaming_snake_case_name);
                node_ast.extend(ast_node_impl);
            }

            NodeKind::Token => {
                token_ast.extend(decl);

                let ast_token_impl =
                    structs::ast_token_for_struct(&camel_case_name, &screaming_snake_case_name);
                token_ast.extend(ast_token_impl);
            }
        }
    }

    // Create all cst enums
    for Enum {
        raw_name,
        camel_case_name,
        screaming_snake_case_name,
        variants,
        kind,
        ..
    } in enums
    {
        let enum_variants = variants.iter().map(|variant| {
            let EnumVariant {
                variant_name,
                variant_type,
                ..
            } = variant;

            quote!(#variant_name(#variant_type))
        });

        match kind {
            EnumKind::NodeOfNodes => {
                // Remove the syntax kinds for enums of nodes since they don't actually exist
                // within the CST
                let idx = syntax_kinds
                    .iter()
                    .position(|entry| entry.raw_name == raw_name)
                    .expect("every enum should have a corresponding syntax kind");
                syntax_kinds.remove(idx);

                node_ast.extend(quote! {
                    #derives
                    pub enum #camel_case_name {
                        #(#enum_variants,)*
                    }
                });

                let ast_node_impl = enums::ast_node_of_nodes(&camel_case_name, &variants);
                node_ast.extend(ast_node_impl);
            }

            // Enums of tokens have a corresponding node generated for them, we need their syntax kinds
            EnumKind::NodeOfTokens => {
                node_ast.extend(quote! {
                    #derives
                    #[repr(transparent)]
                    pub struct #camel_case_name {
                        syntax: crate::SyntaxNode,
                    }
                });

                let ast_node_impl =
                    enums::ast_node_of_tokens(&camel_case_name, &screaming_snake_case_name);
                node_ast.extend(ast_node_impl);
            }
        }
    }

    // Enumerate all the syntax kinds and give each of them a unique number to use as their
    // discriminant. We do this after we've filtered out all of the enums
    let max_syntax_discriminant = syntax_kind::enumerate_syntax_kinds(&mut syntax_kinds)?;
    let syntax_ast = syntax_kind::generate_syntax_kind(&syntax_kinds, max_syntax_discriminant)?;

    let root = project_root();
    let generated_dir = root.join("crates/ddlog-syntax/src/ast/generated");

    let ast_module = "pub mod nodes;\npub mod tokens;";
    fs2::update_formatted(generated_dir.join("mod.rs"), ast_module, mode)?;

    fs2::update_formatted(generated_dir.join("nodes.rs"), &node_ast.to_string(), mode)?;

    fs2::update_formatted(
        generated_dir.join("tokens.rs"),
        &token_ast.to_string(),
        mode,
    )?;

    fs2::update_formatted(
        root.join("crates/ddlog-syntax/src/syntax_kind/generated.rs"),
        &syntax_ast.to_string(),
        mode,
    )?;

    match mode {
        CodegenMode::Run => println!("finished running code generation"),
        CodegenMode::Check => println!("finished checking generated code"),
    }

    Ok(())
}

/// Returns the default derives for both nodes and tokens
fn default_derives() -> TokenStream {
    quote! {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::cmp::PartialEq,
            ::core::cmp::Eq,
            ::core::hash::Hash,
        )]
    }
}
