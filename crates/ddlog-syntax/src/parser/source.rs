use crate::{
    Span,
    SyntaxKind::{self, EOF},
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
        self.eat_trivia();

        let token = *self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub(super) const fn end_of_file(&self) -> Span {
        self.end_of_file
    }

    pub(super) fn peek_kind(&mut self) -> SyntaxKind {
        self.eat_trivia();
        self.peek_kind_raw().unwrap_or(EOF)
    }

    pub(super) fn peek_nth(&mut self, n: usize) -> Token<'src> {
        debug_assert!(n <= 4);
        self.eat_trivia();

        self.tokens
            .get(self.cursor + n)
            .copied()
            .unwrap_or_else(|| Token::new(EOF, self.end_of_file, ""))
    }

    fn peek_kind_raw(&mut self) -> Option<SyntaxKind> {
        self.tokens.get(self.cursor).map(Token::kind)
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if !token.kind().is_trivia() {
                break;
            }

            self.cursor += 1;
        }
    }
}
