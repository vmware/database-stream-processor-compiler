use crate::{database::DocumentSymbols, providers::utils};
use ddlog_diagnostics::{FileId, Interner, Rope};
use ddlog_syntax::{
    ast::{
        nodes::{
            BracketedStructField, EnumDef, EnumVariant, FunctionArg, FunctionDef, Pattern, Stmt,
            StructDef, Type, VariantStructField,
        },
        AstNode, AstToken,
    },
    match_ast, SyntaxNode, SyntaxNodeExt,
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
    let document_symbols = declarations.iter().filter_map(|node| {
        tracing::debug!(
            file = uri,
            "visiting declaration: {}",
            node.debug(symbols.session().interner(), false),
        );

        Some(match_ast! {
            match node {
                EnumDef(def) => symbols.document_enum(file, def.into_owned()),
                StructDef(def) => symbols.document_struct(file, def.into_owned()),
                FunctionDef(function) => symbols.document_function(file, function.into_owned()),
                // TODO: Constant definitions
                // TODO: Type definitions

                _ => {
                    tracing::warn!("didn't document declaration: {}", node.debug(interner, false));
                    return None;
                },
            }
        })
    });

    let symbols = ArcSlice::from_dynamic(document_symbols);
    tracing::trace!(file = uri, "produced {} document symbols", symbols.len());

    symbols
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

            if let Some(ty) = generic.ty() {
                // TODO: There's other types to handle
                if let Type::GenericType(generic) = &*ty {
                    if generic
                        .generics()
                        .map(|generics| generics.generics().count() == 0)
                        .unwrap_or(true)
                    {
                        if let Some(path) = generic.path() {
                            let selection_range = utils::ide_range(&source, selection_range);
                            let range = utils::ide_range(&source, path.text_range());

                            let type_param = DocumentSymbol {
                                name: path.text(interner).to_string(),
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
        .any(|attr| attr.is_deprecated(interner))
        .then(|| vec![SymbolTag::Deprecated]);

    let children = if children.is_empty() {
        None
    } else {
        Some(children)
    };

    let mut detail = String::from("fn ");
    if let Some(name) = function.name() {
        detail.push_str(name.syntax().resolve_text(interner));
    }
    if let Some(generics) = function.generics() {
        stripped_node_text(generics.syntax(), interner, &mut detail);
    }
    if let Some(args) = function.args() {
        stripped_node_text(args.syntax(), interner, &mut detail);
    }
    if let Some(ret) = function.ret() {
        detail.push(' ');
        stripped_node_text(ret.syntax(), interner, &mut detail);
    }

    DocumentSymbol {
        name,
        detail: Some(detail),
        kind: SymbolKind::Function,
        tags,
        range,
        selection_range,
        children,
        ..default_document_symbol()
    }
}

// FIXME: This is dirty, we really want to use an actual formatter since this produces
//        varying outputs based off of the physical input's syntax
fn stripped_node_text(node: &SyntaxNode, interner: &Interner, output: &mut String) {
    let (mut can_insert_whitespace, mut last_was_whitespace) = (false, false);
    node.trimmed_text(interner).for_each_chunk(|chunk| {
        for line in chunk.lines() {
            let line = line.trim();
            if line.is_empty() {
                if can_insert_whitespace && !last_was_whitespace {
                    output.push(' ');
                    can_insert_whitespace = false;
                }

                last_was_whitespace = true;
            } else {
                output.push_str(line);
                can_insert_whitespace = true;
                last_was_whitespace = false;
            }
        }
    });
}

pub(crate) fn document_function_arg(
    symbols: &dyn DocumentSymbols,
    file: FileId,
    arg: FunctionArg,
) -> Option<ArcSlice<DocumentSymbol>> {
    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    let is_deprecated = arg.attributes().any(|attr| attr.is_deprecated(interner));

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

pub(crate) fn document_struct(
    symbols: &dyn DocumentSymbols,
    file: FileId,
    strct: StructDef,
) -> DocumentSymbol {
    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    tracing::debug!(
        file = file.to_str(interner),
        "collecting symbols for struct declaration",
    );

    let range = utils::ide_range(&source, strct.trimmed_range());
    let (name, selection_range) = strct
        .name()
        .as_ref()
        .map(|name| {
            (
                name.text(interner),
                utils::ide_range(&source, name.text_range()),
            )
        })
        .unwrap_or(("???", range));
    let name = name.to_owned();

    // Look for a `#[deprecated]` attribute on the struct
    let tags = strct
        .attributes()
        .any(|attr| attr.is_deprecated(interner))
        .then(|| vec![SymbolTag::Deprecated]);

    let total_fields = strct
        .fields()
        .and_then(|fields| {
            fields
                .as_bracketed_struct_fields()
                .map(|fields| fields.fields().count())
        })
        .unwrap_or(0);
    let mut children = Vec::with_capacity(total_fields);

    if let Some(fields) = strct.fields() {
        if let Some(fields) = fields.as_bracketed_struct_fields() {
            for field in fields.fields() {
                if let Some(field) = symbols.document_struct_field(file, field.into_owned()) {
                    children.push(field);
                }
            }
        }
    }

    let children = if children.is_empty() {
        None
    } else {
        children.shrink_to_fit();
        Some(children)
    };

    DocumentSymbol {
        name,
        // TODO: Get the struct's documentation if there's any
        detail: None,
        kind: SymbolKind::Struct,
        tags,
        range,
        selection_range,
        children,
        ..default_document_symbol()
    }
}

pub(crate) fn document_struct_field(
    symbols: &dyn DocumentSymbols,
    file: FileId,
    field: BracketedStructField,
) -> Option<DocumentSymbol> {
    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    field.name().map(|name| {
        let tags = field
            .attributes()
            .any(|attr| attr.is_deprecated(interner))
            .then(|| vec![SymbolTag::Deprecated]);
        let range = utils::ide_range(&source, name.text_range());

        DocumentSymbol {
            name: name.text(interner).to_owned(),
            detail: None,
            kind: SymbolKind::Variable,
            tags,
            range,
            selection_range: range,
            children: None,
            ..default_document_symbol()
        }
    })
}

pub(crate) fn document_enum(
    symbols: &dyn DocumentSymbols,
    file: FileId,
    enumeration: EnumDef,
) -> DocumentSymbol {
    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    tracing::debug!(
        file = file.to_str(interner),
        "collecting symbols for enum declaration",
    );

    let range = utils::ide_range(&source, enumeration.trimmed_range());
    let (name, selection_range) = enumeration
        .name()
        .as_ref()
        .map(|name| {
            (
                name.text(interner),
                utils::ide_range(&source, name.text_range()),
            )
        })
        .unwrap_or(("???", range));
    let name = name.to_owned();

    // Look for a `#[deprecated]` attribute on the enum
    let tags = enumeration
        .is_deprecated(interner)
        .then(|| vec![SymbolTag::Deprecated]);

    let total_variants = enumeration
        .variants()
        .map(|variants| variants.variants().count())
        .unwrap_or(0);
    let mut children = Vec::with_capacity(total_variants);

    if let Some(variants) = enumeration.variants() {
        for variant in variants.variants() {
            if let Some(field) = symbols.document_enum_variant(file, variant.into_owned()) {
                children.push(field);
            }
        }
    }

    let children = if children.is_empty() {
        None
    } else {
        children.shrink_to_fit();
        Some(children)
    };

    DocumentSymbol {
        name,
        // TODO: Get the enum's documentation if there's any
        detail: None,
        kind: SymbolKind::Enum,
        tags,
        range,
        selection_range,
        children,
        ..default_document_symbol()
    }
}

pub(crate) fn document_enum_variant(
    symbols: &dyn DocumentSymbols,
    file: FileId,
    variant: EnumVariant,
) -> Option<DocumentSymbol> {
    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    variant.variant().map(|name| {
        let tags = variant
            .is_deprecated(interner)
            .then(|| vec![SymbolTag::Deprecated]);
        let range = utils::ide_range(&source, name.text_range());

        // The number of fields that the variant has, if any
        let variant_fields = variant
            .enum_variant_body()
            .and_then(|body| body.as_variant_struct().map(|strct| strct.fields().count()))
            .unwrap_or(0);

        let mut children = Vec::with_capacity(variant_fields);
        if let Some(symbols) = variant
            .enum_variant_body()
            .as_ref()
            .and_then(|body| body.as_variant_struct())
            .map(|variant| {
                variant
                    .fields()
                    .filter_map(|field| symbols.document_variant_field(file, field.into_owned()))
            })
        {
            children.extend(symbols);
        }

        let children = if children.is_empty() {
            None
        } else {
            children.shrink_to_fit();
            Some(children)
        };

        DocumentSymbol {
            name: name.text(interner).to_owned(),
            detail: None,
            kind: SymbolKind::Variable,
            tags,
            range,
            selection_range: range,
            children,
            ..default_document_symbol()
        }
    })
}

pub(crate) fn document_variant_field(
    symbols: &dyn DocumentSymbols,
    file: FileId,
    field: VariantStructField,
) -> Option<DocumentSymbol> {
    let session = symbols.session();
    let interner = session.interner();
    let source = symbols.file_source(file);

    field.name().map(|name| {
        let tags = field
            .attributes()
            .any(|attr| attr.is_deprecated(interner))
            .then(|| vec![SymbolTag::Deprecated]);
        let range = utils::ide_range(&source, name.text_range());

        DocumentSymbol {
            name: name.text(interner).to_owned(),
            detail: None,
            kind: SymbolKind::Variable,
            tags,
            range,
            selection_range: range,
            children: None,
            ..default_document_symbol()
        }
    })
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
        Pattern::StructPattern(strct) => {
            for field in strct.fields() {
                if let Some(pattern) = field.alias() {
                    process_pattern(&*pattern, bindings, source, interner, is_deprecated);
                }
            }
        }

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
