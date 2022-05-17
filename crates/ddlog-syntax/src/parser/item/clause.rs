use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{CLAUSE_DEF, CLAUSE_FACT, CLAUSE_FACT_ARG, CLAUSE_RULE, CLAUSE_RULES},
    TokenSet,
};

const FACT_END_SET: TokenSet = token_set! { T![:-], T![;] };

impl Parser<'_, '_> {
    // test clauses
    // - Foo() :- ();
    // - Baz(10, x * y) :- Foo(x, y);
    // - Bar(x, y, z) :- Bar(z, y, x);
    // - Baz(a, b), Bar(a, c, b) :- Bar(a, b, c);
    // test chained_clauses
    // - Bar(x, y, z) :-
    // -     A(b, c, d, e),
    // -     B(x, y, z),
    // -     X(y + z, f(x)),
    // -     x == b,
    // -     b == c,
    // -     e(c, x, y);
    // test empty_fact
    // - Foo(bar, baz, bing);
    // TODO: Detect `: -` and suggest correcting it to `:-`
    pub(super) fn clause_def(&mut self, clause: Marker) -> Option<CompletedMarker> {
        while !self.at_set(FACT_END_SET) {
            let fact = self.start();

            // TODO: Error handling
            self.clause_fact().unwrap();
            self.eat_commas();

            fact.complete(self, CLAUSE_FACT);
        }

        // If there's a horn clause implication, the facts have attached rules
        if self.at(T![:-]) {
            // TODO: Error handling
            self.clause_rules().unwrap();
        }

        self.expect(T![;]);

        Some(clause.complete(self, CLAUSE_DEF))
    }

    fn clause_rules(&mut self) -> Option<CompletedMarker> {
        let rules = self.start();
        self.expect(T![:-]);

        while !self.at(T![;]) {
            // TODO: Error handling
            self.clause_rule().unwrap();
        }

        Some(rules.complete(self, CLAUSE_RULES))
    }

    fn clause_rule(&mut self) -> Option<CompletedMarker> {
        let rule = self.start();

        // TODO: Error handling
        self.expr().unwrap();
        self.eat_commas();

        Some(rule.complete(self, CLAUSE_RULE))
    }

    fn clause_fact(&mut self) -> Option<CompletedMarker> {
        let relation = self.start();

        self.ident();
        self.expect(T!['(']);

        while !self.at(T![')']) {
            let arg = self.start();

            // TODO: Error handling
            self.expr().unwrap();
            self.eat_commas();

            arg.complete(self, CLAUSE_FACT_ARG);
        }

        self.expect(T![')']);

        Some(relation.complete(self, CLAUSE_FACT))
    }
}
