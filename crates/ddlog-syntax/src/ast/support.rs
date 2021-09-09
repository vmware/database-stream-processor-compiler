use crate::{
    ast::{AstNode, AstToken},
    DifferentialDatalog, SyntaxNode,
};
use std::{borrow::Cow, marker::PhantomData};

type SyntaxNodeChildren<'parent> = cstree::SyntaxNodeChildren<'parent, DifferentialDatalog>;
type SyntaxElementChildren<'parent> = cstree::SyntaxElementChildren<'parent, DifferentialDatalog>;

/// An iterator over a [`SyntaxNode`]'s children, returning those of a particular ast type
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct AstChildren<'parent, N> {
    inner: SyntaxNodeChildren<'parent>,
    __type: PhantomData<N>,
}

impl<'parent, N> AstChildren<'parent, N> {
    #[inline]
    fn new(parent: &'parent SyntaxNode) -> Self {
        Self {
            inner: parent.children(),
            __type: PhantomData,
        }
    }
}

impl<'parent, N> Iterator for AstChildren<'parent, N>
where
    N: AstNode + Clone + 'parent,
{
    type Item = Cow<'parent, N>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find_map(N::cast)
    }
}

/// An iterator over a [`SyntaxNode`]'s children, returning those of a particular token type
#[derive(Debug, Clone)]
pub struct TokenChildren<'parent, T> {
    inner: SyntaxElementChildren<'parent>,
    __type: PhantomData<T>,
}

impl<'parent, T> TokenChildren<'parent, T> {
    #[inline]
    fn new(parent: &'parent SyntaxNode) -> Self {
        Self {
            inner: parent.children_with_tokens(),
            __type: PhantomData,
        }
    }
}

impl<'parent, T> Iterator for TokenChildren<'parent, T>
where
    T: AstToken + Clone + 'parent,
{
    type Item = Cow<'parent, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .find_map(|child| child.as_token().copied().and_then(T::cast))
    }
}

#[inline]
pub(super) fn child<N>(parent: &SyntaxNode) -> Option<Cow<'_, N>>
where
    N: AstNode + Clone,
{
    parent.children().find_map(N::cast)
}

#[inline]
pub(super) fn children<N>(parent: &SyntaxNode) -> AstChildren<'_, N>
where
    N: AstNode,
{
    AstChildren::new(parent)
}

#[inline]
pub(super) fn token<T>(parent: &SyntaxNode) -> Option<Cow<'_, T>>
where
    T: AstToken + Clone,
{
    parent
        .children_with_tokens()
        .find_map(|child| child.as_token().copied().and_then(T::cast))
}

#[inline]
pub(super) fn token_children<T>(parent: &SyntaxNode) -> TokenChildren<'_, T>
where
    T: AstToken,
{
    TokenChildren::new(parent)
}
