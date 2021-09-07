use cstree::TextRange;
use ddlog_diagnostics::Rope;
use lspower::lsp::{Position, Range};

pub fn ide_range(source: &Rope, text_range: TextRange) -> Range {
    let (start, end) = (text_range.start().into(), text_range.end().into());

    let start_line = source.byte_to_line(start);
    let start_line_byte = source.line_to_byte(start_line);
    let start = source.byte_to_char(start) - start_line_byte;

    let end_line = source.byte_to_line(end);
    let end_line_byte = source.line_to_byte(end_line);
    let end = source.byte_to_char(end) - end_line_byte;

    Range::new(
        Position::new(start_line as u32, start as u32),
        Position::new(end_line as u32, end as u32),
    )
}
