mod event;
mod expr;
mod item;
pub(crate) mod sink;
pub(crate) mod source;
mod tests;

use crate::{
    parser::{event::Event, source::Source},
    FileId, Span,
    SyntaxKind::{self, EOF, ERROR, ROOT, TOMBSTONE},
    Token, TokenSet,
};
use ariadne::{Label, Report, ReportKind};
use std::{num::NonZeroUsize, thread};

// FIXME: Add recursion checks
pub(crate) struct Parser<'src, 'token> {
    // TODO: Factor into `TokenSource` abstraction
    source: Source<'src, 'token>,
    events: Vec<Event>,
    errors: Vec<Report<Span>>,
    file: FileId,
    recovery_set: TokenSet,
}

impl<'src, 'token> Parser<'src, 'token> {
    pub(crate) fn new(tokens: &'token [Token<'src>], end_of_file: Span) -> Self {
        Self {
            source: Source::new(tokens, end_of_file),
            events: Vec::with_capacity(1024),
            errors: Vec::new(),
            file: end_of_file.file(),
            recovery_set: TokenSet::empty(),
        }
    }

    pub(crate) fn parse(mut self) -> (Vec<Event>, Vec<Report<Span>>) {
        self.root();

        (self.events, self.errors)
    }

    pub(crate) fn parse_expr(mut self) -> (Vec<Event>, Vec<Report<Span>>) {
        let marker = self.start();
        while !self.at_end() {
            self.expr();
        }
        marker.complete(&mut self, ROOT);

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

    #[track_caller]
    fn bump(&mut self) {
        let token = self
            .source
            .next()
            .expect("bumped while at the end of input");

        tracing::debug!(
            token = ?token.kind(),
            text = token.text(),
            span = ?token.span(),
            "bumped parser",
        );

        self.push_event(Event::Token {
            kind: token.kind(),
            span: token.span(),
        });
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

    fn error(&mut self, error: Report<Span>) {
        if !self.at_set(self.recovery_set) && !self.at_end() {
            let marker = self.start();
            self.bump();
            marker.complete(self, ERROR);
        }

        self.errors.push(error);
    }

    fn at_end(&mut self) -> bool {
        self.peek() == EOF
    }

    /// Get the current token kind of the parser
    fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// Get the current token of the parser
    fn current_token(&self) -> Token {
        self.nth_token(0)
    }

    /// Get the current span of the parser
    fn current_span(&self) -> Span {
        self.nth_token(0).span()
    }

    /// Get the current text of the parser
    fn current_text(&self) -> &'src str {
        self.nth_token(0).text()
    }

    /// Check if the parser is currently at a specific token
    fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    fn at_set(&mut self, set: TokenSet) -> bool {
        set.contains(self.peek())
    }

    /// Check if a token lookahead is something, `n` must be smaller or equal to `4`
    fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        self.source.peek_nth(n).kind() == kind
    }

    /// Look ahead at a token and get its kind, **The max lookahead is 4**.  
    fn nth(&self, n: usize) -> SyntaxKind {
        self.source.peek_nth(n).kind()
    }

    fn nth_token(&self, n: usize) -> Token<'src> {
        self.source.peek_nth(n)
    }

    /// Consume the next token if `kind` matches
    fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        self.bump();

        true
    }

    /// Try to eat a token of the given kind or add an error to the events stack
    fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            true
        } else {
            let error = if self.current() == EOF {
                Report::build(
                    ReportKind::Error,
                    self.file,
                    self.current_span().start() as usize,
                )
                .with_message(format!("expected `{}` but instead the file ends", kind,))
                .with_label(Label::new(self.current_span()).with_message("the file ends here"))
                .finish()
            } else {
                Report::build(
                    ReportKind::Error,
                    self.file,
                    self.current_span().start() as usize,
                )
                .with_message(format!(
                    "expected `{}` but instead found `{}`",
                    kind,
                    self.current_text()
                ))
                .with_label(Label::new(self.current_span()).with_message("unexpected"))
                .finish()
            };
            self.error(error);

            false
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
            kind,
        }
    }

    /// Deletes the entered node and any children since.
    pub(crate) fn discard(mut self, parser: &mut Parser<'_, '_>) {
        self.completed = true;

        drop(parser.events.drain(self.position..));
    }

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

        if self.position == parser.events.len() - 1 {
            // if we don't need to reorder the vec for this,
            // actually remove the placeholder event
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
    kind: SyntaxKind,
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
