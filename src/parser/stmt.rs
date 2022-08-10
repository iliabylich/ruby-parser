use crate::{
    builder::{Builder, LoopType},
    parser::{ParseError, ParseResult, Parser},
    token::TokenKind,
    Node, Token,
};

impl Parser {
    pub(crate) fn try_top_compstmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let (top_stmts, _opt_terms) = self
            .all_of("top_compstmt")
            .and(|| self.parse_top_stmts())
            .and(|| self.parse_opt_terms())
            .stop()?;

        if top_stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::compstmt(top_stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn parse_top_stmts(&mut self) -> ParseResult<Vec<Node>> {
        let mut top_stmts = vec![];
        loop {
            match self.parse_top_stmt() {
                Ok(top_stmt) => top_stmts.push(*top_stmt),
                Err(error) => {
                    match error.strip_lookaheads() {
                        None => {
                            // no match
                            break;
                        }
                        Some(error) => {
                            return Err(ParseError::seq_error("top stmts", top_stmts, error));
                        }
                    }
                }
            }
        }
        Ok(top_stmts)
    }

    pub(crate) fn parse_top_stmt(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("top-level statement")
            .or_else(|| self.parse_preexe())
            .or_else(|| self.parse_stmt())
            .stop()
    }

    pub(crate) fn try_bodystmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let (compstmt, rescue_bodies, opt_else, opt_ensure) = self
            .all_of("bodystmt")
            .and(|| self.try_compstmt())
            .and(|| self.parse_opt_rescue())
            .and(|| self.try_opt_else())
            .and(|| self.try_opt_ensure())
            .stop()?;

        if compstmt.is_none()
            && rescue_bodies.is_empty()
            && opt_else.is_none()
            && opt_ensure.is_none()
        {
            return Ok(None);
        }

        Ok(Some(Builder::begin_body(
            compstmt,
            rescue_bodies,
            opt_else,
            opt_ensure,
        )))
    }

    pub(crate) fn try_compstmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let (stmts, _opt_terms) = self
            .all_of("compstmt")
            .and(|| self.parse_stmts())
            .and(|| self.parse_opt_terms())
            .stop()?;
        if stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::compstmt(stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn parse_stmts(&mut self) -> ParseResult<Vec<Node>> {
        let mut stmts = vec![];
        let mut terms = vec![];

        match self.parse_stmt_or_begin() {
            Ok(node) => stmts.push(*node),
            Err(_) => return Ok(vec![]),
        }

        loop {
            match self.parse_terms() {
                Ok(mut tokens) => terms.append(&mut tokens),
                Err(_) => break,
            }

            match self.parse_stmt_or_begin() {
                Ok(node) => stmts.push(*node),
                Err(_) => break,
            }
        }

        Ok(stmts)
    }

    fn parse_stmt_or_begin(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("stmt_or_begin")
            .or_else(|| self.parse_stmt())
            .or_else(|| self.parse_preexe())
            .stop()
    }

    #[allow(unreachable_code, unused_mut)]
    pub(crate) fn parse_stmt(&mut self) -> ParseResult<Box<Node>> {
        let mut stmt = self.parse_stmt_head()?;
        match self.parse_stmt_tail() {
            Ok((mod_t, expr)) => match mod_t.kind {
                TokenKind::kIF => Ok(Builder::condition_mod(Some(stmt), None, mod_t, expr)),
                TokenKind::kUNLESS => Ok(Builder::condition_mod(None, Some(stmt), mod_t, expr)),
                TokenKind::kWHILE => Ok(Builder::loop_mod(LoopType::While, stmt, mod_t, expr)),
                TokenKind::kUNTIL => Ok(Builder::loop_mod(LoopType::Until, stmt, mod_t, expr)),
                _ => unreachable!("stmt_tail handles only if/unless/while/until modifiers"),
            },
            Err(error) => {
                match error.strip_lookaheads() {
                    None => {
                        // ignore
                        Ok(stmt)
                    }
                    Some(error) => Err(ParseError::seq_error("stmt tail", stmt, error)),
                }
            }
        }
    }

    fn parse_stmt_head(&mut self) -> ParseResult<Box<Node>> {
        if let Ok(alias) = self.parse_alias() {
            return Ok(alias);
        } else if let Ok(undef) = self.parse_undef() {
            return Ok(undef);
        } else if let Ok(postexe) = self.parse_postexe() {
            return Ok(postexe);
        } else if self.current_token().is(TokenKind::kDEF) {
            todo!("handle endless def")
        } else if let Ok(assignment) = self.parse_assignment() {
            return Ok(assignment);
        }

        self.parse_expr()
    }

    fn parse_stmt_tail(&mut self) -> ParseResult<(Token, Box<Node>)> {
        self.one_of("stmt tail")
            .or_else(|| {
                self.all_of("if_mod expr")
                    .and(|| self.try_token(TokenKind::kIF))
                    .and(|| self.parse_expr_value())
                    .stop()
            })
            .or_else(|| {
                self.all_of("unless_mod expr")
                    .and(|| self.try_token(TokenKind::kUNLESS))
                    .and(|| self.parse_expr_value())
                    .stop()
            })
            .or_else(|| {
                self.all_of("while_mod expr")
                    .and(|| self.try_token(TokenKind::kWHILE))
                    .and(|| self.parse_expr_value())
                    .stop()
            })
            .or_else(|| {
                self.all_of("until_mod expr")
                    .and(|| self.try_token(TokenKind::kUNTIL))
                    .and(|| self.parse_expr_value())
                    .stop()
            })
            .stop()
    }

    fn rescue_stmt(&mut self) -> ParseResult<(Token, Box<Node>)> {
        todo!()
    }

    fn parse_assignment(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("assignment")
            .or_else(|| self.parse_mass_assignment())
            .or_else(|| self.parse_simple_assignment())
            .or_else(|| {
                let (lhs, op_t, rhs) = self
                    .all_of("operation assignment")
                    .and(|| {
                        self.one_of("operation assignment lhs")
                            .or_else(|| {
                                let (primary_value, op_t, id_t) = self
                                    .all_of("primary call_op2 tIDENTIFIER")
                                    .and(|| self.parse_primary_value())
                                    .and(|| self.parse_call_op2())
                                    .and(|| self.parse_const_or_identifier())
                                    .stop()?;
                                panic!(
                                    "primary_value call_op tIDENT {:?} {:?} {:?}",
                                    primary_value, op_t, id_t
                                )
                            })
                            .or_else(|| {
                                let (primary_value, lbrack_t, opt_call_args, rbrack_t) = self
                                    .all_of("primary [ args ]")
                                    .and(|| self.parse_primary_value())
                                    .and(|| self.expect_token(TokenKind::tLBRACK))
                                    .and(|| self.parse_opt_call_args())
                                    .and(|| self.parse_rparen())
                                    .stop()?;
                                todo!(
                                    "{:?} {:?} {:?} {:?}",
                                    primary_value,
                                    lbrack_t,
                                    opt_call_args,
                                    rbrack_t
                                )
                            })
                            .or_else(|| self.parse_var_lhs())
                            .or_else(|| self.parse_back_ref())
                            .stop()
                    })
                    .and(|| self.expect_token(TokenKind::tOP_ASGN))
                    .and(|| self.parse_command_rhs())
                    .stop()?;

                todo!("{:?} {:?} {:?}", lhs, op_t, rhs)
            })
            .stop()
    }

    fn parse_mass_assignment(&mut self) -> ParseResult<Box<Node>> {
        let (mlhs, eql_t, rhs) = self
            .all_of("mass-assignment")
            .and(|| self.parse_mlhs())
            .and(|| self.expect_token(TokenKind::tEQL))
            .and(|| {
                self.one_of("mass-assginemtn rhs")
                    .or_else(|| self.parse_command_call())
                    .or_else(|| {
                        self.all_of("mrhs_arg [rescue stmt]")
                            .and(|| self.parse_mrhs_arg())
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

    fn parse_simple_assignment(&mut self) -> ParseResult<Box<Node>> {
        let (lhs, eql_t, rhs) = self
            .all_of("simple assignment")
            .and(|| self.parse_lhs())
            .and(|| self.expect_token(TokenKind::tEQL))
            .and(|| {
                self.one_of("simple assignment rhs")
                    .or_else(|| self.parse_command_call())
                    .or_else(|| self.parse_command_rhs())
                    .stop()
            })
            .stop()?;

        todo!("{:?} {:?} {:?}", lhs, eql_t, rhs)
    }
}
