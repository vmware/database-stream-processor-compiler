use crate::providers::{
    semantic_tokens::{builder::SemanticTokensBuilder, tokens::ModifierSet},
    utils,
};
use cstree::TextRange;
use ddlog_syntax::{
    ast::AstNode,
    match_ast, AstVisitor, RuleCtx,
    SyntaxKind::{COMMENT, IDENT, NUMBER},
    SyntaxNode, SyntaxToken, SyntaxTokenExt, T,
};
use lspower::lsp::{SemanticTokenModifier, SemanticTokenType, SemanticTokens};
use ropey::Rope;

type TokenVec = Vec<(TextRange, SemanticTokenType, ModifierSet)>;

pub(crate) struct SemanticHighlighter {
    tokens: TokenVec,
}

impl SemanticHighlighter {
    pub(crate) fn new() -> Self {
        Self {
            tokens: Vec::with_capacity(1024),
        }
    }

    fn push(&mut self, range: TextRange, kind: SemanticTokenType, modifiers: ModifierSet) {
        self.tokens.push((range, kind, modifiers));
    }

    pub(crate) fn finish(mut self, source: &Rope) -> SemanticTokens {
        let mut builder = SemanticTokensBuilder::with_capacity(None, self.tokens.len());

        // Token encoding is relative, so we have to produce tokens in lexical order
        self.tokens
            .sort_by(|&(lhs, _, _), &(rhs, _, _)| lhs.ordering(rhs));

        for (range, token, modifiers) in self.tokens {
            let range = utils::ide_range(source, range);
            builder.push(range, token, modifiers);
        }

        builder.build()
    }

    fn operator(&mut self, range: TextRange) {
        self.push(range, SemanticTokenType::OPERATOR, ModifierSet::empty());
    }
}

impl AstVisitor for SemanticHighlighter {
    fn check_node(&mut self, node: &SyntaxNode, _ctx: &mut RuleCtx) -> Option<()> {
        match_ast! {
            match node {
                UnaryOp(op) => self.operator(op.trimmed_range()),
                BinOp(op) => self.operator(op.trimmed_range()),

                RelName(name) => {
                    let mut modifiers = ModifierSet::empty();
                    modifiers |= SemanticTokenModifier::DECLARATION;
                    modifiers |= SemanticTokenModifier::DEFINITION;

                    self.push(
                        name.trimmed_range(),
                        // TODO: Make a custom type for relations
                        SemanticTokenType::FUNCTION,
                        modifiers,
                    );
                },

                FunctionName(name) => {
                    let mut modifiers = ModifierSet::empty();
                    modifiers |= SemanticTokenModifier::DECLARATION;
                    modifiers |= SemanticTokenModifier::DEFINITION;

                    self.push(
                        name.trimmed_range(),
                        SemanticTokenType::FUNCTION,
                        modifiers,
                    );
                },

                GenericArg(generic) => {
                    self.push(
                        generic.trimmed_range(),
                        SemanticTokenType::TYPE_PARAMETER,
                        ModifierSet::empty(),
                    );
                },

                _ => return None,
            }
        }

        Some(())
    }

    fn check_token(&mut self, token: &SyntaxToken, ctx: &mut RuleCtx) -> Option<()> {
        let range = token.text_range();

        match token.kind() {
            T![function]
            | T![relation]
            | T![extern]
            | T![input]
            | T![output]
            | T![true]
            | T![false]
            | T![var]
            | T![multiset]
            | T![stream]
            | T![typedef] => {
                let mut modifiers = ModifierSet::empty();
                if matches!(token.kind(), T![extern] | T![input] | T![output]) {
                    modifiers |= SemanticTokenModifier::MODIFICATION;
                } else if matches!(token.kind(), T![true] | T![false]) {
                    modifiers |= SemanticTokenModifier::DEFAULT_LIBRARY;
                }

                self.push(range, SemanticTokenType::KEYWORD, modifiers);
            }

            IDENT => self.push(
                token.text_range(),
                SemanticTokenType::VARIABLE,
                ModifierSet::empty(),
            ),

            COMMENT => {
                let mut modifiers = ModifierSet::empty();
                // Doc comments get the documentation modifier
                let comment_text = token.text(ctx.interner()).trim();
                if comment_text.starts_with("///") || comment_text.starts_with("/**") {
                    modifiers |= SemanticTokenModifier::DOCUMENTATION;
                }

                self.push(token.text_range(), SemanticTokenType::COMMENT, modifiers);
            }

            // TODO: Floats
            NUMBER => self.push(
                token.text_range(),
                SemanticTokenType::NUMBER,
                ModifierSet::empty(),
            ),

            _ => {}
        }

        None
    }
}