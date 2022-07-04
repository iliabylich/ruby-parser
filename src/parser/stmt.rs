use crate::{
    builder::{Builder, Constructor},
    parser::{mlhs, Parser},
    token::TokenValue,
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_top_compstmt(&mut self) -> Option<Box<Node<'a>>> {
        let top_stmts = self.parse_top_stmts();
        self.parse_opt_terms();
        if top_stmts.is_empty() {
            None
        } else {
            Some(Builder::<C>::compstmt(top_stmts))
        }
    }

    pub(crate) fn parse_top_stmts(&mut self) -> Vec<Node<'a>> {
        let mut top_stmts = vec![];
        while let Some(top_stmt) = self.try_top_stmt() {
            top_stmts.push(*top_stmt);
        }
        top_stmts
    }

    pub(crate) fn try_top_stmt(&mut self) -> Option<Box<Node<'a>>> {
        None.or_else(|| self.try_preexe())
            .or_else(|| self.try_stmt())
    }

    pub(crate) fn try_bodystmt(&mut self) -> Option<Box<Node<'a>>> {
        let compstmt = self.try_compstmt()?;
        let rescue_bodies = self.parse_opt_rescue();
        let opt_else = self.try_opt_else();
        let opt_ensure = self.try_opt_ensure();
        Some(Builder::<C>::begin_body(
            compstmt,
            rescue_bodies,
            opt_else,
            opt_ensure,
        ))
    }

    pub(crate) fn try_compstmt(&mut self) -> Option<Box<Node<'a>>> {
        let stmts = self.parse_stmts();
        self.parse_opt_terms();
        if stmts.is_empty() {
            None
        } else {
            Some(Builder::<C>::compstmt(stmts))
        }
    }

    pub(crate) fn parse_stmts(&mut self) -> Vec<Node<'a>> {
        let mut stmts = vec![];
        while let Some(stmt) = self.try_stmt() {
            stmts.push(*stmt);
        }

        if let Some(begin_block) = self.try_preexe() {
            stmts.push(*begin_block);
        }
        stmts
    }

    pub(crate) fn try_stmt(&mut self) -> Option<Box<Node<'a>>> {
        let stmt = self.try_stmt_head()?;

        match self.current_token().value() {
            TokenValue::kIF => {
                let k_if = self.take_token();
                let expr_value = self.try_expr_value();
                panic!("if_mod {:?} {:?} {:?}", stmt, k_if, expr_value);
            }
            TokenValue::kUNLESS => {
                let k_unless = self.take_token();
                let expr_value = self.try_expr_value();
                panic!("unless_mod {:?} {:?} {:?}", stmt, k_unless, expr_value);
            }
            TokenValue::kWHILE => {
                let k_while = self.take_token();
                let expr_value = self.try_expr_value();
                panic!("while_mod {:?} {:?} {:?}", stmt, k_while, expr_value);
            }
            TokenValue::kUNTIL => {
                let k_until = self.take_token();
                let expr_value = self.try_expr_value();
                panic!("until_mod {:?} {:?} {:?}", stmt, k_until, expr_value);
            }
            _ => Some(stmt),
        }
    }

    fn try_stmt_head(&mut self) -> Option<Box<Node<'a>>> {
        if let Some(alias) = self.try_alias() {
            return Some(alias);
        } else if let Some(undef) = self.try_undef() {
            return Some(undef);
        } else if let Some(postexe) = self.try_postexe() {
            return Some(postexe);
        } else if self.current_token().is(TokenValue::kDEF) {
            todo!("handle endless def")
        } else if let Some(assignment) = self.try_assignment() {
            return Some(assignment);
        }

        self.try_expr()
    }

    fn try_assignment(&mut self) -> Option<Box<Node<'a>>> {
        let checkpoint = self.new_checkpoint();

        match self.parse_mlhs() {
            mlhs::MLHS::DefinitelyMlhs { node: mlhs } => {
                // definitely an MLHS, can only be assigned via `=`
                let eql_t = self.expect_token(TokenValue::tEQL);
                if let Some(command_call) = self.try_command_call() {
                    todo!("mlhs = rhs {:?} {:?} {:?}", mlhs, eql_t, command_call);
                } else if let Some(mrhs_arg) = self.try_mrhs_arg() {
                    if let Some(rescue_t) = self.try_token(TokenValue::kRESCUE) {
                        let stmt = self.try_stmt().expect("mlhs -> kRESCUE requires stmt");
                        todo!(
                            "mlhs = rhs rescue stmt {:?} {:?} {:?} {:?} {:?}",
                            mlhs,
                            eql_t,
                            mrhs_arg,
                            rescue_t,
                            stmt
                        )
                    } else {
                        todo!("mlhs = rhs {:?} {:?} {:?}", mlhs, eql_t, mrhs_arg)
                    }
                } else {
                    todo!("mlhs -> tEQL requires rhs")
                }
            }
            mlhs::MLHS::MaybeLhs { node: lhs } => {
                // maybe a plain assignment,
                // but maybe just an expression (that is fully parsed later in `parse_expr`)
                match self.current_token().value() {
                    TokenValue::tEQL | TokenValue::tOP_ASGN => {
                        // definitely an assignment
                        let op_t = self.take_token();
                        let command_rhs = self.try_command_rhs().expect("assignment must have RHS");
                        todo!("assignment {:?} {:?} {:?}", lhs, op_t, command_rhs);
                    }
                    _ => {
                        // rollback, expr can be more that just an lvar get
                        self.restore_checkpoint(checkpoint);
                        None
                    }
                }
            }
            mlhs::MLHS::None => {
                // well, it's not an MLHS, then it's definitely an expression
                None
            }
        }
    }
}
