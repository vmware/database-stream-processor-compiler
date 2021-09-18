use crate::ast::nodes::{FunctionDef, Item, RelationDef, Root, TypeDef};
use std::ops::Deref;

#[allow(unused_variables)]
pub trait AstVisitor {
    #[inline]
    #[doc(hidden)]
    fn default_root(&mut self, root: &Root) {
        for item in root.items() {
            self.visit_item(item.deref());
        }
    }

    #[inline]
    fn visit_root(&mut self, root: &Root) {
        self.default_root(root);
    }

    #[inline]
    #[doc(hidden)]
    fn default_item(&mut self, item: &Item) {
        match item {
            Item::FunctionDef(node) => self.visit_function_def(node),
            Item::RelationDef(node) => self.visit_relation_def(node),
            Item::TypeDef(node) => self.visit_type_def(node),
        }
    }

    #[inline]
    fn visit_item(&mut self, item: &Item) {
        self.default_item(item);
    }

    #[inline]
    #[doc(hidden)]
    fn default_function_def(&mut self, func: &FunctionDef) {
        if let Some(attributes) = func.attributes() {
            self.visit_attributes(attributes.deref());
        }

        if let Some(modifiers) = func.modifiers() {
            self.visit_modifiers(modifiers.deref());
        }

        if let Some(function_name) = func.name() {
            self.visit_function_name(function_name.deref());
        }

        if let Some(generics) = func.generics() {
            self.visit_generics(generics.deref());
        }

        if let Some(args) = func.args() {
            self.visit_function_args(args.deref());
        }

        if let Some(ret) = func.ret() {
            self.visit_function_return(ret.deref());
        }

        if let Some(body) = func.body() {
            self.visit_block(body.deref());
        }
    }

    #[inline]
    fn visit_function_def(&mut self, func: &FunctionDef) {
        self.default_function_def(func);
    }

    #[inline]
    fn visit_relation_def(&mut self, node: &RelationDef) {}

    #[inline]
    fn visit_type_def(&mut self, node: &TypeDef) {}

    #[inline]
    fn visit_attributes(&mut self, node: &crate::ast::nodes::Attributes) {}

    #[inline]
    fn visit_modifiers(&mut self, node: &crate::ast::nodes::Modifiers) {}

    #[inline]
    fn visit_rel_name(&mut self, node: &crate::ast::nodes::RelName) {}

    #[inline]
    fn visit_rel_cols(&mut self, node: &crate::ast::nodes::RelCols) {}

    #[inline]
    fn visit_rel_col(&mut self, node: &crate::ast::nodes::RelCol) {}

    #[inline]
    fn visit_pattern(&mut self, node: &crate::ast::nodes::Pattern) {}

    #[inline]
    fn visit_type_token(&mut self, node: &crate::ast::nodes::Type) {}

    #[inline]
    fn visit_function_name(&mut self, node: &crate::ast::nodes::FunctionName) {}

    #[inline]
    fn visit_generics(&mut self, node: &crate::ast::nodes::Generics) {}

    #[inline]
    fn visit_function_args(&mut self, node: &crate::ast::nodes::FunctionArgs) {}

    #[inline]
    fn visit_function_return(&mut self, node: &crate::ast::nodes::FunctionReturn) {}

    #[inline]
    fn visit_block(&mut self, node: &crate::ast::nodes::Block) {}

    #[inline]
    fn visit_function_arg(&mut self, node: &crate::ast::nodes::FunctionArg) {}

    #[inline]
    fn visit_type_name(&mut self, node: &crate::ast::nodes::TypeName) {}

    #[inline]
    fn visit_type_body(&mut self, node: &crate::ast::nodes::TypeBody) {}

    #[inline]
    fn visit_record_type(&mut self, node: &crate::ast::nodes::RecordType) {}

    #[inline]
    fn visit_sum_type(&mut self, node: &crate::ast::nodes::SumType) {}

    #[inline]
    fn visit_record_name(&mut self, node: &crate::ast::nodes::RecordName) {}

    #[inline]
    fn visit_record_field(&mut self, node: &crate::ast::nodes::RecordField) {}

    #[inline]
    fn visit_attribute(&mut self, node: &crate::ast::nodes::Attribute) {}

    #[inline]
    fn visit_attr_pair(&mut self, node: &crate::ast::nodes::AttrPair) {}

    #[inline]
    fn visit_expr(&mut self, node: &crate::ast::nodes::Expr) {}

    #[inline]
    fn visit_generic_type(&mut self, node: &crate::ast::nodes::GenericType) {}

    #[inline]
    fn visit_tuple_type(&mut self, node: &crate::ast::nodes::TupleType) {}

    #[inline]
    fn visit_function_type(&mut self, node: &crate::ast::nodes::FunctionType) {}

    #[inline]
    fn visit_generic_arg(&mut self, node: &crate::ast::nodes::GenericArg) {}

    #[inline]
    fn visit_tuple_type_elem(&mut self, node: &crate::ast::nodes::TupleTypeElem) {}

    #[inline]
    fn visit_function_type_args(&mut self, node: &crate::ast::nodes::FunctionTypeArgs) {}

    #[inline]
    fn visit_function_return_type(&mut self, node: &crate::ast::nodes::FunctionReturnType) {}

    #[inline]
    fn visit_function_type_arg(&mut self, node: &crate::ast::nodes::FunctionTypeArg) {}

    #[inline]
    fn visit_stmt(&mut self, node: &crate::ast::nodes::Stmt) {}

    #[inline]
    fn visit_expr_stmt(&mut self, node: &crate::ast::nodes::ExprStmt) {}

    #[inline]
    fn visit_var_decl(&mut self, node: &crate::ast::nodes::VarDecl) {}

    #[inline]
    fn visit_if_stmt(&mut self, node: &crate::ast::nodes::IfStmt) {}

    #[inline]
    fn visit_if_block(&mut self, node: &crate::ast::nodes::IfBlock) {}

    #[inline]
    fn visit_else_block(&mut self, node: &crate::ast::nodes::ElseBlock) {}

    #[inline]
    fn visit_var_ref(&mut self, node: &crate::ast::nodes::VarRef) {}

    #[inline]
    fn visit_tuple_pattern(&mut self, node: &crate::ast::nodes::TuplePattern) {}

    #[inline]
    fn visit_tuple_pattern_elem(&mut self, node: &crate::ast::nodes::TuplePatternElem) {}

    #[inline]
    fn visit_literal(&mut self, node: &crate::ast::nodes::Literal) {}

    #[inline]
    fn visit_assign(&mut self, node: &crate::ast::nodes::Assign) {}

    #[inline]
    fn visit_paren_expr(&mut self, node: &crate::ast::nodes::ParenExpr) {}

    #[inline]
    fn visit_bin_expr(&mut self, node: &crate::ast::nodes::BinExpr) {}

    #[inline]
    fn visit_ret_expr(&mut self, node: &crate::ast::nodes::RetExpr) {}

    #[inline]
    fn visit_unary_expr(&mut self, node: &crate::ast::nodes::UnaryExpr) {}

    #[inline]
    fn visit_bool(&mut self, node: &crate::ast::nodes::Bool) {}

    #[inline]
    fn visit_number(&mut self, node: &crate::ast::nodes::Number) {}

    #[inline]
    fn visit_string(&mut self, node: &crate::ast::nodes::String) {}
}
