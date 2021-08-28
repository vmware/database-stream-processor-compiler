/// Shorthand for getting a [`SyntaxKind`] that uses short mnemonics
// TODO: Auto-generate this
#[macro_export]
#[rustfmt::skip]
macro_rules! T {
    (root) => { $crate::SyntaxKind::Root };

    (:) => { $crate::SyntaxKind::Colon };

    (=) => { $crate::SyntaxKind::Eq };
    (==) => { $crate::SyntaxKind::EqEq };
    (!=) => { $crate::SyntaxKind::Neq };
    (!) => { $crate::SyntaxKind::Bang };
    (+) => { $crate::SyntaxKind::Plus };
    (-) => { $crate::SyntaxKind::Minus };
    (*) => { $crate::SyntaxKind::Star };
    (/) => { $crate::SyntaxKind::Slash };

    ('{') => { $crate::SyntaxKind::LBrace };
    ('}') => { $crate::SyntaxKind::RBrace };
    ('[') => { $crate::SyntaxKind::LBracket };
    (']') => { $crate::SyntaxKind::RBracket };
    ('(') => { $crate::SyntaxKind::LParen };
    (')') => { $crate::SyntaxKind::RParen };

    (and) => { $crate::SyntaxKind::And };
    (or) => { $crate::SyntaxKind::Or };

    (bool) => { $crate::SyntaxKind::Bool };
    (number) => { $crate::SyntaxKind::Number };
    (ident) => { $crate::SyntaxKind::Ident };

    (var) => { $crate::SyntaxKind::VarKw };
    (function) => { $crate::SyntaxKind::FnKw };
    (relation) => { $crate::SyntaxKind::RelKw };
    (input) => { $crate::SyntaxKind::InputKw };
    (output) => { $crate::SyntaxKind::OutputKw };
    (typedef) => { $crate::SyntaxKind::TypedefKw };

    (comment) => { $crate::SyntaxKind::Comment };
    (whitespace) => { $crate::SyntaxKind::Whitespace };
    (error) => { $crate::SyntaxKind::Error };
    (eof) => { $crate::SyntaxKind::Eof };
    (tombstone) => { $crate::SyntaxKind::Tombstone };
}
