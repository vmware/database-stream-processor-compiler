use crate::{
    ast::{AstNode, AstToken},
    DifferentialDatalog, SyntaxKind, SyntaxNode,
};
use std::{borrow::Cow, marker::PhantomData};

type SyntaxNodeChildren<'parent> = cstree::SyntaxNodeChildren<'parent, DifferentialDatalog>;
type SyntaxElementChildren<'parent> = cstree::SyntaxElementChildren<'parent, DifferentialDatalog>;

/// An iterator over a [`SyntaxNode`]'s children, returning those of a particular ast type
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct AstChildren<'parent, N> {
    inner: Option<SyntaxNodeChildren<'parent>>,
    __type: PhantomData<N>,
}

impl<'parent, N> AstChildren<'parent, N> {
    /// Create a new [`AstChildren`] from the children of the given [`SyntaxNode`]
    #[inline]
    fn new(parent: &'parent SyntaxNode) -> Self {
        Self {
            inner: Some(parent.children()),
            __type: PhantomData,
        }
    }

    /// Create an empty [`AstChildren`]
    #[inline]
    pub const fn empty() -> Self {
        Self {
            inner: None,
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
        self.inner
            .as_mut()
            .and_then(|children| children.find_map(N::cast))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner
            .as_ref()
            .map_or((0, Some(0)), Iterator::size_hint)
    }
}

impl<'parent, N> ExactSizeIterator for AstChildren<'parent, N>
where
    N: AstNode + Clone + 'parent,
{
    // TODO: Does this do more work than `ExactSizeIterator::len()` should?
    #[inline]
    fn len(&self) -> usize {
        self.inner.as_ref().map_or(0, |children| {
            children
                .clone()
                .filter(|child| N::can_cast_from(child.kind()))
                .count()
        })
    }
}

impl<N> Default for AstChildren<'_, N> {
    /// Create an empty [`AstChildren`]
    ///
    /// See [`AstChildren::empty()`]
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

/// An iterator over a [`SyntaxNode`]'s children, returning those of a particular token type
#[derive(Debug, Clone)]
pub struct TokenChildren<'parent, T> {
    inner: Option<SyntaxElementChildren<'parent>>,
    __type: PhantomData<T>,
}

impl<'parent, T> TokenChildren<'parent, T> {
    /// Create a new [`TokenChildren`] from the children of the given [`SyntaxNode`]
    #[inline]
    fn new(parent: &'parent SyntaxNode) -> Self {
        Self {
            inner: Some(parent.children_with_tokens()),
            __type: PhantomData,
        }
    }

    /// Create an empty [`TokenChildren`]
    #[inline]
    pub const fn empty() -> Self {
        Self {
            inner: None,
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
        self.inner.as_mut().and_then(|children| {
            children.find_map(|child| child.as_token().copied().and_then(T::cast))
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner
            .as_ref()
            .map_or((0, Some(0)), Iterator::size_hint)
    }
}

impl<'parent, N> ExactSizeIterator for TokenChildren<'parent, N>
where
    N: AstToken + Clone + 'parent,
{
    // TODO: Does this do more work than `ExactSizeIterator::len()` should?
    #[inline]
    fn len(&self) -> usize {
        self.inner.as_ref().map_or(0, |children| {
            children
                .clone()
                .filter(|child| N::can_cast_from(child.kind()))
                .count()
        })
    }
}

impl<T> Default for TokenChildren<'_, T> {
    /// Create an empty [`TokenChildren`]
    ///
    /// See [`TokenChildren::empty()`]
    #[inline]
    fn default() -> Self {
        Self::empty()
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
pub(super) fn nth_child<N, const IDX: usize>(parent: &SyntaxNode) -> Option<Cow<'_, N>>
where
    N: AstNode + Clone,
{
    parent.children().filter_map(N::cast).nth(IDX)
}

#[inline]
pub(super) fn children<N>(parent: &SyntaxNode) -> AstChildren<'_, N>
where
    N: AstNode,
{
    AstChildren::new(parent)
}

#[inline]
pub(super) fn token_exists<T>(parent: &SyntaxNode) -> bool
where
    T: AstToken,
{
    parent.children_with_tokens().any(|child| {
        child
            .as_token()
            .map(|token| T::can_cast_from(token.kind()))
            .unwrap_or(false)
    })
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
pub(super) fn nth_token<T, const IDX: usize>(parent: &SyntaxNode) -> Option<Cow<'_, T>>
where
    T: AstToken + Clone,
{
    parent
        .children_with_tokens()
        .filter_map(|child| child.as_token().copied().and_then(T::cast))
        .nth(IDX)
}

#[inline]
pub(super) fn token_children<T>(parent: &SyntaxNode) -> TokenChildren<'_, T>
where
    T: AstToken,
{
    TokenChildren::new(parent)
}

#[cold]
#[track_caller]
#[inline(never)]
pub(super) fn failed_enum_to_node_cast(
    parent: &'static str,
    expected: &'static str,
    actual: SyntaxKind,
) -> ! {
    panic!("attempted to cast {parent} to a {expected}, but got {actual}")
}
