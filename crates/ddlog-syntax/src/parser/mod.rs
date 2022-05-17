mod event;
mod expr;
mod item;
mod precedence;
pub(crate) mod sink;
pub(crate) mod source;
mod stmt;
mod tests;
mod types;
mod utils;

use crate::{
    parser::{event::Event, expr::EXPR_RECOVERY_SET, source::Source},
    Span,
    SyntaxKind::{self, EOF, ERROR, ROOT, TOMBSTONE},
    Token, TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};
use std::{cell::Cell, fmt::Debug, num::NonZeroUsize, panic::Location, rc::Rc, thread};

// FIXME: Add recursion checks
pub(crate) struct Parser<'src, 'token> {
    // TODO: Factor into `TokenSource` abstraction
    source: Source<'src, 'token>,
    events: Vec<Event>,
    // TODO: Create an error sink
    errors: Vec<Diagnostic>,
    recovery_set: TokenSet,
    // This is for tracking if the parser is infinitely recursing
    steps: Rc<Cell<u32>>,
}

impl<'src, 'token> Parser<'src, 'token> {
    pub(crate) fn new(tokens: &'token [Token<'src>], end_of_file: Span) -> Self {
        Self {
            source: Source::new(tokens, end_of_file),
            events: Vec::with_capacity(1024),
            errors: Vec::new(),
            recovery_set: TokenSet::empty(),
            steps: Rc::new(Cell::new(0)),
        }
    }

    // test fibonacci
    // - fn fibonacci(n: usize) -> usize {
    // -     if n == 0 or n == 1 {
    // -         n
    // -     } else {
    // -         fibonacci(n - 1) + fibonacci(n - 2)
    // -     }
    // - }
    //
    // test paths
    // - fn main() {
    // -     let edges = input("edges");
    // -     let paths = edges.iterative(|edges| {
    // -         edges
    // -             .join(edges.reverse())
    // -             .distinct()
    // -     });
    // -     output(paths, "paths");
    // - }
    pub(crate) fn parse(mut self) -> (Vec<Event>, Vec<Diagnostic>) {
        self.root();

        (self.events, self.errors)
    }

    pub(crate) fn parse_expr(mut self) -> (Vec<Event>, Vec<Diagnostic>) {
        let root = self.start();

        while !self.at_end() {
            if self.expr().is_none() {
                // TODO: Get full span text
                let source = self.current_text();
                let span = self.error_eat_until(EXPR_RECOVERY_SET);

                let error = Diagnostic::error()
                    .with_message("expected an expression")
                    .with_label(Label::primary(span).with_message(format!(
                        "expected an ident, literal, '-', '!', '(' or '{{', got '{}'",
                        source
                    )));

                self.push_error(error);
                break;
            }
        }

        root.complete(&mut self, ROOT);
        (self.events, self.errors)
    }

    pub(crate) fn parse_stmt(mut self) -> (Vec<Event>, Vec<Diagnostic>) {
        let root = self.start();

        while !self.at_end() {
            if self.stmt().is_none() {
                // TODO: Get full span text
                let source = self.current_text();
                // TODO: Statement recovery set
                let span = self.error_eat_until(EXPR_RECOVERY_SET);

                let error = Diagnostic::error()
                    .with_message("expected a statement")
                    .with_label(Label::primary(span).with_message(format!(
                        "expected an ident, literal, '-', '!', '(' or '{{', got '{}'",
                        source,
                    )));

                self.push_error(error);
                break;
            }
        }

        root.complete(&mut self, ROOT);
        (self.events, self.errors)
    }
}

/// Utility functions
impl<'src, 'token> Parser<'src, 'token> {
    fn root(&mut self) -> CompletedMarker {
        let marker = self.start();
        self.items();

        marker.complete(self, ROOT)
    }

    fn stack_frame(&self) -> StackFrame {
        StackFrame::new(self.steps.clone())
    }

    fn eat_semicolons(&mut self) {
        while self.try_expect(T![;]) {}
    }

    fn eat_commas(&mut self) {
        while self.try_expect(T![,]) {}
    }

    #[track_caller]
    fn bump_span(&mut self) -> Span {
        if let Some(token) = self.source.next() {
            tracing::trace!(
                token = ?token.kind(),
                text = token.text(),
                span = ?token.span(),
                "bumped parser",
            );

            self.push_event(Event::Token {
                kind: token.kind(),
                span: token.span(),
            });

            token.span()
        } else {
            tracing::trace!("bumped at end of file");

            self.source.end_of_file()
        }
    }

    #[track_caller]
    fn bump(&mut self) {
        self.bump_span();
    }

    fn try_peek(&mut self) -> Option<SyntaxKind> {
        self.source.try_peek_kind()
    }

    fn peek(&mut self) -> SyntaxKind {
        self.source.peek_kind()
    }

    fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.push_event(Event::tombstone());

        Marker::new(pos)
    }

    fn push_event(&mut self, event: Event) {
        self.events.push(event);
    }

    fn error(&mut self, error: Diagnostic) {
        if !self.at_set(self.recovery_set) && !self.at_end() {
            let marker = self.start();
            self.bump();
            marker.complete(self, ERROR);
        }

        self.push_error(error);
    }

    fn push_error(&mut self, error: Diagnostic) {
        self.errors.push(error);
    }

    #[track_caller]
    fn error_eat_until(&mut self, set: TokenSet) -> Span {
        let caller = Location::caller();
        tracing::trace!(
            %set,
            "eating until error set @ {}:{}:{}",
            caller.file(),
            caller.line(),
            caller.column(),
        );

        let marker = self.start();
        let span = self.eat_until_set(set);
        marker.complete(self, ERROR);

        tracing::trace!(
            %set,
            "finished eating until error set @ {}:{}:{}",
            caller.file(),
            caller.line(),
            caller.column(),
        );

        span
    }

    fn at_end(&mut self) -> bool {
        self.peek() == EOF
    }

    /// Get the current token kind of the parser
    fn current(&mut self) -> SyntaxKind {
        self.nth(0)
    }

    /// Get the current token of the parser
    fn current_token(&mut self) -> Token<'src> {
        self.nth_token(0)
    }

    /// Get the current span of the parser
    fn current_span(&mut self) -> Span {
        self.current_token().span()
    }

    /// Get the current text of the parser
    fn current_text(&mut self) -> &'src str {
        self.current_token().text()
    }

    /// Check if the parser is currently at a specific token
    fn at(&mut self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    fn at_set(&mut self, set: TokenSet) -> bool {
        set.contains(self.peek())
    }

    fn eat_until_set(&mut self, set: TokenSet) -> Span {
        let mut last_span = self.current_span();
        while !self.at_set(set) && !self.at_end() {
            last_span = self.bump_span();
        }

        last_span
    }

    /// Check if a token lookahead is something, `n` must be smaller or equal to `4`
    fn nth_at(&mut self, n: usize, kind: SyntaxKind) -> bool {
        self.source.peek_nth(n).kind() == kind
    }

    /// Look ahead at a token and get its kind, **The max lookahead is 4**.  
    fn nth(&mut self, n: usize) -> SyntaxKind {
        self.source.peek_nth(n).kind()
    }

    fn nth_token(&mut self, n: usize) -> Token<'src> {
        self.source.peek_nth(n)
    }

    /// Consume the next token if `kind` matches
    #[allow(dead_code)]
    fn eat(&mut self, kind: SyntaxKind) -> bool {
        self.eat_span(kind).is_some()
    }

    fn eat_span(&mut self, kind: SyntaxKind) -> Option<Span> {
        if !self.at(kind) {
            return None;
        }

        Some(self.bump_span())
    }

    /// Try to eat a token of the given kind or add an error to the events stack
    #[track_caller]
    fn expect(&mut self, kind: SyntaxKind) -> bool {
        self.expect_span(kind).is_some()
    }

    /// Try to eat a token of the given kind or add an error to the events stack
    #[track_caller]
    fn expect_span(&mut self, kind: SyntaxKind) -> Option<Span> {
        if let Some(span) = self.eat_span(kind) {
            Some(span)
        } else {
            let caller = Location::caller();
            tracing::trace!(
                "expected '{}', got '{}' @ {}:{}:{}",
                kind,
                self.current(),
                caller.file(),
                caller.line(),
                caller.column(),
            );

            let error = if self.current() == EOF {
                Diagnostic::error()
                    .with_message(format!("expected '{}' but instead the file ends", kind))
                    .with_label(
                        Label::primary(self.current_span()).with_message("the file ends here"),
                    )
            } else {
                Diagnostic::error()
                    .with_message(format!(
                        "expected '{}' but instead found '{:?}'",
                        kind,
                        self.current_text(),
                    ))
                    .with_label(
                        Label::primary(self.current_span()).with_message("unexpected token"),
                    )
            };
            self.error(error);

            None
        }
    }

    /// Returns `true` if the current token is the same as `kind`
    fn try_expect(&mut self, kind: SyntaxKind) -> bool {
        self.try_expect_span(kind).is_some()
    }

    fn try_expect_span(&mut self, kind: SyntaxKind) -> Option<Span> {
        if self.at(kind) {
            self.expect_span(kind)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Marker {
    position: usize,
    completed: bool,
}

impl Marker {
    pub(super) const fn new(position: usize) -> Self {
        Self {
            position,
            completed: false,
        }
    }

    /// Finishes the syntax tree node and assigns `kind` to it,
    /// and mark the create a [`CompletedMarker`] for possible future
    /// operation like [`CompletedMarker::precede()`] to deal with preceded_by.
    pub(super) fn complete(
        mut self,
        parser: &mut Parser<'_, '_>,
        kind: SyntaxKind,
    ) -> CompletedMarker {
        self.completed = true;

        let event_at_pos = &mut parser.events[self.position];
        assert_eq!(*event_at_pos, Event::tombstone());

        *event_at_pos = Event::Enter {
            kind,
            preceded_by: None,
        };

        parser.events.push(Event::Exit);

        CompletedMarker {
            position: self.position,
            _kind: kind,
        }
    }

    // /// Deletes the entered node and any children since.
    // pub(crate) fn discard(mut self, parser: &mut Parser<'_, '_>) {
    //     self.completed = true;
    //
    //     drop(parser.events.drain(self.position..));
    // }

    /// Deletes the node, but retains its children.
    /// The children will move to the node's parent.
    pub(crate) fn abandon(mut self, parser: &mut Parser<'_, '_>) {
        self.completed = true;

        match &mut parser.events[self.position] {
            Event::Enter {
                kind,
                preceded_by: None,
            } => {
                *kind = TOMBSTONE;
            }

            _ => unreachable!(),
        }
        debug_assert_eq!(parser.events[self.position], Event::tombstone());

        if self.position == parser.events.len() - 1 {
            // If we don't need to reorder the vec for this,
            // just remove the placeholder event
            parser.events.pop();
        }
    }
}

impl Drop for Marker {
    #[inline]
    #[track_caller]
    fn drop(&mut self) {
        #[cold]
        #[track_caller]
        #[inline(never)]
        fn incomplete_marker() -> ! {
            panic!("failed to complete marker before dropping it")
        }

        if !self.completed && !thread::panicking() {
            incomplete_marker()
        }
    }
}

#[derive(Debug)]
struct CompletedMarker {
    position: usize,
    _kind: SyntaxKind,
}

impl CompletedMarker {
    fn precede(self, parser: &mut Parser<'_, '_>) -> Marker {
        let marker = parser.start();

        if let Event::Enter { preceded_by, .. } = &mut parser.events[self.position] {
            *preceded_by = NonZeroUsize::new(marker.position - self.position);
        } else {
            unreachable!();
        }

        marker
    }
}

struct StackFrame {
    steps: Rc<Cell<u32>>,
}

impl StackFrame {
    fn new(steps: Rc<Cell<u32>>) -> Self {
        let num_steps = steps.get();
        steps.set(num_steps + 1);

        Self { steps }
    }
}

impl Drop for StackFrame {
    #[track_caller]
    fn drop(&mut self) {
        #[cold]
        #[track_caller]
        #[inline(never)]
        fn step_overflow() -> ! {
            panic!("the parser seems to be recursing forever")
        }

        let steps = self.steps.get();
        self.steps.set(steps - 1);

        if !thread::panicking() && steps >= 1_000_000 {
            step_overflow()
        }
    }
}
