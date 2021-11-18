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

pub fn generate_ast(mode: CodegenMode) -> Result<()> {
    match mode {
        CodegenMode::Run => println!("running code generation..."),
        CodegenMode::Check => println!("checking generated code..."),
    }

    let grammar = grammar()?;
    let (structs, enums, syntax_kinds, max_syntax_discriminant) = parser::from_grammar(&grammar)?;
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
                node_ast.extend(quote! {
                    #derives
                    pub enum #camel_case_name {
                        #(#enum_variants,)*
                    }
                });

                let ast_node_impl = enums::ast_node_of_nodes(&camel_case_name, &variants);
                node_ast.extend(ast_node_impl);
            }

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

/*
// TODO: Refactor and document this, it should be spread across files
// TODO: Sort any sort of inputs we get to ensure that we're as
//       deterministic as possible (maybe use a `Sorted` wrapper type?)

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

const FUNKY_CHARS: &[&str] = &["(", ")", "{", "}", "[", "]", "#[", "/*", "*/
"];

const KEYWORDS: &[&str] = &[
    "function", "let", "match", "input", "output", "relation", "struct", "for", "while", "loop",
    "apply", "extern", "and", "or", "if", "else", "return", "break", "true", "false", "multiset",
    "stream", "enum", "const", "as",
];



struct AstGenerator<'a> {
    grammar: &'a Grammar,
    mode: CodegenMode,
    ast_node_path: PathBuf,
    ast_token_path: PathBuf,
    ast_visitor_path: PathBuf,
    rust_keywords: Vec<Ident>,
    token_enums: Vec<String>,
}

impl<'a> AstGenerator<'a> {
    fn new(grammar: &'a Grammar, mode: CodegenMode) -> Result<Self> {
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

        let syntax_src_dir = project_root().join("crates/ddlog-syntax/src");
        let ast_node_path = syntax_src_dir.join("ast/generated/nodes.rs");
        let ast_token_path = syntax_src_dir.join("ast/generated/tokens.rs");
        let ast_visitor_path = syntax_src_dir.join("ast/generated/visitor.rs");
        let ast_module_path = syntax_src_dir.join("ast/generated/mod.rs");

        let rust_keywords = rust_keywords();

        let token_enums = grammar
            .iter()
            .filter_map(|node| {
                let node = &grammar[node];
                if let Rule::Alt(inner) = &node.rule {
                    if inner.iter().all(|rule| matches!(rule, Rule::Token(_))) {
                        return Some(node.name.to_owned());
                    }
                }

                None
            })
            .collect();

        let ast_module = "pub mod nodes;\npub mod tokens;\npub mod visitor;";
        fs2::update_formatted(&ast_module_path, ast_module, mode)?;

        Ok(Self {
            grammar,
            mode,
            ast_node_path,
            ast_token_path,
            ast_visitor_path,
            rust_keywords,
            token_enums,
        })
    }

    fn token_ast(&self, mut token_ast: TokenStream) -> Result<()> {
        let mut token_names = HashSet::new();

        let derives = quote! {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::cmp::PartialEq,
                ::core::cmp::Eq,
                ::core::hash::Hash,
            )]
        };

        let mut tokens: Vec<_> = self
            .grammar
            .tokens()
            .map(|token| &*self.grammar[token].name)
            .collect();
        tokens.sort_unstable();

        for token in tokens {
            if !token_names.insert(token) {
                anyhow::bail!("grammar contained multiple tokens named '{}'", token);
            }

            let token_name = self.ast_struct(token)?;
            token_ast.extend(quote! {
                #derives
                #[repr(transparent)]
                pub struct #token_name {
                    pub(crate) syntax: crate::SyntaxToken,
                }
            });

            token_ast.extend(self.implement_ast_token(token)?);
        }

        let token_ast = token_ast.to_string();
        fs2::update_formatted(&self.ast_token_path, &token_ast, self.mode)?;

        Ok(())
    }

    fn node_ast(&self) -> Result<TokenStream> {
        let (mut node_ast, mut token_ast) = (TokenStream::new(), TokenStream::new());
        let mut rule_names = HashSet::new();

        let mut nodes: Vec<_> = self
            .grammar
            .iter()
            .map(|node| {
                let node = &self.grammar[node];
                (&*node.name, &node.rule)
            })
            .collect();
        nodes.sort_unstable_by(|&(n1, _), &(n2, _)| n1.cmp(n2));

        let derives = quote! {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::cmp::PartialEq,
                ::core::cmp::Eq,
                ::core::hash::Hash,
            )]
        };

        for (name, rule) in nodes {
            if !rule_names.insert(&*name) {
                anyhow::bail!("grammar contained multiple nodes named '{}'", name);
            }

            let rule_name = self.ast_struct(name)?;
            let methods = self.gather_node_methods(rule, true, false, None)?;

            let ast_token_impl = if let Rule::Alt(inner) = rule {
                if inner.is_empty() {
                    anyhow::bail!(
                        "created an alternative rule with zero variants (must have at least 1): '{}'",
                        name,
                    );
                }
                if inner.len() > u8::MAX as usize {
                    anyhow::bail!(
                        "created an alternative rule with {} variants (must have less than {}): '{}'",
                        inner.len(),
                        u8::MAX,
                        name,
                    );
                }
// If every variant of the alternative is a token, the generated enum is a token
                let is_token = inner.iter().all(|rule| matches!(rule, Rule::Token(_)));

                let mut variants = inner
                    .iter()
                    .map(|rule| self.get_rule_name(rule))
                    .collect::<Result<Vec<_>, _>>()?;
                variants.sort_unstable_by(|(v1, _), (v2, _)| v1.cmp(v2));

                let variants = variants
                    .into_iter()
                    .map(|(variant, ty)| quote! { #variant(#ty), });

                let ast = if is_token {
                    &mut token_ast
                } else {
                    &mut node_ast
                };
                ast.extend(quote! {
                    #derives
                    #[repr(u8)]
                    pub enum #rule_name {
                        #(#variants)*
                    }
                });

                if is_token {
                    let token_name = self.ast_struct(name)?;
                    let variant_kinds = inner
                        .iter()
                        .map(|rule| self.get_rule_syntax_kind(rule))
                        .collect::<Result<Vec<_>>>()?;
                    let variant_names = inner
                        .iter()
                        .map(|rule| self.get_rule_name(rule).map(|(name, _)| name))
                        .collect::<Result<Vec<_>>>()?;

                    let cast_arms = inner
                        .iter()
                        .map(|rule| {
                            let kind = self.get_rule_syntax_kind(rule)?;
                            let (variant, ty) = self.get_rule_name(rule)?;

                            Ok(quote! {
                                crate::SyntaxKind::#kind => ::core::option::Option::Some(
                                    ::std::borrow::Cow::Owned(
                                        Self::#variant(
                                            #ty {
                                                syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(
                                                    syntax,
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            })
                        })
                        .collect::<Result<Vec<_>>>()?;

                    Some(quote! {
                        impl crate::ast::AstToken for #token_name {
                            #[inline]
                            fn can_cast_from(kind: crate::SyntaxKind) -> bool {
                                ::core::matches!(kind, #(crate::SyntaxKind::#variant_kinds)|*)
                            }

                            #[inline]
                            fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
                                match crate::SyntaxToken::kind(syntax) {
                                    #(#cast_arms)*
                                    _ => ::core::option::Option::None,
                                }
                            }

                            #[inline]
                            fn syntax(&self) -> &crate::SyntaxToken {
                                match self {
                                    #(Self::#variant_names(syntax) => syntax.syntax(),)*
                                }
                            }
                        }
                    })
                } else {
                    None
                }
            } else {
                node_ast.extend(quote! {
                    #derives
                    #[repr(transparent)]
                    pub struct #rule_name {
                        pub(crate) syntax: crate::SyntaxNode,
                    }
                });

                None
            };

            if !methods.is_empty() {
                let ast = if ast_token_impl.is_some() {
                    &mut token_ast
                } else {
                    &mut node_ast
                };

                ast.extend(quote! {
                    impl #rule_name {
                        #methods
                    }
                });
            }

            if let Some(ast_token_impl) = ast_token_impl {
                token_ast.extend(ast_token_impl);
            } else {
                node_ast.extend(self.implement_ast_node(rule, name)?);
            }
        }

        let node_ast = node_ast.to_string();
        fs2::update_formatted(&self.ast_node_path, &node_ast, self.mode)?;

        Ok(token_ast)
    }

// TODO: Finish visitor generation
// TODO: Maybe this all needs to be refactored by turning the ungrammar into
// a more real ast that defines structs, enums, fields, etc. and their canonical
// names so that codegen just consists of traversing the pre-processed tree
// instead of constantly re-generating and re-checking things while trying to make
// ad-hoc guesses at what the "correct" names of things are.
    fn ast_visitors(&self) -> Result<()> {
        let visitors: TokenStream = self
            .grammar
            .iter()
            .map(|node| {
                let node_data = &self.grammar[node];

                if let Rule::Alt(arms) = &node_data.rule {
                    if arms.iter().all(|arm| matches!(arm, Rule::Token(_))) {
                        return Ok(TokenStream::new());
                    }
                } else if matches!(node_data.rule, Rule::Token(_)) {
                    return Ok(TokenStream::new());
                }

                let struct_name = self.ast_struct(&node_data.name)?;
                let struct_path = quote! { crate::ast::nodes::#struct_name };

                let visitor_method =
                    format_ident!("visit_{}", self.ast_method(&node_data.name, false)?);
                let default_method =
                    format_ident!("default_{}", self.ast_method(&node_data.name, false)?);

                let body = self.default_visitor(&node_data.rule, Some(&node_data.name))?;

                let visitor = quote! {
                    #[inline]
                    #[doc(hidden)]
                    pub fn #default_method(&mut self, node: &#struct_path) {
                        #body
                    }

                    #[inline]
                    pub fn #visitor_method(&mut self, node: &#struct_path) {
                        self.#default_method(node);
                    }
                };

                Ok(visitor)
            })
            .collect::<Result<_>>()?;

        let visitors = quote! {
            pub trait AstVisitor {
                #visitors
            }
        };
        let visitors = visitors.to_string();
        fs2::update_formatted(&self.ast_visitor_path, &visitors, self.mode)?;

        Ok(())
    }

    fn default_visitor(&self, rule: &Rule, name: Option<&str>) -> Result<TokenStream> {
        match rule {
            Rule::Opt(rule) => {
                if matches!(&**rule, Rule::Token(_)) {
                    return Ok(TokenStream::new());
                }

                let method = name.map_or_else(
                    || {
                        self.get_rule_name(&**rule)
                            .unwrap_or_else(|_| (format_ident!("err"), TokenStream::new()))
                            .0
                    },
                    |name| self.ast_method(name, false).unwrap(),
                );
                let visitor = format_ident!("visit_{}", method);

                Ok(quote! {
                    if let ::core::option::Option::Some(node) = node.#method() {
                        self.#visitor(&*node)
                    }
                })
            }

            &Rule::Node(node) => {
                let data = &self.grammar[node];
                if matches!(data.rule, Rule::Token(_)) {
                    return Ok(TokenStream::new());
                }

                let method = self.ast_method(&data.name, false)?;
                let visitor = format_ident!("visit_{}", method);

                Ok(quote! {
                    if let ::core::option::Option::Some(node) = node.#method() {
                        self.#visitor(&*node)
                    }
                })
            }

            Rule::Seq(rules) => rules
                .iter()
                .map(|rule| self.default_visitor(rule, None))
                .collect(),

            Rule::Labeled { label, rule, .. } => self.default_visitor(rule, Some(label)),

            Rule::Alt(_) => Ok(TokenStream::new()),
            Rule::Rep(rule) => {
                if matches!(&**rule, Rule::Token(_)) {
                    return Ok(TokenStream::new());
                }

                let method = name.map_or_else(
                    || {
                        self.get_rule_name(&**rule).map_or_else(
                            |_| quote!(something_broke),
                            |(name, _)| name.to_token_stream(),
                        )
                    },
                    |name| self.ast_method(name, false).unwrap().to_token_stream(),
                );

                let visitor = method.to_string();
                let mut visitor = &*visitor;
                if let Some(visit) = visitor.strip_suffix('s') {
                    visitor = visit;
                }

                let visitor = format_ident!("visit_{}", visitor);

                Ok(quote! {
                    for node in node.#method() {
                        self.#visitor(::core::ops::Deref::deref(&node));
                    }
                })
            }

            Rule::Token(_) => Ok(TokenStream::new()),
        }
    }

    fn get_rule_name(&self, rule: &Rule) -> Result<(Ident, TokenStream)> {
        match rule {
            Rule::Labeled { label, .. } => {
                let ident = self.ast_struct(label)?;
                let ty = ident.to_token_stream();

                Ok((ident, ty))
            }

            &Rule::Node(rule) => {
                let ident = self.ast_struct(&self.grammar[rule].name)?;
                let ty = ident.to_token_stream();

                Ok((ident, ty))
            }

            &Rule::Token(token) => {
                let token = self.ast_struct(&self.grammar[token].name)?;
                let ty = quote! { crate::ast::tokens::#token };

                Ok((token, ty))
            }

            Rule::Seq(_) => anyhow::bail!(
                "direct sequences in alternatives are not supported, use a named rule (rule: {:?})",
                rule,
            ),

            Rule::Alt(_) => anyhow::bail!(
                "direct alternatives in alternatives are not supported, use a named rule (rule: {:?})",
                rule,
            ),

            Rule::Opt(_) => anyhow::bail!(
                "direct optionals in alternatives are not supported, use a named rule (rule: {:?})",
                rule,
            ),

            Rule::Rep(_) => anyhow::bail!(
                "direct repetition in alternatives are not supported, use a named rule (rule: {:?})",
                rule,
            ),
        }
    }

    fn get_rule_syntax_kind(&self, rule: &Rule) -> Result<Ident> {
        match rule {
            Rule::Labeled { label, .. } => self.syntax_kind_variant(label),

            &Rule::Node(rule) => self.syntax_kind_variant(&self.grammar[rule].name),

            &Rule::Token(token) => self.syntax_kind_variant(&self.grammar[token].name),

            Rule::Seq(_) => anyhow::bail!(
                "direct sequences in alternatives are not supported, use a named rule",
            ),

            Rule::Alt(_) => anyhow::bail!(
                "direct alternatives in alternatives are not supported, use a named rule",
            ),

            Rule::Opt(_) => anyhow::bail!(
                "direct optionals in alternatives are not supported, use a named rule",
            ),

            Rule::Rep(_) => anyhow::bail!(
                "direct repetition in alternatives are not supported, use a named rule",
            ),
        }
    }

    fn implement_ast_node(&self, rule: &Rule, name: &str) -> Result<TokenStream> {
        let node_name = self.ast_struct(name)?;
        let syntax_kind = self.syntax_kind_variant(name)?;

        if let Rule::Alt(rules) = rule {
            let any_are_tokens = rules.iter().any(|rule| matches!(rule, Rule::Token(_)));
            if any_are_tokens {
                anyhow::bail!(
                    "alternatives containing direct tokens are not supported, use named rules",
                );
            }

            let mut variants = rules
                .iter()
                .map(|rule| self.get_rule_name(rule))
                .collect::<Result<Vec<_>, _>>()?;
            variants.sort_unstable_by(|(v1, _), (v2, _)| v1.cmp(v2));

            let syntax_variants = variants.iter().map(|(variant, _)| {
                quote! {
                    Self::#variant(node) => node.syntax(),
                }
            });
            let variant_kinds = variants
                .iter()
                .map(|(variant, _)| self.syntax_kind_variant(&variant.to_string()))
                .collect::<Result<Vec<_>>>()?;
            let casted_variants = variants
                .iter()
                .map(|(variant, _)| {
                    let variant_kind = self.syntax_kind_variant(&variant.to_string())?;
                    let starts_with_vowel = node_name
                        .to_string()
                        .chars()
                        .next()
                        .map(|first| ['a', 'e', 'i', 'o', 'u'].contains(&first))
                        .unwrap_or(false);

// Statically format the panic message
                    let panic_message = format!(
                        "malformed codegen for casting SyntaxKind::{} into a{} {}::{}",
                        variant_kind,
                        if starts_with_vowel { "n" } else { "" },
                        node_name,
                        variant,
                    );

                    Ok(quote! {
                        crate::SyntaxKind::#variant_kind => {
                            let node = match #variant::cast(syntax) {
                                Some(node) => node,
                                None => {
                                    if ::core::cfg!(debug_assertions) {
                                        ::core::panic!(#panic_message)
                                    } else {
// Safety: The match guard validates the inner syntax kind
                                        unsafe { ::core::hint::unreachable_unchecked() }
                                    }
                                }
                            };

                            ::core::option::Option::Some(
                                ::std::borrow::Cow::Owned(
                                    Self::#variant(
                                        match node {
                                            ::std::borrow::Cow::Owned(owned) => owned,
                                            ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                                        }
                                    ),
                                ),
                            )
                        }
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(quote! {
                impl crate::ast::AstNode for #node_name {
                    #[inline]
                    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
                        ::core::matches!(kind, #(crate::SyntaxKind::#variant_kinds)|*)
                    }

                    #[inline]
                    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
                        match crate::SyntaxNode::kind(syntax) {
                            #(#casted_variants)*

                            _ => ::core::option::Option::None,
                        }
                    }

                    #[inline]
                    fn syntax(&self) -> &crate::SyntaxNode {
                        match self {
                            #(#syntax_variants)*
                        }
                    }
                }
            })
        } else {
            Ok(quote! {
                impl crate::ast::AstNode for #node_name {
                    #[inline]
                    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
                        kind == crate::SyntaxKind::#syntax_kind
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
            })
        }
    }

    fn implement_ast_token(&self, name: &str) -> Result<TokenStream> {
        let token_name = self.ast_struct(name)?;
        let syntax_kind = self.syntax_kind_variant(name)?;

        Ok(quote! {
            impl crate::ast::AstToken for #token_name {
                #[inline]
                fn can_cast_from(kind: crate::SyntaxKind) -> bool {
                    kind == crate::SyntaxKind::#syntax_kind
                }

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

                #[inline]
                fn syntax(&self) -> &crate::SyntaxToken {
                    &self.syntax
                }
            }
        })
    }

// TODO: This is recursive, but does it *really* matter?
// TODO: Pluralize names when needed
    fn gather_node_methods(
        &self,
        rule: &Rule,
        top_level: bool,
        is_multiple: bool,
        name: Option<Ident>,
    ) -> Result<TokenStream> {
        match rule {
            Rule::Labeled { label, rule } => {
                debug_assert!(name.is_none());

                let method_name = format_ident!("{}", label);
                self.gather_node_methods(rule, false, is_multiple, Some(method_name))
            }

            &Rule::Node(node) => {
                let method_name = name.map_or_else(
                    || self.ast_method(&self.grammar[node].name, is_multiple),
                    Ok,
                )?;
                let node_name = self.ast_struct(&self.grammar[node].name)?;
                let is_token_enum = self.token_enums.contains(&self.grammar[node].name);

                let ret_ty = if is_token_enum {
                    if is_multiple {
                        quote! {
                            crate::ast::support::TokenChildren<'_, crate::ast::tokens::#node_name>
                        }
                    } else {
                        quote! {
                            ::core::option::Option<
                                ::std::borrow::Cow<'_, crate::ast::tokens::#node_name>
                            >
                        }
                    }
                } else if is_multiple {
                    quote! {
                        crate::ast::support::AstChildren<'_, crate::ast::nodes::#node_name>
                    }
                } else {
                    quote! {
                        ::core::option::Option<
                            ::std::borrow::Cow<'_, crate::ast::nodes::#node_name>
                        >
                    }
                };
                let support = if is_token_enum {
                    if is_multiple {
                        quote! {
                            crate::ast::support::token_children(&self.syntax)
                        }
                    } else {
                        quote! {
                            crate::ast::support::token(&self.syntax)
                        }
                    }
                } else if is_multiple {
                    quote! {
                        crate::ast::support::children(&self.syntax)
                    }
                } else {
                    quote! {
                        crate::ast::support::child(&self.syntax)
                    }
                };

                let method = quote! {
                    #[inline]
                    pub fn #method_name(&self) -> #ret_ty {
                        #support
                    }
                };

                Ok(method)
            }

            &Rule::Token(token) => {
                let method_name = name.map_or_else(
                    || self.ast_method(&self.grammar[token].name, is_multiple),
                    Ok,
                )?;
                let token_name = self.ast_struct(&self.grammar[token].name)?;

                let ret_ty = if is_multiple {
                    quote! {
                        crate::ast::support::TokenChildren<'_, crate::ast::tokens::#token_name>
                    }
                } else {
                    quote! {
                        ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::#token_name>>
                    }
                };
                let support = if is_multiple {
                    quote! {
                        crate::ast::support::token_children(&self.syntax)
                    }
                } else {
                    quote! {
                        crate::ast::support::token(&self.syntax)
                    }
                };

                Ok(quote! {
                    #[inline]
                    pub fn #method_name(&self) -> #ret_ty {
                        #support
                    }
                })
            }

// TODO: The rules aren't sorted, does that matter?
            Rule::Seq(rules) => {
                debug_assert!(name.is_none());
                debug_assert!(!is_multiple);

                let mut methods = TokenStream::new();
                for rule in rules {
                    let method = self.gather_node_methods(rule, false, false, None)?;
                    methods.extend(method);
                }

                Ok(methods)
            }

            Rule::Alt(rules) if top_level => {
                debug_assert!(name.is_none());
                debug_assert!(!is_multiple);

                let mut methods = TokenStream::new();
                for rule in rules {
                    let (variant_name, variant_ty) = self.get_rule_name(rule)?;

                    let variant_str = variant_name.to_string();
                    let as_cast = self.as_method_name(&variant_str, false)?;
                    let is_check = self.is_method_name(&variant_str, false)?;

                    methods.extend(quote! {
                        #[inline]
                        pub fn #as_cast(&self) -> ::core::option::Option<&#variant_ty> {
                            if let Self::#variant_name(node) = self {
                                ::core::option::Option::Some(node)
                            } else {
                                ::core::option::Option::None
                            }
                        }

                        #[inline]
                        pub fn #is_check(&self) -> bool {
                            ::core::matches!(self, Self::#variant_name(_))
                        }
                    });
                }

                Ok(methods)
            }

            Rule::Alt(_) => Ok(TokenStream::new()),

            Rule::Opt(rule) => self.gather_node_methods(rule, false, is_multiple, name),

            Rule::Rep(rule) => self.gather_node_methods(rule, false, true, name),
        }
    }

    fn ast_struct(&self, name: &str) -> Result<Ident> {
        let name = if let Some((_, special_name)) =
            NAMED_TOKENS.iter().find(|&&(token, _)| token == name)
        {
            special_name.to_camel_case()
        } else {
            name.to_camel_case()
        };

        let struct_name = format_ident!("{}", name);

        if self.rust_keywords.contains(&struct_name) {
            anyhow::bail!(
                "created ast struct named '{}', this will cause rust compilation errors",
                struct_name,
            );
        }

        Ok(struct_name)
    }

    fn ast_method(&self, mut name: &str, plural: bool) -> Result<Ident> {
        if let Some((_, special_name)) = NAMED_TOKENS.iter().find(|&&(token, _)| token == name) {
            name = special_name;
        }
        let plural = if plural && !name.ends_with('s') {
            "s"
        } else {
            ""
        };

        let method =
            if self
                .rust_keywords
                .contains(&format_ident!("{}{}", name.to_lowercase(), plural))
            {
                format_ident!("{}_token{}", name.to_snake_case(), plural)
            } else {
                format_ident!("{}{}", name.to_snake_case(), plural)
            };

        if self.rust_keywords.contains(&method) {
            anyhow::bail!(
                "created ast method named '{}{}', this will cause rust compilation errors",
                method,
                plural,
            );
        }

        Ok(method)
    }

    fn as_method_name(&self, mut name: &str, plural: bool) -> Result<Ident> {
        let plural = if plural { "s" } else { "" };
        if let Some((_, special_name)) = NAMED_TOKENS.iter().find(|&&(token, _)| token == name) {
            name = special_name;
        }

        let method =
            if self
                .rust_keywords
                .contains(&format_ident!("as_{}{}", name.to_lowercase(), plural))
            {
                format_ident!("as_{}_token{}", name.to_snake_case(), plural)
            } else {
                format_ident!("as_{}{}", name.to_snake_case(), plural)
            };

        if self.rust_keywords.contains(&method) {
            anyhow::bail!(
                "created ast method named '{}{}', this will cause rust compilation errors",
                method,
                plural,
            );
        }

        Ok(method)
    }

    fn is_method_name(&self, mut name: &str, plural: bool) -> Result<Ident> {
        let plural = if plural { "s" } else { "" };
        if let Some((_, special_name)) = NAMED_TOKENS.iter().find(|&&(token, _)| token == name) {
            name = special_name;
        }

        let method =
            if self
                .rust_keywords
                .contains(&format_ident!("is_{}{}", name.to_lowercase(), plural))
            {
                format_ident!("is_{}_token{}", name.to_snake_case(), plural)
            } else {
                format_ident!("is_{}{}", name.to_snake_case(), plural)
            };

        if self.rust_keywords.contains(&method) {
            anyhow::bail!(
                "created ast method named '{}{}', this will cause rust compilation errors",
                method,
                plural,
            );
        }

        Ok(method)
    }

    fn syntax_kind_variant(&self, token_name: &str) -> Result<Ident> {
        let variant = if let Some(&(_, name)) =
            NAMED_TOKENS.iter().find(|&&(token, _)| token == token_name)
        {
            format_ident!("{}", name)
        } else {
            format_ident!("{}", token_name.to_shouty_snake_case())
        };

        if self.rust_keywords.contains(&variant) {
            anyhow::bail!(
                "created syntax kind variant named '{}', this will cause rust compilation errors",
                variant,
            );
        }

        Ok(variant)
    }
}

fn generate_syntax_kind(tokens: &[&str], mode: CodegenMode) -> Result<()> {
    let target_path = project_root().join("crates/ddlog-syntax/src/syntax_kind/generated.rs");

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

    let debug_impl = generate_debug(tokens);
    let display_impl = generate_display(tokens);
    let trait_impls = trait_implementations();
    let token_macro = token_macro(tokens);

    let code = quote! {
        #[derive(logos::Logos)]
        #[allow(
            non_camel_case_types,
            clippy::upper_case_acronyms,
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

        fn lex_block_comment(lexer: &mut logos::Lexer<'_, SyntaxKind>) -> bool {
            let remainder = lexer.remainder();
            let mut nesting = 0;

            for (idx, _) in remainder.char_indices() {
                match remainder.get(idx..idx + 2) {
                    Some("* /") if nesting == 0 => {
                        lexer.bump(idx + 2);

                        return true;
                    }
                    Some("* /") => nesting -= 1,
                    Some("/ *") => nesting += 1,

                    Some(_) => continue,
                    None => break,
                }
            }

            false
        }

        #debug_impl
        #display_impl
        #trait_impls
        #token_macro
    }
    .to_string();

    fs2::update_formatted(target_path, &code, mode)?;

    Ok(())
}

fn token_macro(tokens: &[&str]) -> TokenStream {
    let arms = tokens.iter().filter_map(|&token| {
        let ident = if KEYWORDS.contains(&token) {
            let ident = format_ident!("{}", token);
            quote! { #ident }
        } else if FUNKY_CHARS.contains(&token) {
            let char_count = token.chars().count();
            assert!(char_count >= 1);

            if char_count == 1 {
                let char = token.chars().next().unwrap();

                quote! { #char }
            } else {
                quote! { #token }
            }
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

fn token_variant_string(data: &str) -> String {
    if let Some(&(_, name)) = NAMED_TOKENS.iter().find(|&&(token, _)| token == data) {
        name.to_owned()
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

/// A list of rust keywords so that we can warn the user about using them
fn rust_keywords() -> Vec<Ident> {
    vec![
        format_ident!("type"),
        format_ident!("return"),
        format_ident!("break"),
        format_ident!("continue"),
        format_ident!("match"),
        format_ident!("union"),
        format_ident!("fn"),
        format_ident!("enum"),
        format_ident!("struct"),
        format_ident!("impl"),
        format_ident!("abstract"),
        format_ident!("as"),
        format_ident!("const"),
        format_ident!("if"),
        format_ident!("else"),
        format_ident!("extern"),
        format_ident!("false"),
        format_ident!("true"),
        format_ident!("in"),
        format_ident!("for"),
        format_ident!("while"),
        format_ident!("loop"),
        format_ident!("mod"),
        format_ident!("move"),
        format_ident!("crate"),
        format_ident!("pub"),
        format_ident!("mut"),
        format_ident!("ref"),
        format_ident!("self"),
        format_ident!("Self"),
        format_ident!("static"),
        format_ident!("trait"),
        format_ident!("super"),
        format_ident!("unsafe"),
        format_ident!("use"),
        format_ident!("where"),
        format_ident!("let"),
        format_ident!("async"),
        format_ident!("await"),
        format_ident!("dyn"),
    ]
}
*/
