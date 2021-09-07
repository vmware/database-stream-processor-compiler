/// Matches a [`SyntaxNode`][`crate::SyntaxNode`] against an [`ast`][`crate::ast::nodes`] type.
///
/// # Example:
///
/// ```ignore
/// match_ast! {
///     match node {
///         Root(root) => { ... },
///         BinExpr(bin) => { ... },
///         _ => None,
///     }
/// }
/// ```
// TODO: This would be more useful if it handled or patterns and stuff
#[macro_export]
macro_rules! match_ast {
    (match $node:ident {
        $($tt:tt)*
    }) => {
        $crate::match_ast!(match ($node) { $($tt)* })
    };

    (match ($node:expr) {
        $( $ast:ident($it:ident) => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = <$crate::ast::nodes::$ast as $crate::ast::AstNode>::cast(&$node) { $res } else )*
        { $catch_all }
    }};
}
