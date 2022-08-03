use crate::{
    builder::Builder,
    parser::{ParseError, ParseResult, Parser},
    token::TokenKind,
    Node, Token,
};

impl Parser {
    pub(crate) fn try_top_compstmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let (top_stmts, _opt_terms) = self
            .all_of("top_compstmt")
            .and(|| self.try_top_stmts())
            .and(|| self.try_opt_terms())
            .stop()?;

        if top_stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::compstmt(top_stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn try_top_stmts(&mut self) -> ParseResult<Vec<Node>> {
        let mut top_stmts = vec![];
        loop {
            match self.try_top_stmt() {
                Ok(top_stmt) => top_stmts.push(*top_stmt),
                Err(error) => {
                    match error.strip_lookaheads() {
                        None => {
                            // no match
                            break;
                        }
                        Some(error) => {
                            return Err(ParseError::seq_error::<Vec<Node>, _>(
                                "top stmts",
                                top_stmts,
                                error,
                            ));
                        }
                    }
                }
            }
        }
        Ok(top_stmts)
    }

    pub(crate) fn try_top_stmt(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("top-level statement")
            .or_else(|| self.try_preexe())
            .or_else(|| self.try_stmt())
            .stop()
    }

    pub(crate) fn try_bodystmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let (compstmt, rescue_bodies, opt_else, opt_ensure) = self
            .all_of("bodystmt")
            .and(|| self.try_compstmt())
            .and(|| self.try_opt_rescue())
            .and(|| self.try_opt_else())
            .and(|| self.try_opt_ensure())
            .stop()?;

        Ok(Builder::begin_body(
            compstmt,
            rescue_bodies,
            Some(opt_else),
            Some(opt_ensure),
        ))
    }

    pub(crate) fn try_compstmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let (stmts, _opt_terms) = self
            .all_of("compstmt")
            .and(|| self.try_stmts())
            .and(|| self.try_opt_terms())
            .stop()?;
        if stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::compstmt(stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn try_stmts(&mut self) -> ParseResult<Vec<Node>> {
        let mut stmts = vec![];
        while let Ok(stmt) = self.try_stmt() {
            stmts.push(*stmt);
        }

        if let Ok(begin_block) = self.try_preexe() {
            stmts.push(*begin_block);
        }
        Ok(stmts)
    }

    #[allow(unreachable_code, unused_mut)]
    pub(crate) fn try_stmt(&mut self) -> ParseResult<Box<Node>> {
        let mut stmt = self.try_stmt_head()?;
        match self.try_stmt_tail() {
            Ok((mod_t, expr)) => match mod_t.kind {
                TokenKind::kIF => todo!("{:?} {:?} {:?}", stmt, mod_t, expr),
                TokenKind::kUNLESS => todo!("{:?} {:?} {:?}", stmt, mod_t, expr),
                TokenKind::kWHILE => todo!("{:?} {:?} {:?}", stmt, mod_t, expr),
                TokenKind::kUNTIL => todo!("{:?} {:?} {:?}", stmt, mod_t, expr),
                _ => unreachable!("stmt_tail handles only if/unless/while/until modifiers"),
            },
            Err(error) => {
                match error.strip_lookaheads() {
                    None => {
                        // ignore
                        Ok(stmt)
                    }
                    Some(error) => Err(ParseError::seq_error::<Box<Node>, _>(
                        "stmt tail",
                        stmt,
                        error,
                    )),
                }
            }
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

    fn try_stmt_tail(&mut self) -> ParseResult<(Token, Box<Node>)> {
        self.one_of("stmt tail")
            .or_else(|| {
                self.all_of("if_mod expr")
                    .and(|| self.try_token(TokenKind::kIF))
                    .and(|| self.try_expr_value())
                    .stop()
            })
            .or_else(|| {
                self.all_of("unless_mod expr")
                    .and(|| self.try_token(TokenKind::kUNLESS))
                    .and(|| self.try_expr_value())
                    .stop()
            })
            .or_else(|| {
                self.all_of("while_mod expr")
                    .and(|| self.try_token(TokenKind::kWHILE))
                    .and(|| self.try_expr_value())
                    .stop()
            })
            .or_else(|| {
                self.all_of("until_mod expr")
                    .and(|| self.try_token(TokenKind::kUNTIL))
                    .and(|| self.try_expr_value())
                    .stop()
            })
            .stop()
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
                                    .stop()?;
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
                                    .stop()?;
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
                            .stop()
                    })
                    .and(|| self.expect_token(TokenKind::tOP_ASGN))
                    .and(|| self.try_command_rhs())
                    .stop()?;

                todo!("{:?} {:?} {:?}", lhs, op_t, rhs)
            })
            .stop()
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
                                    .stop()?;
                                #[allow(unreachable_code)]
                                Ok(todo!("{:?}", maybe_rescut_stmt) as Box<Node>)
                            })
                            .stop()
                            .map(|(value, rescue)| todo!("{:?} {:?}", value, rescue))
                    })
                    .stop()
            })
            .stop()?;
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
                    .stop()
            })
            .stop()?;

        todo!("{:?} {:?} {:?}", lhs, eql_t, rhs)
    }
}
