use crate::{
    builder::{Builder, Constructor},
    parser::{mlhs, ParseError, ParseResultApi, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_top_compstmt(&mut self) -> Result<Option<Box<Node>>, ParseError> {
        let top_stmts = self.parse_top_stmts();
        self.parse_opt_terms();
        if top_stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::<C>::compstmt(top_stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn parse_top_stmts(&mut self) -> Vec<Node> {
        let mut top_stmts = vec![];
        while let Ok(top_stmt) = self.try_top_stmt() {
            top_stmts.push(*top_stmt);
        }
        top_stmts
    }

    pub(crate) fn try_top_stmt(&mut self) -> Result<Box<Node>, ParseError> {
        self.one_of("top-level statement")
            .or_else(|| self.try_preexe())
            .or_else(|| self.try_stmt())
            .done()
    }

    pub(crate) fn try_bodystmt(&mut self) -> Result<Box<Node>, ParseError> {
        let compstmt = self.try_compstmt()?;
        let rescue_bodies = self.try_opt_rescue()?;
        let opt_else = self.try_opt_else()?;
        let opt_ensure = self.try_opt_ensure()?;
        Ok(Builder::<C>::begin_body(
            compstmt,
            rescue_bodies,
            Some(opt_else),
            Some(opt_ensure),
        ))
    }

    pub(crate) fn try_compstmt(&mut self) -> Result<Option<Box<Node>>, ParseError> {
        let stmts = self.parse_stmts();
        self.parse_opt_terms();
        if stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::<C>::compstmt(stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn parse_stmts(&mut self) -> Vec<Node> {
        let mut stmts = vec![];
        while let Ok(stmt) = self.try_stmt() {
            stmts.push(*stmt);
        }

        if let Ok(begin_block) = self.try_preexe() {
            stmts.push(*begin_block);
        }
        stmts
    }

    pub(crate) fn try_stmt(&mut self) -> Result<Box<Node>, ParseError> {
        let stmt = self.try_stmt_head()?;

        match self.current_token().kind() {
            TokenKind::kIF => {
                let k_if = self.take_token();
                let expr_value = self.try_expr_value()?;
                panic!("if_mod {:?} {:?} {:?}", stmt, k_if, expr_value);
            }
            TokenKind::kUNLESS => {
                let k_unless = self.take_token();
                let expr_value = self.try_expr_value()?;
                panic!("unless_mod {:?} {:?} {:?}", stmt, k_unless, expr_value);
            }
            TokenKind::kWHILE => {
                let k_while = self.take_token();
                let expr_value = self.try_expr_value()?;
                panic!("while_mod {:?} {:?} {:?}", stmt, k_while, expr_value);
            }
            TokenKind::kUNTIL => {
                let k_until = self.take_token();
                let expr_value = self.try_expr_value()?;
                panic!("until_mod {:?} {:?} {:?}", stmt, k_until, expr_value);
            }
            _ => Ok(stmt),
        }
    }

    fn try_stmt_head(&mut self) -> Result<Box<Node>, ParseError> {
        if let Ok(alias) = self.try_alias() {
            return Ok(alias);
        } else if let Ok(undef) = self.try_undef() {
            return Ok(undef);
        } else if let Ok(postexe) = self.try_postexe() {
            return Ok(postexe);
        } else if self.current_token().is(TokenKind::kDEF) {
            todo!("handle endless def")
        } else if let Ok(assignment) = self.try_assignment() {
            return Ok(assignment);
        }

        self.try_expr()
    }

    fn try_assignment(&mut self) -> Result<Box<Node>, ParseError> {
        let checkpoint = self.new_checkpoint();

        match self.parse_mlhs().ignore_lookahead_errors()? {
            Some(mlhs::MLHS::DefinitelyMlhs { node: mlhs }) => {
                // definitely an MLHS, can only be assigned via `=`
                let eql_t = self.expect_token(TokenKind::tEQL);
                if let Ok(command_call) = self.try_command_call() {
                    todo!("mlhs = rhs {:?} {:?} {:?}", mlhs, eql_t, command_call);
                } else if let Ok(mrhs_arg) = self.try_mrhs_arg() {
                    if let Ok(rescue_t) = self.try_token(TokenKind::kRESCUE) {
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
            Some(mlhs::MLHS::MaybeLhs { node: lhs }) => {
                // maybe a plain assignment,
                // but maybe just an expression (that is fully parsed later in `parse_expr`)
                match self.current_token().kind() {
                    TokenKind::tEQL | TokenKind::tOP_ASGN => {
                        // definitely an assignment
                        let op_t = self.take_token();
                        let command_rhs = self.try_command_rhs().expect("assignment must have RHS");
                        todo!("assignment {:?} {:?} {:?}", lhs, op_t, command_rhs);
                    }
                    _ => {
                        // rollback, expr can be more that just an lvar get
                        checkpoint.restore();
                        Err(ParseError::empty())
                    }
                }
            }
            None => {
                // well, it's not an MLHS, then it's definitely an expression
                Err(ParseError::empty())
            }
        }
    }
}
