use crate::{FileCache, Span};
use ariadne::{CharSet as OutputCharSet, Config, Report, ReportBuilder, ReportKind};
use std::{
    borrow::Cow,
    io::{self, Write},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Diagnostic {
    #[doc(hidden)]
    pub message: Option<Cow<'static, str>>,
    #[doc(hidden)]
    pub note: Option<Cow<'static, str>>,
    #[doc(hidden)]
    pub labels: Vec<Label>,
    #[doc(hidden)]
    pub level: Level,
    #[doc(hidden)]
    pub code: Option<u32>,
    #[doc(hidden)]
    pub message_span: Option<Span>,
}

impl Diagnostic {
    #[inline]
    pub const fn new(level: Level) -> Self {
        Self {
            message: None,
            note: None,
            labels: Vec::new(),
            level,
            code: None,
            message_span: None,
        }
    }

    #[inline]
    pub const fn error() -> Self {
        Self::new(Level::Error)
    }

    #[inline]
    pub const fn warning() -> Self {
        Self::new(Level::Warning)
    }

    #[inline]
    pub const fn note() -> Self {
        Self::new(Level::Note)
    }

    #[inline]
    pub fn with_message<M>(mut self, message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        self.message = Some(message.into());
        self
    }

    #[inline]
    pub const fn with_message_span(mut self, message_span: Span) -> Self {
        self.message_span = Some(message_span);
        self
    }

    #[inline]
    pub fn with_note<N>(mut self, note: N) -> Self
    where
        N: Into<Cow<'static, str>>,
    {
        self.note = Some(note.into());
        self
    }

    #[inline]
    pub const fn with_code(mut self, code: u32) -> Self {
        self.code = Some(code);
        self
    }

    #[inline]
    pub fn with_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    #[inline]
    #[track_caller]
    pub fn emit_to<W>(
        self,
        config: &DiagnosticConfig,
        cache: &mut FileCache,
        writer: W,
    ) -> io::Result<()>
    where
        W: Write,
    {
        let diagnostic = self.into_report(config);
        diagnostic.write(cache, writer)?;

        Ok(())
    }

    #[track_caller]
    fn into_report(self, config: &DiagnosticConfig) -> Report<Span> {
        let primary_span = self.message_span.or_else(|| {
            self.labels
                .iter()
                .find_map(|label| label.is_primary.then(|| label.span))
        })
        .expect("expected a primary label or a message span within a diagnostic but failed to get one");

        let mut diagnostic: ReportBuilder<Span> = Report::build(
            self.level.report_kind(),
            primary_span.file(),
            primary_span.start() as usize,
        );

        // Setup the proper config options for the diagnostic
        diagnostic = diagnostic.with_config(
            Config::default()
                .with_color(config.colored)
                .with_compact(config.compact)
                .with_char_set(config.charset.into_output()),
        );

        if let Some(message) = self.message {
            diagnostic = diagnostic.with_message(message);
        }
        if let Some(note) = self.note {
            diagnostic = diagnostic.with_note(note);
        }
        if let Some(code) = self.code {
            diagnostic = diagnostic.with_code(code);
        }

        for label in self.labels {
            diagnostic = diagnostic.with_label(label.into_report());
        }

        diagnostic.finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    Error,
    Warning,
    Note,
}

impl Level {
    fn report_kind(self) -> ReportKind {
        match self {
            Self::Error => ReportKind::Error,
            Self::Warning => ReportKind::Warning,
            Self::Note => ReportKind::Advice,
        }
    }
}

impl Default for Level {
    #[inline]
    fn default() -> Self {
        Self::Error
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    #[doc(hidden)]
    pub message: Option<Cow<'static, str>>,
    #[doc(hidden)]
    pub span: Span,
    #[doc(hidden)]
    pub is_primary: bool,
}

impl Label {
    #[inline]
    pub const fn primary(span: Span) -> Self {
        Self {
            message: None,
            span,
            is_primary: true,
        }
    }

    #[inline]
    pub const fn secondary(span: Span) -> Self {
        Self {
            message: None,
            span,
            is_primary: false,
        }
    }

    #[inline]
    pub fn with_message<M>(mut self, message: M) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        self.message = Some(message.into());
        self
    }

    fn into_report(self) -> ariadne::Label<Span> {
        let mut label = ariadne::Label::new(self.span);

        if let Some(message) = self.message {
            label = label.with_message(message);
        }
        label = label.with_priority(if self.is_primary { 1 } else { 0 });

        label
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticConfig {
    colored: bool,
    compact: bool,
    charset: CharSet,
}

impl DiagnosticConfig {
    #[inline]
    pub const fn new() -> Self {
        Self {
            colored: true,
            compact: false,
            charset: CharSet::Ascii,
        }
    }

    #[inline]
    pub const fn with_color(self, colored: bool) -> Self {
        Self { colored, ..self }
    }

    #[inline]
    pub const fn with_compact(self, compact: bool) -> Self {
        Self { compact, ..self }
    }

    #[inline]
    pub const fn with_charset(self, charset: CharSet) -> Self {
        Self { charset, ..self }
    }
}

impl Default for DiagnosticConfig {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharSet {
    Ascii,
    Unicode,
}

impl CharSet {
    const fn into_output(self) -> OutputCharSet {
        match self {
            Self::Ascii => OutputCharSet::Ascii,
            Self::Unicode => OutputCharSet::Unicode,
        }
    }
}

impl Default for CharSet {
    #[inline]
    fn default() -> Self {
        Self::Ascii
    }
}
