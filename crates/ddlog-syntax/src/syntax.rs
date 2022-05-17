use crate::{
    ast::{nodes, AstNode, AstToken},
    SyntaxElementRef, SyntaxKind, SyntaxNode, SyntaxText, SyntaxToken, TokenSet,
};
use cstree::{NodeOrToken, TextRange};
use ddlog_diagnostics::{FileId, Interner, Span};
use std::borrow::Cow;

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

/// Extensions to [`SyntaxNode`]
pub trait SyntaxNodeExt {
    #[doc(hidden)]
    fn to_node(&self) -> &SyntaxNode;

    /// Get all of the tokens of this node, recursively, including whitespace and comments.
    #[inline]
    fn tokens(&self) -> Vec<&SyntaxToken> {
        self.to_node()
            .descendants_with_tokens()
            .filter_map(|x| x.into_token())
            .collect()
    }

    /// Get all the tokens of this node, recursively, not including whitespace and comments.
    #[inline]
    fn lossy_tokens(&self) -> Vec<&SyntaxToken> {
        self.to_node()
            .descendants_with_tokens()
            .filter_map(|x| x.into_token().filter(|token| !token.kind().is_trivia()))
            .collect()
    }

    /// Get the first non-whitespace child token.
    #[inline]
    fn first_lossy_token(&self) -> Option<&SyntaxToken> {
        self.to_node()
            .children_with_tokens()
            .filter_map(|it| it.into_token().filter(|x| !x.kind().is_trivia()))
            .next()
    }

    /// Check if the node is a certain AST node and that it can be casted to it.
    #[inline]
    fn is<T>(&self) -> bool
    where
        T: AstNode,
    {
        T::can_cast_from(self.to_node().kind())
    }

    /// Cast this node to a certain AST node.
    ///
    /// # Panics
    ///
    /// Panics if the underlying node cannot be cast to the AST node
    ///
    #[inline]
    fn to<T>(&self) -> Cow<'_, T>
    where
        T: AstNode + Clone,
    {
        T::cast(self.to_node()).unwrap_or_else(|| {
            panic!(
                "Tried to cast node as `{:?}` but was unable to cast",
                stringify!(T),
            )
        })
    }

    /// Try to cast this node to a certain AST node
    #[inline]
    fn try_to<T>(&self) -> Option<Cow<'_, T>>
    where
        T: AstNode + Clone,
    {
        T::cast(self.to_node())
    }

    /// Compare two syntax nodes by comparing their underlying non-whitespace tokens.
    ///
    /// This is a more accurate way of comparing nodes because it does not count whitespace.
    /// Text based equality counts `foo. bar` and `foo.bar` as different, while this counts them as the same.
    ///
    // # Examples
    //
    // ```
    // use ddlog_syntax::{SyntaxNodeExt, parse_expr};
    //
    // let left = parse_expr("foo. bar", 0).syntax();
    // let right = parse_expr("foo.bar", 0).syntax();
    //
    // assert!(left.lexical_eq(&right));
    //
    // assert_ne!(left.text(), right.text());
    // ```
    #[inline]
    fn lexical_eq(&self, right: &SyntaxNode, interner: &Interner) -> bool {
        let left = self.lossy_tokens();
        let right = right.lossy_tokens();

        if left.len() == right.len() {
            left.iter()
                .zip(right.iter())
                // TODO: Compare interned strings
                .all(|(l, r)| l.resolve_text(interner) == r.resolve_text(interner))
        } else {
            false
        }
    }

    /// Get the text range of this node, not including any leading or trailing whitespace.
    //
    // # Examples
    //
    // ```
    // use ddlog_syntax::{SyntaxNodeExt, parse_expr, TextRange};
    //
    // let node = parse_expr(" foo. bar  ", 0).syntax();
    //
    // assert_eq!(node.trimmed_range(), TextRange::new(1.into(), 9.into()));
    //
    // assert_eq!(node.text_range(), TextRange::new(0.into(), 11.into()));
    // ```
    #[inline]
    fn trimmed_range(&self) -> TextRange {
        let node = self.to_node();
        let tokens = node.lossy_tokens();
        let start = tokens
            .first()
            .map(|t| t.text_range().start())
            .unwrap_or_else(|| 0.into());
        let end = tokens
            .last()
            .map(|t| t.text_range().end())
            .unwrap_or_else(|| 0.into());

        TextRange::new(start, end)
    }

    #[inline]
    fn trimmed_span(&self, file: FileId) -> Span {
        let node = self.to_node();
        let tokens = node.lossy_tokens();
        let start = tokens
            .first()
            .map(|t| t.text_range().start().into())
            .unwrap_or_else(|| 0);
        let end = tokens
            .last()
            .map(|t| t.text_range().end().into())
            .unwrap_or_else(|| 0);

        Span::new(start, end, file)
    }

    /// Get the text of this node, not including leading or trailing whitespace
    ///
    // # Examples
    //
    // ```
    // use ddlog_syntax::{SyntaxNodeExt, parse_expr, TextRange};
    //
    // let node = parse_expr(" foo. bar  ", 0).syntax();
    //
    // assert_eq!(node.trimmed_text(), "foo. bar");
    // ```
    #[inline]
    fn trimmed_text<'node, 'intern>(
        &'node self,
        interner: &'intern Interner,
    ) -> SyntaxText<'node, 'intern> {
        let trimmed = self.to_node().trimmed_range();
        let offset = self.to_node().text_range().start();

        self.to_node().resolve_text(interner).slice(TextRange::new(
            trimmed.start().checked_sub(offset).unwrap_or_default(),
            trimmed.end().checked_sub(offset).unwrap_or_default(),
        ))
    }

    /// Check whether this node's kind is contained in a token set.
    #[inline]
    fn in_set(&self, set: TokenSet) -> bool {
        set.contains(self.to_node().kind())
    }

    /// A human readable name for this node's kind. e.g.:
    /// `BREAK_STMT` => `Break statement`
    ///
    /// Returns a capitalized name without an underscore for anything not a statement. e.g.:
    /// `FN_DECL` => `Fn decl`
    fn readable_stmt_name(&self) -> String {
        pub fn substitute_underscores(string: &mut str) {
            for byte in unsafe { string.as_bytes_mut() } {
                if *byte == b'_' {
                    *byte = b' ';
                }
            }
        }

        let mut string = format!("{:?}", self.to_node().kind());
        string.make_ascii_lowercase();
        substitute_underscores(&mut string);

        if self.to_node().is::<nodes::Stmt>() {
            ddlog_utils::strings::replace_in_place(&mut string, "stmt", "statement");
        } else if self.to_node().is::<nodes::Expr>() {
            ddlog_utils::strings::replace_in_place(&mut string, "Expr", "expression");
        }

        string
    }

    /// Whether this node is an iteration statement.
    #[inline]
    fn is_loop(&self) -> bool {
        #[allow(dead_code)]
        const ITERATION_STMT: &[SyntaxKind] = &[
            // TODO
        ];
        ITERATION_STMT.contains(&self.to_node().kind())
    }

    /// Go over the descendants of this node, at every descendant call `func`, and keep traversing
    /// the descendants of that node if the function's return is `true`. If the function returns false
    /// then stop traversing the descendants of that node go on to the next child.
    ///
    /// For example:
    /// ```ignore
    /// ROOT
    ///     CHILD // <-- Call `F` with CHILD, `F` returns `false` so go on to the next child...
    ///         SUBCHILD
    ///     CHILD // <-- Call `F` with CHILD, `F` return `true` so go on to the children of CHILD
    ///         SUBCHILD // <-- Same thing
    ///             SUBSUBCHILD
    ///     CHILD // Go on to the next child and do the same thing
    /// ```
    #[inline]
    fn descendants_with<F>(&self, func: &mut F)
    where
        F: FnMut(&SyntaxNode) -> bool,
    {
        for node in self.to_node().children() {
            if func(node) {
                node.descendants_with(func);
            }
        }
    }

    /// Separate all the lossy tokens of this node, then compare each token's text with the corresponding
    /// text in `tokens`.
    #[inline]
    fn structural_lossy_token_eq(&self, tokens: &[impl AsRef<str>], interner: &Interner) -> bool {
        let node_tokens = self.to_node().lossy_tokens();
        if node_tokens.len() == tokens.len() {
            node_tokens
                .iter()
                .zip(tokens.iter())
                .all(|(lhs, rhs)| lhs.resolve_text(interner) == rhs.as_ref())
        } else {
            false
        }
    }

    /// Whether the node contains any comments.
    #[inline]
    fn contains_comments(&self) -> bool {
        self.tokens()
            .iter()
            .any(|tok| tok.kind() == SyntaxKind::COMMENT)
    }

    /// Get the first child with a specific kind.
    #[inline]
    fn child_with_kind(&self, kind: SyntaxKind) -> Option<&SyntaxNode> {
        self.to_node().children().find(|child| child.kind() == kind)
    }

    /// Get the parent of this node, recursing through any grouping expressions
    #[inline]
    fn expr_parent(&self) -> Option<&SyntaxNode> {
        let parent = self.to_node().parent()?;
        if parent.kind() == SyntaxKind::PAREN_EXPR {
            parent.expr_parent()
        } else {
            Some(parent)
        }
    }

    /// Get the first child in this node that can be casted to an AST node
    #[inline]
    fn child_with_ast<T>(&self) -> Option<Cow<'_, T>>
    where
        T: AstNode + Clone,
    {
        self.to_node().children().find_map(|child| child.try_to())
    }

    /// Same as [`descendants_with`](Self::descendants_with) but considers tokens too.
    #[inline]
    fn descendants_with_tokens_with<F>(&self, mut func: F)
    where
        F: FnMut(SyntaxElementRef) -> bool,
    {
        for elem in self.to_node().children_with_tokens() {
            match elem {
                NodeOrToken::Node(node) => {
                    if func(elem) {
                        node.descendants_with_tokens_with_ref(&mut func);
                    }
                }

                NodeOrToken::Token(_) => {
                    func(elem);
                }
            }
        }
    }

    #[inline]
    fn descendants_with_tokens_with_ref<F>(&self, func: &mut F)
    where
        F: FnMut(SyntaxElementRef) -> bool,
    {
        for elem in self.to_node().children_with_tokens() {
            match elem {
                NodeOrToken::Node(node) => {
                    if func(elem) {
                        node.descendants_with_tokens_with_ref(func);
                    }
                }

                NodeOrToken::Token(_) => {
                    func(elem);
                }
            }
        }
    }

    /// Get a specific token in the node which matches a syntax kind.
    ///
    /// This does not consider tokens in descendant nodes
    #[inline]
    fn token_with_kind(&self, kind: SyntaxKind) -> Option<&SyntaxToken> {
        self.to_node()
            .children_with_tokens()
            .find_map(|t| t.into_token().filter(|it| it.kind() == kind))
    }

    // FIXME: I don't like having to give the `FileId` here
    #[inline]
    fn span(&self, file: FileId) -> Span {
        let range = self.to_node().text_range();
        Span::new(range.start().into(), range.end().into(), file)
    }
}

impl SyntaxNodeExt for SyntaxNode {
    #[inline]
    fn to_node(&self) -> &SyntaxNode {
        self
    }
}

pub trait SyntaxTokenExt {
    fn to_token(&self) -> &SyntaxToken;

    /// Check whether this token's kind is contained in a token set.
    #[inline]
    fn in_set(&self, set: TokenSet) -> bool {
        set.contains(self.to_token().kind())
    }

    #[inline]
    fn text_range(&self) -> TextRange {
        self.to_token().text_range()
    }

    /// Check if the node is a certain AST token and that it can be casted to it.
    #[inline]
    fn is<T>(&self) -> bool
    where
        T: AstToken,
    {
        T::can_cast_from(self.to_token().kind())
    }

    /// Cast this token to a certain AST token.
    ///
    /// # Panics
    ///
    /// Panics if the underlying token cannot be cast to the AST token
    #[inline]
    fn to<T>(&self) -> Cow<'_, T>
    where
        T: AstToken + Clone,
    {
        T::cast(self.to_token()).unwrap_or_else(|| {
            panic!(
                "Tried to cast token as `{:?}` but was unable to cast",
                stringify!(T),
            )
        })
    }

    /// Try to cast this node to a certain AST token
    #[inline]
    fn try_to<T>(&self) -> Option<Cow<'_, T>>
    where
        T: AstToken + Clone,
    {
        T::cast(self.to_token())
    }

    // FIXME: I don't like having to give the `FileId` here
    #[inline]
    fn span(&self, file: FileId) -> Span {
        let range = self.to_token().text_range();
        Span::new(range.start().into(), range.end().into(), file)
    }

    #[inline]
    fn text<'intern>(&self, interner: &'intern Interner) -> &'intern str {
        self.to_token().resolve_text(interner)
    }

    #[inline]
    fn lexical_eq(&self, text: &str, interner: &Interner) -> bool {
        self.text(interner) == text
    }
}

impl SyntaxTokenExt for SyntaxToken {
    #[inline]
    fn to_token(&self) -> &SyntaxToken {
        self
    }
}
