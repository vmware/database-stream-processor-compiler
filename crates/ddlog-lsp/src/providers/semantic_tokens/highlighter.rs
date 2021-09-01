use crate::providers::semantic_tokens::{builder::SemanticTokensBuilder, tokens::ModifierSet};
use cstree::NodeOrToken;
use ddlog_syntax::{SyntaxKind, SyntaxNode};
use lspower::lsp::{Position, Range, SemanticTokenType, SemanticTokens};
use ropey::Rope;

pub(crate) struct SemanticHighlighter {}

impl SemanticHighlighter {
    // FIXME: Completely rewrite this since it's super dirty, we want to use the
    //        typed traversal methods
    pub(crate) fn highlight(source: &Rope, root: &SyntaxNode) -> SemanticTokens {
        // FIXME: Provide ids so we can delta update highlighting
        let mut builder = SemanticTokensBuilder::new(None);

        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            for element in node.descendants_with_tokens() {
                match element {
                    NodeOrToken::Node(node) => stack.push(node),
                    NodeOrToken::Token(token) => match token.kind() {
                        SyntaxKind::IDENT => {
                            let line =
                                source.byte_to_line(token.text_range().start().into()) as u32;
                            let start =
                                source.byte_to_char(token.text_range().start().into()) as u32;
                            let end = source.byte_to_char(token.text_range().end().into()) as u32;

                            builder.push(
                                Range::new(Position::new(line, start), Position::new(line, end)),
                                SemanticTokenType::TYPE,
                                ModifierSet::empty(),
                            );
                        }

                        _ => {}
                    },
                }
            }
        }

        builder.build()
    }
}
