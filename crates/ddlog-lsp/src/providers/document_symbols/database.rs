use crate::{database::DocumentSymbols, providers::utils};
use ddlog_diagnostics::{FileId, Interner, Rope};
use ddlog_syntax::{
    ast::{
        nodes::{FunctionArg, FunctionDef, Pattern, RelCol, RelationDef, Stmt, Type},
        AstNode, AstToken,
    },
    match_ast, visitor, AstVisitor, RuleCtx, SyntaxNode,
};
use ddlog_utils::ArcSlice;
use lspower::lsp::{DocumentSymbol, Position, Range, SymbolKind, SymbolTag};

pub(crate) fn document_symbols(
    symbols: &dyn DocumentSymbols,
    file: FileId,
) -> ArcSlice<DocumentSymbol> {
    let session = symbols.session();
    let interner = session.interner();
    let uri = file.to_str(interner);

    let declarations = symbols.declarations(file);
    let document_symbols = declarations.iter().map(|node| {
        tracing::debug!(
            file = uri,
            "visiting declaration: {}",
            node.debug(symbols.session().interner(), true),
        );

        match_ast! {
            match node {
                FunctionDef(function) => {
                    tracing::debug!("documenting function");
                    symbols.document_function(file, function.into_owned())
                },
                RelationDef(relation) => {
                    tracing::debug!("documenting relation");
                    symbols.document_relation(file, relation.into_owned())
                },
                // TODO: Type definitions

                _ => unreachable!(),
            }
        }
    });

    let symbols = ArcSlice::new(document_symbols);
    tracing::trace!(file = uri, "produced {} document symbols", symbols.len());

    symbols
}

#[derive(Debug, Default)]
struct DeclarationCollector(Vec<SyntaxNode>);

impl AstVisitor for DeclarationCollector {
    fn check_node(&mut self, node: &SyntaxNode, _ctx: &mut RuleCtx) -> Option<()> {
        match_ast! {
            match node {
                FunctionDef(_function) => {
                    tracing::trace!("collected function declaration");
                    self.0.push(node.clone());
                },
                RelationDef(_relation) => {
                    tracing::trace!("collected relation declaration");
                    self.0.push(node.clone());
                },
                // TODO: Type definitions

                _ => {},
            }
        }

        None
    }
}

pub(crate) fn declarations(symbols: &dyn DocumentSymbols, file: FileId) -> ArcSlice<SyntaxNode> {
    let session = symbols.session();
    let interner = session.interner();
    let uri = file.to_str(interner);

    tracing::debug!(file = uri, "collecting declarations for a file");

    let root = symbols.syntax(file);
    let mut ctx = RuleCtx::new(file, symbols.file_source(file), interner.clone());

    let mut collector = DeclarationCollector::default();
    visitor::apply_visitor(&root, &mut collector, &mut ctx);

    let declarations = collector.0;
    tracing::debug!(
        file = uri,
        "collected {} total declarations for",
        declarations.len(),
    );

    ArcSlice::new(declarations)
}

pub(crate) fn document_function(
    symbols: &dyn DocumentSymbols,
    file: FileId,
    function: FunctionDef,
) -> DocumentSymbol {
    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    tracing::debug!(
        file = file.to_str(interner),
        "collecting symbols for function declaration",
    );

    let range = utils::ide_range(&source, function.trimmed_range());
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
                children.extend(arg.iter().cloned());
            }
        }
    }

    if let Some(block) = function.body() {
        for stmt in block.statements() {
            if let Stmt::VarDecl(decl) = &*stmt {
                let mut bindings = Vec::with_capacity(decl.binding().is_some() as usize);
                if let Some(binding) = decl.binding() {
                    process_pattern(&*binding, &mut bindings, &source, interner, false);
                }

                children.extend(bindings);
            }

            // TODO: Nested variable declarations :(
        }
    }

    // Look for a `#[deprecated]` attribute on the function
    let tags = function
        .attributes()
        .map(|attrs| attrs.any_are_deprecated(interner))
        .unwrap_or(false)
        .then(|| vec![SymbolTag::Deprecated]);

    let children = if children.is_empty() {
        None
    } else {
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

pub(crate) fn document_function_arg(
    symbols: &dyn DocumentSymbols,
    file: FileId,
    arg: FunctionArg,
) -> Option<ArcSlice<DocumentSymbol>> {
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
        Some(ArcSlice::new(bindings))
    }
}

pub(crate) fn document_relation(
    symbols: &dyn DocumentSymbols,
    file: FileId,
    relation: RelationDef,
) -> DocumentSymbol {
    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    tracing::debug!(
        file = file.to_str(interner),
        "collecting symbols for relation declaration",
    );

    let range = utils::ide_range(&source, relation.trimmed_range());
    let (name, selection_range) = relation
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

    // Look for a `#[deprecated]` attribute on the relation
    let tags = relation
        .attributes()
        .map(|attrs| attrs.any_are_deprecated(interner))
        .unwrap_or(false)
        .then(|| vec![SymbolTag::Deprecated]);

    let total_columns = relation
        .columns()
        .map(|args| args.columns().count())
        .unwrap_or(0);
    let mut children = Vec::with_capacity(total_columns);

    if let Some(columns) = relation.columns() {
        for column in columns.columns() {
            if let Some(column) = symbols.document_relation_column(file, column.into_owned()) {
                children.extend(column.iter().cloned());
            }
        }
    }

    let children = if children.is_empty() {
        None
    } else {
        Some(children)
    };

    DocumentSymbol {
        name,
        // TODO: Get the relation's documentation if there's any
        detail: None,
        // I can't figure out if there's custom symbol kinds, so
        // saying that relations are classes is my best guess for now
        kind: SymbolKind::Class,
        tags,
        range,
        selection_range,
        children,
        ..default_document_symbol()
    }
}

pub(crate) fn document_relation_column(
    symbols: &dyn DocumentSymbols,
    file: FileId,
    column: RelCol,
) -> Option<ArcSlice<DocumentSymbol>> {
    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    let is_deprecated = column
        .attributes()
        .map(|attrs| attrs.any_are_deprecated(interner))
        .unwrap_or(false);

    let mut bindings = Vec::with_capacity(column.binding().is_some() as usize);
    if let Some(binding) = column.binding() {
        process_pattern(&*binding, &mut bindings, &source, interner, is_deprecated);
    }

    if bindings.is_empty() {
        None
    } else {
        Some(ArcSlice::new(bindings))
    }
}

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
