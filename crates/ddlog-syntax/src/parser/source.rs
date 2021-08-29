use crate::{
    Span,
    SyntaxKind::{self, EOF, WHITESPACE},
    Token,
};

pub struct Source<'src, 'token> {
    tokens: &'token [Token<'src>],
    cursor: usize,
    end_of_file: Span,
}

impl<'src, 'token> Source<'src, 'token> {
    pub const fn new(tokens: &'token [Token<'src>], end_of_file: Span) -> Self {
        Self {
            tokens,
            cursor: 0,
            end_of_file,
        }
    }

    pub fn next(&mut self) -> Option<Token<'src>> {
        self.eat_whitespace();

        let token = *self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub(super) fn peek_kind(&mut self) -> SyntaxKind {
        self.eat_whitespace();
        self.peek_kind_raw().unwrap_or(EOF)
    }

    pub(super) fn peek_nth(&self, n: usize) -> Token<'src> {
        debug_assert!(n <= 4);

        let mut idx = self.cursor + n;
        while self.tokens.get(idx).map(Token::kind) == Some(WHITESPACE) {
            idx += 1;
        }

        self.tokens
            .get(idx)
            .copied()
            .unwrap_or_else(|| Token::new(EOF, self.end_of_file, ""))
    }

    fn peek_kind_raw(&self) -> Option<SyntaxKind> {
        self.tokens.get(self.cursor).map(Token::kind)
    }

    fn eat_whitespace(&mut self) {
        while self.peek_kind_raw() == Some(WHITESPACE) {
            self.cursor += 1;
        }
    }
}
