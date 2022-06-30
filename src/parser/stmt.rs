use super::*;

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn parse_top_compstmt(&mut self) -> Option<Box<Node<'a>>> {
        let top_stmts = self.parse_top_stmts();
        self.parse_opt_terms();
        Builder::<C>::compstmt(top_stmts)
    }

    pub(crate) fn parse_top_stmts(&mut self) -> Vec<Node<'a>> {
        let mut top_stmts = vec![];
        while let Some(top_stmt) = self.try_top_stmt() {
            top_stmts.push(*top_stmt);
        }
        top_stmts
    }

    pub(crate) fn try_top_stmt(&mut self) -> Option<Box<Node<'a>>> {
        None.or_else(|| self.parse_preexe())
            .or_else(|| self.parse_stmt())
    }

    pub(crate) fn parse_bodystmt(&mut self) -> Node<'a> {
        todo!()
    }

    pub(crate) fn parse_compstmt(&mut self) -> Option<Box<Node<'a>>> {
        let stmts = self.parse_stmts();
        self.parse_opt_terms();
        Builder::<C>::compstmt(stmts)
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
            return Some(alias);
        } else if let Some(undef) = self.parse_undef() {
            return Some(undef);
        } else if let Some(postexe) = self.parse_postexe() {
            return Some(postexe);
        } else if matches!(self.current_token().value(), TokenValue::kDEF) {
            todo!("handle endless def")
        }

        match self.parse_mlhs() {
            mlhs::MLHS::DefinitelyMlhs { node: mlhs } => {
                // definitely an MLHS, can only be assigned via `=`
                let eql_t = self.expect_token(TokenValue::tEQL);
                if let Some(command_call) = self.parse_command_call() {
                    todo!("mlhs = rhs {:?} {:?} {:?}", mlhs, eql_t, command_call);
                } else if let Some(mrhs_arg) = self.parse_mrhs_arg() {
                    if let Some(rescue_t) = self.try_token(TokenValue::kRESCUE) {
                        let stmt = self.parse_stmt().expect("mlhs -> kRESCUE requires stmt");
                        todo!(
                            "mlhs = rhs rescue stmt {:?} {:?} {:?} {:?} {:?}",
                            mlhs,
                            eql_t,
                            mrhs_arg,
                            rescue_t,
                            stmt
                        )
                    }
                }
            }
            mlhs::MLHS::MaybeLhs { node: lhs } => {
                // maybe a plain assignment, maybe an expression (that is fully parsed in `parse_expr`)
                match self.current_token().value() {
                    TokenValue::tEQL | TokenValue::tOP_ASGN => {
                        // definitely an assignment
                        let op_t = self.take_token();
                        let command_rhs =
                            self.parse_command_rhs().expect("assignment must have RHS");
                        todo!("assignment {:?} {:?} {:?}", lhs, op_t, command_rhs);
                    }
                    _ => {
                        todo!("rollback, expr can be more that just an lvar get");
                    }
                }
            }
            mlhs::MLHS::None => {
                // well, it's not an MLHS, then it's definitely an expression
            }
        }

        self.parse_expr()
    }
}
