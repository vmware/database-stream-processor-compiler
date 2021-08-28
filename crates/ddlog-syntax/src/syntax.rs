use crate::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DifferentialDatalog {}

impl cstree::Language for DifferentialDatalog {
    type Kind = SyntaxKind;

    #[inline]
    #[track_caller]
    fn kind_from_raw(raw: cstree::SyntaxKind) -> Self::Kind {
        SyntaxKind::from(raw)
    }

    #[inline]
    fn kind_to_raw(kind: Self::Kind) -> cstree::SyntaxKind {
        kind.into()
    }
}
