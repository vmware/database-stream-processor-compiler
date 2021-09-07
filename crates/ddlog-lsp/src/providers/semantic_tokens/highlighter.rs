use crate::providers::{
    semantic_tokens::{builder::SemanticTokensBuilder, tokens::ModifierSet},
    utils,
};
use cstree::NodeOrToken;
use ddlog_diagnostics::Interner;
use ddlog_syntax::{
    ast::{
        nodes::{FuncName, RelName, Root},
        tokens::Number,
        AstNode,
    },
    SyntaxKind::{BIN_OP, COMMENT, IDENT, UNARY_OP},
    SyntaxNodeExt, SyntaxTokenExt, T,
};
use lspower::lsp::{SemanticTokenModifier, SemanticTokenType, SemanticTokens};
use ropey::Rope;

pub(crate) struct SemanticHighlighter {}

impl SemanticHighlighter {
    // FIXME: Completely rewrite this since it's super dirty, we want to use the
    //        typed traversal methods
    pub(crate) fn highlight(source: &Rope, root: &Root, interner: &Interner) -> SemanticTokens {
        // FIXME: Provide ids so we can delta update highlighting
        // TODO: Persistent buffer
        let mut tokens = Vec::with_capacity(1024);

        // FIXME: Use a visitor for this
        root.syntax().descendants_with_tokens_with(|element| {
            match element {
                NodeOrToken::Node(node) => {
                    if matches!(node.kind(), UNARY_OP | BIN_OP) {
                        tracing::trace!("highlighting operator: {}", node.debug(interner, true),);

                        tokens.push((
                            node.text_range(),
                            SemanticTokenType::OPERATOR,
                            ModifierSet::empty(),
                        ));

                        false
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

                        false
                    } else if node.is::<FuncName>() {
                        tracing::trace!(
                            "highlighting function name: {}",
                            node.debug(interner, true),
                        );

                        let mut modifiers = ModifierSet::empty();
                        modifiers |= SemanticTokenModifier::DECLARATION;
                        modifiers |= SemanticTokenModifier::DEFINITION;

                        tokens.push((node.text_range(), SemanticTokenType::FUNCTION, modifiers));

                        false
                    } else {
                        tracing::trace!(
                            "pushing node to work stack: {}",
                            node.debug(interner, true),
                        );

                        true
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
                        | T![var]
                        | T![multiset]
                        | T![stream] => {
                            tracing::trace!("highlighting keyword: {}", token.debug(interner));

                            tokens.push((
                                token.text_range(),
                                SemanticTokenType::KEYWORD,
                                ModifierSet::empty(),
                            ));
                        }

                        _ => {
                            if token.is::<Number>() {
                                tracing::trace!("highlighting number: {}", token.debug(interner));

                                tokens.push((
                                    token.text_range(),
                                    SemanticTokenType::NUMBER,
                                    ModifierSet::empty(),
                                ));
                            }
                        }
                    }

                    false
                }
            }
        });

        let mut builder = SemanticTokensBuilder::new(None);

        // Token encoding is relative, so we have to produce tokens in lexical order
        tokens.sort_by(|&(lhs, _, _), &(rhs, _, _)| lhs.ordering(rhs));
        for (range, token, modifiers) in tokens {
            let range = utils::ide_range(source, range);
            builder.push(range, token, modifiers);
        }

        builder.build()
    }
}
