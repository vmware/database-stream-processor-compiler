use crate::providers::{
    semantic_tokens::{builder::SemanticTokensBuilder, tokens::ModifierSet},
    utils,
};
use cstree::TextRange;
use ddlog_syntax::{
    ast::{AstNode, AstToken},
    match_ast, AstVisitor, RuleCtx,
    SyntaxKind::COMMENT,
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
                FunctionDef(func) => {
                    if let Some(name) = func.name() {
                        let mut modifiers = ModifierSet::empty();
                        modifiers |= SemanticTokenModifier::DECLARATION;
                        modifiers |= SemanticTokenModifier::DEFINITION;

                        self.push(
                            name.text_range(),
                            SemanticTokenType::FUNCTION,
                            modifiers,
                        );
                    }

                    if let Some(ret) = func.ret() {
                        if let Some(arrow) = ret.right_arrow() {
                            self.push(
                                arrow.text_range(),
                                SemanticTokenType::OPERATOR,
                                ModifierSet::empty(),
                            );
                        }
                    }

                    None
                },

                StructDef(strct) => {
                    if let Some(name) = strct.name() {
                        let mut modifiers = ModifierSet::empty();
                        modifiers |= SemanticTokenModifier::DECLARATION;
                        modifiers |= SemanticTokenModifier::DEFINITION;

                        self.push(
                            name.text_range(),
                            SemanticTokenType::FUNCTION,
                            modifiers,
                        );
                    }

                    None
                },

                EnumDef(enum_def) => {
                    if let Some(name) = enum_def.name() {
                        let mut modifiers = ModifierSet::empty();
                        modifiers |= SemanticTokenModifier::DECLARATION;
                        modifiers |= SemanticTokenModifier::DEFINITION;

                        self.push(
                            name.text_range(),
                            SemanticTokenType::ENUM,
                            modifiers,
                        );
                    }

                    None
                },

                GenericArg(generic) => {
                    self.push(
                        generic.trimmed_range(),
                        SemanticTokenType::TYPE_PARAMETER,
                        ModifierSet::empty(),
                    );

                    Some(())
                },

                VarRef(var) => {
                    self.push(
                        var.trimmed_range(),
                        SemanticTokenType::VARIABLE,
                        ModifierSet::empty(),
                    );

                    Some(())
                },

                Number(number) => {
                    self.push(
                        number.trimmed_range(),
                        SemanticTokenType::NUMBER,
                        ModifierSet::empty(),
                    );

                    Some(())
                },

                String(string) => {
                    self.push(
                        string.trimmed_range(),
                        SemanticTokenType::STRING,
                        ModifierSet::empty(),
                    );

                    Some(())
                },

                Char(char) => {
                    self.push(
                        char.trimmed_range(),
                        SemanticTokenType::STRING,
                        ModifierSet::empty(),
                    );

                    Some(())
                },

                UnaryOp(op) => {
                    self.operator(op.trimmed_range());
                    None
                },

                BinOp(op) => {
                    self.operator(op.trimmed_range());
                    None
                },

                _ => None,
            }
        }
    }

    fn check_token(&mut self, token: &SyntaxToken, ctx: &mut RuleCtx) -> Option<()> {
        let range = token.text_range();

        match token.kind() {
            T![fn]
            | T![struct]
            | T![enum]
            | T![pub]
            | T![let]
            | T![impl]
            | T![type]
            | T![as]
            | T![use]
            | T![const]
            | T![for]
            | T![match]
            | T![break]
            | T![return]
            | T![continue]
            | T![if]
            | T![else]
            | T![loop]
            | T![in]
            | T![while]
            | T![and]
            | T![or]
            | T![true]
            | T![false] => {
                let mut modifiers = ModifierSet::empty();
                if matches!(token.kind(), T![pub]) {
                    modifiers |= SemanticTokenModifier::MODIFICATION;
                } else if matches!(token.kind(), T![true] | T![false]) {
                    modifiers |= SemanticTokenModifier::DEFAULT_LIBRARY;
                }

                self.push(range, SemanticTokenType::KEYWORD, modifiers);
            }

            COMMENT => {
                let mut modifiers = ModifierSet::empty();
                // Doc comments get the documentation modifier
                let comment_text = token.text(ctx.interner()).trim();
                if comment_text.starts_with("///") || comment_text.starts_with("/**") {
                    modifiers |= SemanticTokenModifier::DOCUMENTATION;
                }

                self.push(token.text_range(), SemanticTokenType::COMMENT, modifiers);
            }

            _ => {}
        }

        None
    }
}
