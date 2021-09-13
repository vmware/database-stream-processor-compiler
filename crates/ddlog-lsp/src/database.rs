use crate::providers::utils;
use ddlog_diagnostics::{Diagnostic, FileId, Interner, Rope};
use ddlog_syntax::{
    ast::{
        nodes::{FunctionArg, FunctionDef, Pattern, Type},
        AstNode, AstToken,
    },
    match_ast, validation, visitor, AstVisitor, RuleCtx, SyntaxNode,
};
use lspower::lsp::{DocumentSymbol, Position, Range, SymbolKind, SymbolTag, Url};
use salsa::{Database, ParallelDatabase, Snapshot, Storage};
use std::{
    fmt::{self, Debug},
    ops::Deref,
};
use triomphe::{Arc, ThinArc};

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ArcSlice<T> {
    slice: ThinArc<(), T>,
}

impl<T> ArcSlice<T> {
    pub fn new<I>(slice: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        Self {
            slice: ThinArc::from_header_and_iter((), slice.into_iter()),
        }
    }
}

impl<T> Deref for ArcSlice<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.slice.slice
    }
}

impl<T> Clone for ArcSlice<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            slice: self.slice.clone(),
        }
    }
}

impl<T> Debug for ArcSlice<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.slice.slice).finish()
    }
}

#[salsa::query_group(SessionDatabase)]
pub trait Session {
    #[salsa::input]
    fn session(&self) -> Arc<crate::Session>;

    #[salsa::transparent]
    fn file_id(&self, url: &Url) -> FileId;
}

fn file_id(session: &dyn Session, url: &Url) -> FileId {
    FileId::new(session.session().interner().get_or_intern(url.as_str()))
}

#[salsa::query_group(SourceDatabase)]
pub trait Source: Session {
    #[salsa::input]
    fn file_source(&self, file: FileId) -> Rope;

    fn parsed(&self, file: FileId) -> (SyntaxNode, ArcSlice<Diagnostic>);

    fn parse_diagnostics(&self, file: FileId) -> ArcSlice<Diagnostic>;

    fn syntax(&self, file: FileId) -> SyntaxNode;
}

fn parsed(source: &dyn Source, file: FileId) -> (SyntaxNode, ArcSlice<Diagnostic>) {
    let source_text = source.file_source(file);
    let session = source.session();

    let (parsed, cache) = ddlog_syntax::parse(file, &source_text.to_string(), session.node_cache());
    session.give_node_cache(cache);

    let (root, mut diagnostics) = parsed.into_parts();
    diagnostics.shrink_to_fit();

    (root, ArcSlice::new(diagnostics))
}

fn syntax(source: &dyn Source, file: FileId) -> SyntaxNode {
    source.parsed(file).0
}

fn parse_diagnostics(source: &dyn Source, file: FileId) -> ArcSlice<Diagnostic> {
    source.parsed(file).1
}

#[salsa::query_group(ValidationDatabase)]
pub trait Validation: Source {
    fn validation_diagnostics(&self, file: FileId) -> ArcSlice<Diagnostic>;
}

fn validation_diagnostics(validation: &dyn Validation, file: FileId) -> ArcSlice<Diagnostic> {
    let mut ctx = RuleCtx::new(
        file,
        validation.file_source(file),
        validation.session().interner().clone(),
    );

    validation::run_validators(&validation.syntax(file), &mut ctx);
    ctx.diagnostics.shrink_to_fit();

    ArcSlice::new(ctx.diagnostics)
}

#[salsa::query_group(SymbolsDatabase)]
pub trait Symbols: Source {
    fn document_symbols(&self, file: FileId) -> ArcSlice<DocumentSymbol>;

    fn declarations(&self, file: FileId) -> ArcSlice<SyntaxNode>;

    fn document_function(&self, file: FileId, function: FunctionDef) -> DocumentSymbol;

    fn document_function_arg(
        &self,
        file: FileId,
        arg: FunctionArg,
    ) -> Option<ArcSlice<DocumentSymbol>>;
}

fn document_symbols(symbols: &dyn Symbols, file: FileId) -> ArcSlice<DocumentSymbol> {
    let declarations = symbols.declarations(file);

    for decl in declarations.iter() {
        println!(
            "declaration: {}",
            decl.debug(symbols.session().interner(), true),
        );
    }

    let document_symbols = declarations.iter().map(|node| {
        match_ast! {
            match node {
                FunctionDef(function) => {
                    println!("documenting function");
                    symbols.document_function(file, function.into_owned())
                },
                // TODO: Relation symbols
                RelationDef(relation) => default_document_symbol(),
                // TODO: Type definitions

                _ => unreachable!(),
            }
        }
    });

    ArcSlice::new(document_symbols)
}

fn declarations(symbols: &dyn Symbols, file: FileId) -> ArcSlice<SyntaxNode> {
    #[derive(Debug, Default)]
    struct DeclarationCollector(Vec<SyntaxNode>);

    impl AstVisitor for DeclarationCollector {
        fn check_node(&mut self, node: &SyntaxNode, _ctx: &mut RuleCtx) -> Option<()> {
            println!("checking node: {}", node.debug(_ctx.interner(), true));

            match_ast! {
                match node {
                    FunctionDef(_function) => {
                        println!("pushed function");
                        self.0.push(node.clone());
                    },
                    RelationDef(_relation) => self.0.push(node.clone()),
                    // TODO: Type definitions

                    _ => {},
                }
            }

            None
        }
    }

    let session = symbols.session();
    let root = symbols.syntax(file);
    let mut ctx = RuleCtx::new(file, symbols.file_source(file), session.interner().clone());

    let mut collector = DeclarationCollector::default();
    visitor::apply_visitor(&root, &mut collector, &mut ctx);
    collector.0.shrink_to_fit();

    ArcSlice::new(collector.0)
}

fn document_function(symbols: &dyn Symbols, file: FileId, function: FunctionDef) -> DocumentSymbol {
    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    let range = utils::ide_range(&source, function.signature_span(true));
    let (name, selection_range) = function
        .name()
        .as_ref()
        .and_then(|name| name.ident())
        .map(|name| {
            (
                name.text(interner),
                utils::ide_range(&source, name.text_range()),
            )
        })
        .unwrap_or(("???", range));
    let name = name.to_owned();

    let total_generics = function
        .generics()
        .map(|generics| generics.generics().count())
        .unwrap_or(0);
    let total_args = function.args().map(|args| args.args().count()).unwrap_or(0);
    let total_body_vars = function
        .body()
        // Not every statement will be a variable declaration
        .map(|body| body.statements().count() / 2)
        .unwrap_or(0);

    let mut children = Vec::with_capacity(total_generics + total_args + total_body_vars);

    if let Some(generics) = function.generics() {
        for generic in generics.generics() {
            let selection_range = generic.trimmed_range();

            if let Some(ty) = generic.type_token() {
                if let Type::GenericType(generic) = &*ty {
                    if generic
                        .generics()
                        .map(|generics| generics.generics().count() == 0)
                        .unwrap_or(true)
                    {
                        if let Some(name) = generic.ident() {
                            let selection_range = utils::ide_range(&source, selection_range);
                            let range = utils::ide_range(&source, name.text_range());

                            let type_param = DocumentSymbol {
                                name: name.text(interner).to_owned(),
                                detail: None,
                                kind: SymbolKind::TypeParameter,
                                tags: None,
                                range,
                                selection_range,
                                children: None,
                                ..default_document_symbol()
                            };

                            children.push(type_param);
                        }
                    }
                }
            }
        }
    }

    if let Some(args) = function.args() {
        for arg in args.args() {
            if let Some(arg) = symbols.document_function_arg(file, arg.into_owned()) {
                println!("got bindings: {:?}", arg);

                children.extend(arg.iter().cloned());
            }
        }
    }

    // TODO: Declarations within the function body

    // Look for a `#[deprecated]` attribute on the function
    let tags = function
        .attributes()
        .map(|attrs| attrs.any_are_deprecated(interner))
        .unwrap_or(false)
        .then(|| vec![SymbolTag::Deprecated]);

    let children = if children.is_empty() {
        None
    } else {
        children.shrink_to_fit();
        Some(children)
    };

    DocumentSymbol {
        name,
        // TODO: Get the function's documentation if there's any
        detail: None,
        kind: SymbolKind::Function,
        tags,
        range,
        selection_range,
        children,
        ..default_document_symbol()
    }
}

fn document_function_arg(
    symbols: &dyn Symbols,
    file: FileId,
    arg: FunctionArg,
) -> Option<ArcSlice<DocumentSymbol>> {
    println!("function arg");

    // Each function argument is a pattern, which means that it can bind multiple variables.
    // Ergo, we use this function to recursively walk through the bound patterns, adding to
    // the list of symbols as we go
    fn process_pattern(
        binding: &Pattern,
        bindings: &mut Vec<DocumentSymbol>,
        source: &Rope,
        interner: &Interner,
        is_deprecated: bool,
    ) {
        println!("process pattern");

        match binding {
            Pattern::TuplePattern(tuple) => {
                for element in tuple.elements() {
                    if let Some(pattern) = element.pattern() {
                        process_pattern(&*pattern, bindings, source, interner, is_deprecated);
                    }
                }
            }

            Pattern::VarRef(var) => {
                if let Some(ident) = var.ident() {
                    let tags = is_deprecated.then(|| vec![SymbolTag::Deprecated]);
                    let range = utils::ide_range(source, ident.text_range());

                    bindings.push(DocumentSymbol {
                        name: ident.text(interner).to_owned(),
                        detail: None,
                        kind: SymbolKind::Variable,
                        tags,
                        range,
                        selection_range: range,
                        children: None,
                        ..default_document_symbol()
                    });
                }
            }
        }
    }

    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    let is_deprecated = arg
        .attributes()
        .map(|attrs| attrs.any_are_deprecated(interner))
        .unwrap_or(false);

    let mut bindings = Vec::with_capacity(arg.binding().is_some() as usize);
    if let Some(binding) = arg.binding() {
        process_pattern(&*binding, &mut bindings, &source, interner, is_deprecated);
    }

    if bindings.is_empty() {
        None
    } else {
        bindings.shrink_to_fit();
        Some(ArcSlice::new(bindings))
    }
}

const fn default_document_symbol() -> DocumentSymbol {
    // We do this weird manual construction so that we can make the function const
    let position = Position {
        line: 0,
        character: 0,
    };
    let range = Range {
        start: position,
        end: position,
    };

    // Note: We allow deprecation here because the `deprecated` field is,
    //       ironically, deprecated. However, since `DocumentSymbol` doesn't
    //       implement `Default` we're forced to construct the field *somehow*,
    //       which forces us to trigger the deprecation error
    #[allow(deprecated)]
    DocumentSymbol {
        name: String::new(),
        detail: None,
        kind: SymbolKind::Unknown,
        tags: None,
        range,
        selection_range: range,
        children: None,
        deprecated: None,
    }
}

#[salsa::database(SessionDatabase, SourceDatabase, ValidationDatabase, SymbolsDatabase)]
#[derive(Default)]
pub struct DDlogDatabase {
    storage: Storage<Self>,
}

impl Debug for DDlogDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DDlogDatabase").finish()
    }
}

impl Database for DDlogDatabase {}

impl ParallelDatabase for DDlogDatabase {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(Self {
            storage: self.storage.snapshot(),
        })
    }
}
