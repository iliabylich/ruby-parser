use super::*;

impl<'a, Builder> Parser<'a, Builder>
where
    Builder: Constructor,
{
    pub(crate) fn parse_top_compstmt(&mut self) -> Node<'a> {
        let top_stmts = self.parse_top_stmts();
        self.parse_opt_terms();
        todo!("builder.compstmt {:?}", top_stmts)
    }

    pub(crate) fn parse_top_stmts(&mut self) -> Vec<Node<'a>> {
        let mut top_stmts = vec![];
        while let Some(top_stmt) = self.parse_top_stmt() {
            top_stmts.push(*top_stmt);
        }
        top_stmts
    }

    pub(crate) fn parse_top_stmt(&mut self) -> Option<Box<Node<'a>>> {
        if let Some(begin_block) = self.parse_preexe() {
            return Some(begin_block);
        }
        self.parse_stmt()
    }

    pub(crate) fn parse_bodystmt(&mut self) -> Node<'a> {
        todo!()
    }

    pub(crate) fn parse_compstmt(&mut self) -> Option<Box<Node<'a>>> {
        let stmts = self.parse_stmts();
        self.parse_opt_terms();
        todo!("compstmt({:?})", stmts)
    }

    pub(crate) fn parse_stmts(&mut self) -> Vec<Node<'a>> {
        let mut stmts = vec![];
        while let Some(stmt) = self.parse_stmt() {
            stmts.push(*stmt);
        }

        if let Some(begin_block) = self.parse_preexe() {
            stmts.push(*begin_block);
        }
        stmts
    }

    pub(crate) fn parse_stmt(&mut self) -> Option<Box<Node<'a>>> {
        let stmt = self.parse_stmt_head()?;

        match self.current_token().value() {
            TokenValue::kIF => {
                let k_if = self.take_token();
                let expr_value = self.parse_expr_value();
                panic!("if_mod {:?} {:?} {:?}", stmt, k_if, expr_value);
            }
            TokenValue::kUNLESS => {
                let k_unless = self.take_token();
                let expr_value = self.parse_expr_value();
                panic!("unless_mod {:?} {:?} {:?}", stmt, k_unless, expr_value);
            }
            TokenValue::kWHILE => {
                let k_while = self.take_token();
                let expr_value = self.parse_expr_value();
                panic!("while_mod {:?} {:?} {:?}", stmt, k_while, expr_value);
            }
            TokenValue::kUNTIL => {
                let k_until = self.take_token();
                let expr_value = self.parse_expr_value();
                panic!("until_mod {:?} {:?} {:?}", stmt, k_until, expr_value);
            }
            _ => Some(stmt),
        }
    }

    pub(crate) fn parse_stmt_head(&mut self) -> Option<Box<Node<'a>>> {
        if let Some(alias) = self.parse_alias() {
            Some(alias)
        } else if self.current_token().value() == &TokenValue::kUNDEF {
            Some(self.parse_undef())
        } else if self.current_token().value() == &TokenValue::klEND {
            Some(self.parse_postexe())
        } else if let Some(command_asgn) = self.parse_command_asgn() {
            Some(command_asgn)
        } else if let Some(mlhs) = self.parse_mlhs() {
            let eql = self.expect_token(TokenValue::tEQL);
            if let Some(command_call) = self.parse_command_call() {
                panic!("multi_assign({:?}, {:?}, {:?})", mlhs, eql, command_call);
            } else if let Some(mrhs) = self.parse_mrhs() {
                panic!("assign({:?}, {:?}, array({:?}))", mlhs, eql, mrhs)
            } else if let Some(mrhs_arg) = self.parse_mrhs_arg() {
                let rescue_mod = self.expect_token(TokenValue::kRESCUE);
                let stmt = self.parse_stmt().expect("expected stmt");
                panic!(
                    "multi_assign({:?} {:?} {:?} {:?} {:?})",
                    mlhs, eql, mrhs_arg, rescue_mod, stmt
                )
            } else {
                panic!(
                    "expected command_call/mrhs/mrhs_arg, got {:?}",
                    self.current_token()
                )
            }
        } else if let Some(expr) = self.parse_expr() {
            Some(expr)
        } else {
            panic!("expected stmt, got {:?}", self.current_token())
        }
    }
}
