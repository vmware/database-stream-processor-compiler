#[macro_use]
mod generated;
mod tests;

pub use generated::SyntaxKind;

impl SyntaxKind {
    #[inline]
    pub const fn is_trivia(self) -> bool {
        matches!(self, Self::WHITESPACE | Self::COMMENT)
    }
}
