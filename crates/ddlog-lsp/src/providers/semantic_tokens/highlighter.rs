use crate::providers::semantic_tokens::{builder::SemanticTokensBuilder, tokens::ModifierSet};
use cstree::NodeOrToken;
use ddlog_syntax::{
    ast::{nodes::Root, tokens::Ident, AstNode, AstToken},
    SyntaxTokenExt,
};
use lspower::lsp::{Position, Range, SemanticTokenType, SemanticTokens};
use ropey::Rope;

pub(crate) struct SemanticHighlighter {}

impl SemanticHighlighter {
    // FIXME: Completely rewrite this since it's super dirty, we want to use the
    //        typed traversal methods
    pub(crate) fn highlight(source: &Rope, root: &Root) -> SemanticTokens {
        // FIXME: Provide ids so we can delta update highlighting
        let mut builder = SemanticTokensBuilder::new(None);

        let mut stack = vec![root.syntax()];
        while let Some(node) = stack.pop() {
            for element in node.descendants_with_tokens() {
                match element {
                    NodeOrToken::Node(node) => stack.push(node),
                    NodeOrToken::Token(token) => {
                        println!("{:?}", token);

                        if let Some(ident) = token.try_to::<Ident>() {
                            let line =
                                source.byte_to_line(ident.text_range().start().into()) as u32;
                            let start =
                                source.byte_to_char(ident.text_range().start().into()) as u32;
                            let end = source.byte_to_char(ident.text_range().end().into()) as u32;

                            builder.push(
                                Range::new(Position::new(line, start), Position::new(line, end)),
                                SemanticTokenType::TYPE,
                                ModifierSet::empty(),
                            );
                        }
                    }
                }
            }
        }

        builder.build()
    }
}
