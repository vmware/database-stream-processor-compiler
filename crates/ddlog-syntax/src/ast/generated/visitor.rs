pub trait AstVisitor {
    #[inline]
    #[doc(hidden)]
    fn default_root(&mut self, node: &crate::ast::nodes::Root) {
        for node in node.root() {
            self.visit_root(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_root(&mut self, node: &crate::ast::nodes::Root) {
        self.default_root(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_item(&mut self, node: &crate::ast::nodes::Item) {}
    #[inline]
    fn visit_item(&mut self, node: &crate::ast::nodes::Item) {
        self.default_item(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_function_def(&mut self, node: &crate::ast::nodes::FunctionDef) {
        if let ::core::option::Option::Some(node) = node.attributes() {
            self.visit_attributes(&*node)
        }
        if let ::core::option::Option::Some(node) = node.modifiers() {
            self.visit_modifiers(&*node)
        }
        if let ::core::option::Option::Some(node) = node.generics() {
            self.visit_generics(&*node)
        }
        if let ::core::option::Option::Some(node) = node.function_args() {
            self.visit_function_args(&*node)
        }
        if let ::core::option::Option::Some(node) = node.ret() {
            self.visit_ret(&*node)
        }
        if let ::core::option::Option::Some(node) = node.block() {
            self.visit_block(&*node)
        }
    }
    #[inline]
    fn visit_function_def(&mut self, node: &crate::ast::nodes::FunctionDef) {
        self.default_function_def(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_relation_def(&mut self, node: &crate::ast::nodes::RelationDef) {
        if let ::core::option::Option::Some(node) = node.attributes() {
            self.visit_attributes(&*node)
        }
        if let ::core::option::Option::Some(node) = node.modifiers() {
            self.visit_modifiers(&*node)
        }
        if let ::core::option::Option::Some(node) = node.rel_kw() {
            self.visit_rel_kw(&*node)
        }
        if let ::core::option::Option::Some(node) = node.rel_cols() {
            self.visit_rel_cols(&*node)
        }
    }
    #[inline]
    fn visit_relation_def(&mut self, node: &crate::ast::nodes::RelationDef) {
        self.default_relation_def(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_type_def(&mut self, node: &crate::ast::nodes::TypeDef) {
        if let ::core::option::Option::Some(node) = node.attributes() {
            self.visit_attributes(&*node)
        }
        if let ::core::option::Option::Some(node) = node.modifiers() {
            self.visit_modifiers(&*node)
        }
        if let ::core::option::Option::Some(node) = node.type_body() {
            self.visit_type_body(&*node)
        }
    }
    #[inline]
    fn visit_type_def(&mut self, node: &crate::ast::nodes::TypeDef) {
        self.default_type_def(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_attributes(&mut self, node: &crate::ast::nodes::Attributes) {
        for node in node.attributes() {
            self.visit_attribute(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_attributes(&mut self, node: &crate::ast::nodes::Attributes) {
        self.default_attributes(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_modifiers(&mut self, node: &crate::ast::nodes::Modifiers) {
        for node in node.modifiers() {
            self.visit_modifier(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_modifiers(&mut self, node: &crate::ast::nodes::Modifiers) {
        self.default_modifiers(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_rel_cols(&mut self, node: &crate::ast::nodes::RelCols) {
        for node in node.columns() {
            self.visit_column(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_rel_cols(&mut self, node: &crate::ast::nodes::RelCols) {
        self.default_rel_cols(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_rel_col(&mut self, node: &crate::ast::nodes::RelCol) {
        if let ::core::option::Option::Some(node) = node.attributes() {
            self.visit_attributes(&*node)
        }
        if let ::core::option::Option::Some(node) = node.pattern() {
            self.visit_pattern(&*node)
        }
        if let ::core::option::Option::Some(node) = node.type_token() {
            self.visit_type_token(&*node)
        }
    }
    #[inline]
    fn visit_rel_col(&mut self, node: &crate::ast::nodes::RelCol) {
        self.default_rel_col(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_pattern(&mut self, node: &crate::ast::nodes::Pattern) {}
    #[inline]
    fn visit_pattern(&mut self, node: &crate::ast::nodes::Pattern) {
        self.default_pattern(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_type_token(&mut self, node: &crate::ast::nodes::Type) {}
    #[inline]
    fn visit_type_token(&mut self, node: &crate::ast::nodes::Type) {
        self.default_type_token(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_generics(&mut self, node: &crate::ast::nodes::Generics) {
        for node in node.generics() {
            self.visit_generic(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_generics(&mut self, node: &crate::ast::nodes::Generics) {
        self.default_generics(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_function_args(&mut self, node: &crate::ast::nodes::FunctionArgs) {
        for node in node.args() {
            self.visit_arg(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_function_args(&mut self, node: &crate::ast::nodes::FunctionArgs) {
        self.default_function_args(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_function_return(&mut self, node: &crate::ast::nodes::FunctionReturn) {
        if let ::core::option::Option::Some(node) = node.type_token() {
            self.visit_type_token(&*node)
        }
    }
    #[inline]
    fn visit_function_return(&mut self, node: &crate::ast::nodes::FunctionReturn) {
        self.default_function_return(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_block(&mut self, node: &crate::ast::nodes::Block) {
        for node in node.statements() {
            self.visit_statement(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_block(&mut self, node: &crate::ast::nodes::Block) {
        self.default_block(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_function_arg(&mut self, node: &crate::ast::nodes::FunctionArg) {
        if let ::core::option::Option::Some(node) = node.attributes() {
            self.visit_attributes(&*node)
        }
        if let ::core::option::Option::Some(node) = node.pattern() {
            self.visit_pattern(&*node)
        }
        if let ::core::option::Option::Some(node) = node.type_token() {
            self.visit_type_token(&*node)
        }
    }
    #[inline]
    fn visit_function_arg(&mut self, node: &crate::ast::nodes::FunctionArg) {
        self.default_function_arg(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_type_body(&mut self, node: &crate::ast::nodes::TypeBody) {}
    #[inline]
    fn visit_type_body(&mut self, node: &crate::ast::nodes::TypeBody) {
        self.default_type_body(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_record_type(&mut self, node: &crate::ast::nodes::RecordType) {
        if let ::core::option::Option::Some(node) = node.err() {
            self.visit_err(&*node)
        }
    }
    #[inline]
    fn visit_record_type(&mut self, node: &crate::ast::nodes::RecordType) {
        self.default_record_type(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_sum_type(&mut self, node: &crate::ast::nodes::SumType) {
        for node in node.sum_type() {
            self.visit_sum_type(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_sum_type(&mut self, node: &crate::ast::nodes::SumType) {
        self.default_sum_type(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_record_field(&mut self, node: &crate::ast::nodes::RecordField) {
        if let ::core::option::Option::Some(node) = node.attributes() {
            self.visit_attributes(&*node)
        }
        if let ::core::option::Option::Some(node) = node.pattern() {
            self.visit_pattern(&*node)
        }
        if let ::core::option::Option::Some(node) = node.type_token() {
            self.visit_type_token(&*node)
        }
    }
    #[inline]
    fn visit_record_field(&mut self, node: &crate::ast::nodes::RecordField) {
        self.default_record_field(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_attribute(&mut self, node: &crate::ast::nodes::Attribute) {
        for node in node.AttrPair() {
            self.visit_AttrPair(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_attribute(&mut self, node: &crate::ast::nodes::Attribute) {
        self.default_attribute(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_attr_pair(&mut self, node: &crate::ast::nodes::AttrPair) {
        if let ::core::option::Option::Some(node) = node.expr() {
            self.visit_expr(&*node)
        }
    }
    #[inline]
    fn visit_attr_pair(&mut self, node: &crate::ast::nodes::AttrPair) {
        self.default_attr_pair(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_expr(&mut self, node: &crate::ast::nodes::Expr) {}
    #[inline]
    fn visit_expr(&mut self, node: &crate::ast::nodes::Expr) {
        self.default_expr(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_generic_type(&mut self, node: &crate::ast::nodes::GenericType) {
        if let ::core::option::Option::Some(node) = node.Generics() {
            self.visit_Generics(&*node)
        }
    }
    #[inline]
    fn visit_generic_type(&mut self, node: &crate::ast::nodes::GenericType) {
        self.default_generic_type(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_tuple_type(&mut self, node: &crate::ast::nodes::TupleType) {
        for node in node.elements() {
            self.visit_element(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_tuple_type(&mut self, node: &crate::ast::nodes::TupleType) {
        self.default_tuple_type(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_function_type(&mut self, node: &crate::ast::nodes::FunctionType) {
        if let ::core::option::Option::Some(node) = node.function_type_args() {
            self.visit_function_type_args(&*node)
        }
        if let ::core::option::Option::Some(node) = node.ret() {
            self.visit_ret(&*node)
        }
    }
    #[inline]
    fn visit_function_type(&mut self, node: &crate::ast::nodes::FunctionType) {
        self.default_function_type(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_generic_arg(&mut self, node: &crate::ast::nodes::GenericArg) {
        if let ::core::option::Option::Some(node) = node.type_token() {
            self.visit_type_token(&*node)
        }
    }
    #[inline]
    fn visit_generic_arg(&mut self, node: &crate::ast::nodes::GenericArg) {
        self.default_generic_arg(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_tuple_type_elem(&mut self, node: &crate::ast::nodes::TupleTypeElem) {
        if let ::core::option::Option::Some(node) = node.type_token() {
            self.visit_type_token(&*node)
        }
    }
    #[inline]
    fn visit_tuple_type_elem(&mut self, node: &crate::ast::nodes::TupleTypeElem) {
        self.default_tuple_type_elem(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_function_type_args(&mut self, node: &crate::ast::nodes::FunctionTypeArgs) {
        for node in node.args() {
            self.visit_arg(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_function_type_args(&mut self, node: &crate::ast::nodes::FunctionTypeArgs) {
        self.default_function_type_args(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_function_return_type(&mut self, node: &crate::ast::nodes::FunctionReturnType) {
        if let ::core::option::Option::Some(node) = node.type_token() {
            self.visit_type_token(&*node)
        }
    }
    #[inline]
    fn visit_function_return_type(&mut self, node: &crate::ast::nodes::FunctionReturnType) {
        self.default_function_return_type(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_function_type_arg(&mut self, node: &crate::ast::nodes::FunctionTypeArg) {
        if let ::core::option::Option::Some(node) = node.type_token() {
            self.visit_type_token(&*node)
        }
    }
    #[inline]
    fn visit_function_type_arg(&mut self, node: &crate::ast::nodes::FunctionTypeArg) {
        self.default_function_type_arg(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_stmt(&mut self, node: &crate::ast::nodes::Stmt) {}
    #[inline]
    fn visit_stmt(&mut self, node: &crate::ast::nodes::Stmt) {
        self.default_stmt(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_expr_stmt(&mut self, node: &crate::ast::nodes::ExprStmt) {
        if let ::core::option::Option::Some(node) = node.expr() {
            self.visit_expr(&*node)
        }
    }
    #[inline]
    fn visit_expr_stmt(&mut self, node: &crate::ast::nodes::ExprStmt) {
        self.default_expr_stmt(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_var_decl(&mut self, node: &crate::ast::nodes::VarDecl) {
        if let ::core::option::Option::Some(node) = node.pattern() {
            self.visit_pattern(&*node)
        }
        if let ::core::option::Option::Some(node) = node.expr() {
            self.visit_expr(&*node)
        }
    }
    #[inline]
    fn visit_var_decl(&mut self, node: &crate::ast::nodes::VarDecl) {
        self.default_var_decl(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_if_stmt(&mut self, node: &crate::ast::nodes::IfStmt) {
        for node in node.IfBlock() {
            self.visit_IfBlock(::core::ops::Deref::deref(&node));
        }
        if let ::core::option::Option::Some(node) = node.ElseBlock() {
            self.visit_ElseBlock(&*node)
        }
    }
    #[inline]
    fn visit_if_stmt(&mut self, node: &crate::ast::nodes::IfStmt) {
        self.default_if_stmt(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_if_block(&mut self, node: &crate::ast::nodes::IfBlock) {
        if let ::core::option::Option::Some(node) = node.expr() {
            self.visit_expr(&*node)
        }
        if let ::core::option::Option::Some(node) = node.block() {
            self.visit_block(&*node)
        }
    }
    #[inline]
    fn visit_if_block(&mut self, node: &crate::ast::nodes::IfBlock) {
        self.default_if_block(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_else_block(&mut self, node: &crate::ast::nodes::ElseBlock) {
        if let ::core::option::Option::Some(node) = node.block() {
            self.visit_block(&*node)
        }
    }
    #[inline]
    fn visit_else_block(&mut self, node: &crate::ast::nodes::ElseBlock) {
        self.default_else_block(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_tuple_pattern(&mut self, node: &crate::ast::nodes::TuplePattern) {
        for node in node.elements() {
            self.visit_element(::core::ops::Deref::deref(&node));
        }
    }
    #[inline]
    fn visit_tuple_pattern(&mut self, node: &crate::ast::nodes::TuplePattern) {
        self.default_tuple_pattern(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_tuple_pattern_elem(&mut self, node: &crate::ast::nodes::TuplePatternElem) {
        if let ::core::option::Option::Some(node) = node.pattern() {
            self.visit_pattern(&*node)
        }
    }
    #[inline]
    fn visit_tuple_pattern_elem(&mut self, node: &crate::ast::nodes::TuplePatternElem) {
        self.default_tuple_pattern_elem(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_literal(&mut self, node: &crate::ast::nodes::Literal) {}
    #[inline]
    fn visit_literal(&mut self, node: &crate::ast::nodes::Literal) {
        self.default_literal(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_assign(&mut self, node: &crate::ast::nodes::Assign) {
        if let ::core::option::Option::Some(node) = node.pattern() {
            self.visit_pattern(&*node)
        }
        if let ::core::option::Option::Some(node) = node.expr() {
            self.visit_expr(&*node)
        }
    }
    #[inline]
    fn visit_assign(&mut self, node: &crate::ast::nodes::Assign) {
        self.default_assign(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_paren_expr(&mut self, node: &crate::ast::nodes::ParenExpr) {
        if let ::core::option::Option::Some(node) = node.expr() {
            self.visit_expr(&*node)
        }
    }
    #[inline]
    fn visit_paren_expr(&mut self, node: &crate::ast::nodes::ParenExpr) {
        self.default_paren_expr(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_bin_expr(&mut self, node: &crate::ast::nodes::BinExpr) {
        if let ::core::option::Option::Some(node) = node.expr() {
            self.visit_expr(&*node)
        }
        if let ::core::option::Option::Some(node) = node.bin_op() {
            self.visit_bin_op(&*node)
        }
        if let ::core::option::Option::Some(node) = node.expr() {
            self.visit_expr(&*node)
        }
    }
    #[inline]
    fn visit_bin_expr(&mut self, node: &crate::ast::nodes::BinExpr) {
        self.default_bin_expr(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_ret_expr(&mut self, node: &crate::ast::nodes::RetExpr) {
        if let ::core::option::Option::Some(node) = node.expr() {
            self.visit_expr(&*node)
        }
    }
    #[inline]
    fn visit_ret_expr(&mut self, node: &crate::ast::nodes::RetExpr) {
        self.default_ret_expr(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_unary_expr(&mut self, node: &crate::ast::nodes::UnaryExpr) {
        if let ::core::option::Option::Some(node) = node.unary_op() {
            self.visit_unary_op(&*node)
        }
        if let ::core::option::Option::Some(node) = node.expr() {
            self.visit_expr(&*node)
        }
    }
    #[inline]
    fn visit_unary_expr(&mut self, node: &crate::ast::nodes::UnaryExpr) {
        self.default_unary_expr(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_bool(&mut self, node: &crate::ast::nodes::Bool) {}
    #[inline]
    fn visit_bool(&mut self, node: &crate::ast::nodes::Bool) {
        self.default_bool(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_unary_op(&mut self, node: &crate::ast::nodes::UnaryOp) {}
    #[inline]
    fn visit_unary_op(&mut self, node: &crate::ast::nodes::UnaryOp) {
        self.default_unary_op(node);
    }
    #[inline]
    #[doc(hidden)]
    fn default_bin_op(&mut self, node: &crate::ast::nodes::BinOp) {}
    #[inline]
    fn visit_bin_op(&mut self, node: &crate::ast::nodes::BinOp) {
        self.default_bin_op(node);
    }
}
