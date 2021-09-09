mod extra_commas;
mod modifiers;

use crate::{
    ast::nodes::Root, visitor::RuleCtx, AstVisitor, SyntaxElementRef, SyntaxKind, SyntaxNode,
    SyntaxNodeExt,
};
use extra_commas::ExtraCommas;
use modifiers::ModifierValidator;

pub fn run_validators(node: &SyntaxNode, ctx: &mut RuleCtx) {
    let validators = validators();
    for mut validator in validators {
        if node.is::<Root>() {
            validator.check_root(node, ctx);
        }

        node.descendants_with_tokens_with(|elem| {
            match elem {
                SyntaxElementRef::Node(node) => {
                    if node.kind() == SyntaxKind::ERROR {
                        return false;
                    }

                    validator.check_node(node, ctx);
                }

                SyntaxElementRef::Token(token) => {
                    validator.check_token(token, ctx);
                }
            }

            true
        });
    }
}

fn validators() -> Vec<Box<dyn AstVisitor>> {
    vec![Box::new(ModifierValidator), Box::new(ExtraCommas)]
}
