/// Matches a [`SyntaxNode`][`crate::SyntaxNode`] against an [`ast`][`crate::ast::nodes`] type.
///
/// # Examples
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
///
/// ```ignore
/// match_ast! {
///     match node {
///         Root(_) => { ... }
///         BinExpr(bin) => { ... }
///         something_else => { ... }
///     }
/// }
/// ```
///
// TODO: This would be more useful if it handled or patterns and stuff
#[macro_export]
macro_rules! match_ast {
    (match $node:ident {
        $($tt:tt)*
    }) => {{
        $crate::match_ast!(match ($node) { $($tt)* })
    }};

    (match ($node:expr) {
        $($arms:tt)*
    }) => {{
        match $node {
            node => {
                $crate::match_ast_binding!(
                    node
                    []
                    $($arms)*
                )
            }
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! match_ast_binding {
    ($node:ident [$($acc:tt)*] $ast:ident(_) => $res:expr, $($rest:tt)*) => {
        $crate::match_ast_binding!(
            $node
            [
                $($acc)*
                if <$crate::SyntaxNode as $crate::SyntaxNodeExt>::is::<$crate::ast::nodes::$ast>(&$node) {
                    $res
                } else
            ]
            $($rest)*
        )
    };
    ($node:ident [$($acc:tt)*] $ast:ident(_) => $res:block $($rest:tt)*) => {
        $crate::match_ast_binding!(
            $node
            [
                $($acc)*
                if <$crate::SyntaxNode as $crate::SyntaxNodeExt>::is::<$crate::ast::nodes::$ast>(&$node) {
                    $res
                } else
            ]
            $($rest)*
        )
    };

    ($node:ident [$($acc:tt)*] $ast:ident($it:tt) => $res:expr, $($rest:tt)*) => {
        $crate::match_ast_binding!(
            $node
            [
                $($acc)*
                if let ::core::option::Option::Some($it) =
                    <$crate::ast::nodes::$ast as $crate::ast::AstNode>::cast(&$node)
                {
                    $res
                } else
            ]
            $($rest)*
        )
    };
    ($node:ident [$($acc:tt)*] $ast:ident($it:tt) => $res:block $($rest:tt)*) => {
        $crate::match_ast_binding!(
            $node
            [
                $($acc)*
                if let ::core::option::Option::Some($it) =
                    <$crate::ast::nodes::$ast as $crate::ast::AstNode>::cast(&$node)
                {
                    $res
                } else
            ]
            $($rest)*
        )
    };

    ($node:ident [$($acc:tt)*] $binding:ident => $catch_all:expr $(,)?) => {
        $($acc)* {
            match $node {
                binding => { $catch_all }
            }
        }
    };
    ($node:ident [$($acc:tt)*] _ => $catch_all:expr $(,)?) => {
        $($acc)* { $catch_all }
    };
}
