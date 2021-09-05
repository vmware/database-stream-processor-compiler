use crate::providers::semantic_tokens::{builder::SemanticTokensBuilder, tokens::ModifierSet};
use cstree::{NodeOrToken, TextRange};
use ddlog_diagnostics::Interner;
use ddlog_syntax::{
    ast::{
        nodes::{BinOp, FuncName, RelName, Root, UnaryOp},
        tokens::Number,
        AstNode,
    },
    SyntaxKind::{BIN_OP, COMMENT, IDENT, UNARY_OP},
    SyntaxNodeExt, SyntaxTokenExt, T,
};
use lspower::lsp::{Position, Range, SemanticTokenModifier, SemanticTokenType, SemanticTokens};
use ropey::Rope;

pub(crate) struct SemanticHighlighter {}

impl SemanticHighlighter {
    // FIXME: Completely rewrite this since it's super dirty, we want to use the
    //        typed traversal methods
    pub(crate) fn highlight(source: &Rope, root: &Root, interner: &Interner) -> SemanticTokens {
        // FIXME: Provide ids so we can delta update highlighting
        // TODO: Persistent buffer
        let mut tokens = Vec::with_capacity(1024);

        let mut stack = vec![root.syntax()];
        while let Some(node) = stack.pop() {
            for element in node.descendants_with_tokens().skip(1) {
                match element {
                    NodeOrToken::Node(node) => {
                        if matches!(node.kind(), UNARY_OP | BIN_OP) {
                            tracing::trace!(
                                "highlighting operator: {}",
                                node.debug(interner, true),
                            );

                            tokens.push((
                                node.text_range(),
                                SemanticTokenType::OPERATOR,
                                ModifierSet::empty(),
                            ));
                        } else if node.is::<RelName>() {
                            tracing::trace!(
                                "highlighting relation name: {}",
                                node.debug(interner, true),
                            );

                            let mut modifiers = ModifierSet::empty();
                            modifiers |= SemanticTokenModifier::DECLARATION;
                            modifiers |= SemanticTokenModifier::DEFINITION;

                            tokens.push((
                                node.text_range(),
                                // TODO: Make a custom type for relations
                                SemanticTokenType::FUNCTION,
                                modifiers,
                            ));
                        } else if node.is::<FuncName>() {
                            tracing::trace!(
                                "highlighting function name: {}",
                                node.debug(interner, true),
                            );

                            let mut modifiers = ModifierSet::empty();
                            modifiers |= SemanticTokenModifier::DECLARATION;
                            modifiers |= SemanticTokenModifier::DEFINITION;

                            tokens.push((
                                node.text_range(),
                                SemanticTokenType::FUNCTION,
                                modifiers,
                            ));
                        } else {
                            tracing::trace!(
                                "pushing node to work stack: {}",
                                node.debug(interner, true),
                            );

                            stack.push(node);
                        }
                    }

                    NodeOrToken::Token(token) => {
                        tracing::trace!("visiting token: {}", token.debug(interner));

                        match token.kind() {
                            IDENT => {
                                tracing::trace!("highlighting ident: {}", token.debug(interner));

                                tokens.push((
                                    token.text_range(),
                                    SemanticTokenType::TYPE,
                                    ModifierSet::empty(),
                                ));
                            }

                            COMMENT => {
                                tracing::trace!("highlighting comment: {}", token.debug(interner));

                                tokens.push((
                                    token.text_range(),
                                    SemanticTokenType::COMMENT,
                                    ModifierSet::empty(),
                                ));
                            }

                            T![function]
                            | T![relation]
                            | T![extern]
                            | T![input]
                            | T![output]
                            | T![true]
                            | T![false]
                            | T![var] => {
                                tracing::trace!("highlighting keyword: {}", token.debug(interner));

                                tokens.push((
                                    token.text_range(),
                                    SemanticTokenType::KEYWORD,
                                    ModifierSet::empty(),
                                ));
                            }

                            _ => {
                                if token.is::<Number>() {
                                    tracing::trace!(
                                        "highlighting number: {}",
                                        token.debug(interner)
                                    );

                                    tokens.push((
                                        token.text_range(),
                                        SemanticTokenType::NUMBER,
                                        ModifierSet::empty(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut builder = SemanticTokensBuilder::new(None);

        // Token encoding is relative, so we have to produce tokens in lexical order
        tokens.sort_by(|&(lhs, _, _), &(rhs, _, _)| lhs.ordering(rhs));
        for (range, token, modifiers) in tokens {
            let range = ide_range(source, range);
            builder.push(range, token, modifiers);
        }

        builder.build()
    }
}

fn ide_range(source: &Rope, text_range: TextRange) -> Range {
    let (start, end) = (text_range.start().into(), text_range.end().into());
    // Since ranges are relative to a line's start, semantic tokens can't
    // cross line boundaries
    debug_assert_eq!(source.byte_to_line(start), source.byte_to_line(end));

    let line = source.byte_to_line(start);
    let line_byte = source.line_to_byte(line);

    // Ranges are relative to the start of the line
    let start = source.byte_to_char(start) - line_byte;
    let end = source.byte_to_char(end) - line_byte;

    Range::new(
        Position::new(line as u32, start as u32),
        Position::new(line as u32, end as u32),
    )
}
