use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, Parser},
    token::TokenKind,
    Node, Token,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_top_compstmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let top_stmts = self.try_top_stmts();
        self.try_opt_terms();
        if top_stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::<C>::compstmt(top_stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn try_top_stmts(&mut self) -> Vec<Node> {
        let mut top_stmts = vec![];
        while let Ok(top_stmt) = self.try_top_stmt() {
            top_stmts.push(*top_stmt);
        }
        top_stmts
    }

    pub(crate) fn try_top_stmt(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("top-level statement")
            .or_else(|| self.try_preexe())
            .or_else(|| self.try_stmt())
            .unwrap()
    }

    pub(crate) fn try_bodystmt(&mut self) -> ParseResult<Box<Node>> {
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

    pub(crate) fn try_compstmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let stmts = self.try_stmts();
        self.try_opt_terms();
        if stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::<C>::compstmt(stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn try_stmts(&mut self) -> Vec<Node> {
        let mut stmts = vec![];
        while let Ok(stmt) = self.try_stmt() {
            stmts.push(*stmt);
        }

        if let Ok(begin_block) = self.try_preexe() {
            stmts.push(*begin_block);
        }
        stmts
    }

    pub(crate) fn try_stmt(&mut self) -> ParseResult<Box<Node>> {
        let stmt = self.try_stmt_head()?;

        match self.current_token().kind {
            TokenKind::kIF => {
                let k_if = self.current_token();
                self.skip_token();
                let expr_value = self.try_expr_value()?;
                panic!("if_mod {:?} {:?} {:?}", stmt, k_if, expr_value);
            }
            TokenKind::kUNLESS => {
                let k_unless = self.current_token();
                self.skip_token();
                let expr_value = self.try_expr_value()?;
                panic!("unless_mod {:?} {:?} {:?}", stmt, k_unless, expr_value);
            }
            TokenKind::kWHILE => {
                let k_while = self.current_token();
                self.skip_token();
                let expr_value = self.try_expr_value()?;
                panic!("while_mod {:?} {:?} {:?}", stmt, k_while, expr_value);
            }
            TokenKind::kUNTIL => {
                let k_until = self.current_token();
                self.skip_token();
                let expr_value = self.try_expr_value()?;
                panic!("until_mod {:?} {:?} {:?}", stmt, k_until, expr_value);
            }
            _ => Ok(stmt),
        }
    }

    fn try_stmt_head(&mut self) -> ParseResult<Box<Node>> {
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

    fn rescue_stmt(&mut self) -> ParseResult<(Token, Box<Node>)> {
        todo!()
    }

    fn try_assignment(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("assignment")
            .or_else(|| self.try_mass_assignment())
            .or_else(|| self.try_simple_assignment())
            .or_else(|| {
                let (lhs, op_t, rhs) = self
                    .all_of("operation assignment")
                    .and(|| {
                        self.one_of("operation assignment lhs")
                            .or_else(|| {
                                let (primary_value, op_t, id_t) = self
                                    .all_of("primary call_op2 tIDENTIFIER")
                                    .and(|| self.try_primary_value())
                                    .and(|| self.try_call_op2())
                                    .and(|| self.try_const_or_identifier())
                                    .unwrap()?;
                                panic!(
                                    "primary_value call_op tIDENT {:?} {:?} {:?}",
                                    primary_value, op_t, id_t
                                )
                            })
                            .or_else(|| {
                                let (primary_value, lbrack_t, opt_call_args, rbrack_t) = self
                                    .all_of("primary [ args ]")
                                    .and(|| self.try_primary_value())
                                    .and(|| self.expect_token(TokenKind::tLBRACK))
                                    .and(|| self.try_opt_call_args())
                                    .and(|| self.try_rparen())
                                    .unwrap()?;
                                todo!(
                                    "{:?} {:?} {:?} {:?}",
                                    primary_value,
                                    lbrack_t,
                                    opt_call_args,
                                    rbrack_t
                                )
                            })
                            .or_else(|| self.try_var_lhs())
                            .or_else(|| self.try_back_ref())
                            .unwrap()
                    })
                    .and(|| self.expect_token(TokenKind::tOP_ASGN))
                    .and(|| self.try_command_rhs())
                    .unwrap()?;

                todo!("{:?} {:?} {:?}", lhs, op_t, rhs)
            })
            .unwrap()
    }

    fn try_mass_assignment(&mut self) -> ParseResult<Box<Node>> {
        let (mlhs, eql_t, rhs) = self
            .all_of("mass-assignment")
            .and(|| self.try_mlhs())
            .and(|| self.expect_token(TokenKind::tEQL))
            .and(|| {
                self.one_of("mass-assginemtn rhs")
                    .or_else(|| self.try_command_call())
                    .or_else(|| {
                        self.all_of("mrhs_arg [rescue stmt]")
                            .and(|| self.try_mrhs_arg())
                            .and(|| {
                                let maybe_rescut_stmt = self
                                    .one_of("[rescue stmt]")
                                    .or_else(|| self.rescue_stmt().map(|data| Some(data)))
                                    .or_else(|| Ok(None))
                                    .unwrap()?;
                                #[allow(unreachable_code)]
                                Ok(todo!("{:?}", maybe_rescut_stmt) as Box<Node>)
                            })
                            .unwrap()
                            .map(|(value, rescue)| todo!("{:?} {:?}", value, rescue))
                    })
                    .unwrap()
            })
            .unwrap()?;
        todo!("{:?} {:?} {:?}", mlhs, eql_t, rhs)
    }

    fn try_simple_assignment(&mut self) -> ParseResult<Box<Node>> {
        let (lhs, eql_t, rhs) = self
            .all_of("simple assignment")
            .and(|| self.try_lhs())
            .and(|| self.expect_token(TokenKind::tEQL))
            .and(|| {
                self.one_of("simple assignment rhs")
                    .or_else(|| self.try_command_call())
                    .or_else(|| self.try_command_rhs())
                    .unwrap()
            })
            .unwrap()?;

        todo!("{:?} {:?} {:?}", lhs, eql_t, rhs)
    }
}
