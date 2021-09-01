use crate::{
    lexer::Token,
    parser::event::Event,
    GreenNodeBuilder, NodeCache,
    SyntaxKind::{self, ROOT, TOMBSTONE},
};
use cstree::GreenNode;
use std::mem;

pub struct Sink<'src, 'cache, 'interner> {
    builder: GreenNodeBuilder<'cache, 'interner>,
    tokens: Vec<Token<'src>>,
    cursor: usize,
    events: Vec<Event>,
    source: &'src str,
}

impl<'src, 'cache, 'interner> Sink<'src, 'cache, 'interner> {
    pub fn new(
        source: &'src str,
        tokens: Vec<Token<'src>>,
        events: Vec<Event>,
        cache: &'cache mut NodeCache<'interner>,
    ) -> Self {
        Self {
            builder: GreenNodeBuilder::with_cache(cache),
            tokens,
            cursor: 0,
            events,
            source,
        }
    }

    fn token(&mut self, kind: SyntaxKind, text: &str) {
        self.cursor += 1;
        self.builder.token(kind.into(), text);
    }

    pub(crate) fn finish(mut self) -> GreenNode {
        let mut preceded_nodes = Vec::new();
        for idx in 0..self.events.len() {
            match mem::take(&mut self.events[idx]) {
                // Ignore tombstone events
                event @ Event::Enter { .. } if event.is_tombstone() => {}

                Event::Enter { kind, preceded_by } => {
                    preceded_nodes.push(kind);

                    if kind != ROOT {
                        self.eat_trivia();
                    }

                    let (mut idx, mut preceded_by) = (idx, preceded_by);
                    while let Some(rel_diff) = preceded_by {
                        idx += rel_diff.get();

                        preceded_by = match mem::take(&mut self.events[idx]) {
                            Event::Enter { kind, preceded_by } => {
                                if kind != TOMBSTONE {
                                    preceded_nodes.push(kind);
                                }

                                preceded_by
                            }

                            _ => unreachable!(),
                        }
                    }

                    for kind in preceded_nodes.drain(..).rev() {
                        self.builder.start_node(kind.into());
                    }

                    // Note: We eat trivia *after* entering all the required nodes
                    //       since otherwise this'll make us eat whitespace before
                    //       we can open up the root node, which is bad
                    self.eat_trivia();
                }

                Event::Exit => {
                    self.builder.finish_node();
                    self.eat_trivia();
                }

                Event::Token { kind, span } => {
                    self.eat_trivia();
                    self.token(kind, &self.source[span]);
                }
            }
        }

        let (node, interner) = self.builder.finish();
        debug_assert!(interner.is_none());

        node
    }

    fn eat_trivia(&mut self) {
        while let Some(&token) = self.tokens.get(self.cursor) {
            if !token.kind().is_trivia() {
                break;
            }

            self.token(token.kind(), token.text());
        }
    }
}
