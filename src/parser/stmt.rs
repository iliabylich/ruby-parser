use crate::{
    builder::{Builder, Constructor},
    parser::{ParseError, ParseResult, Parser},
    token::TokenKind,
    transactions::{ParseResultApi, StepData},
    Node, Token,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_top_compstmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let (top_stmts, _opt_terms) = self
            .all_of("top_compstmt")
            .and(|| self.try_top_stmts())
            .and(|| self.try_opt_terms())
            .unwrap()?;

        if top_stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::<C>::compstmt(top_stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn try_top_stmts(&mut self) -> ParseResult<Vec<Node>> {
        let mut top_stmts = vec![];
        loop {
            match self.try_top_stmt() {
                Ok(top_stmt) => top_stmts.push(*top_stmt),
                Err(error) => {
                    if error.is_lookahead() {
                        break;
                    }

                    return Err(ParseError::SeqError {
                        name: "top stmts",
                        steps: top_stmts
                            .into_iter()
                            .map(|top_stmt| StepData::from(Box::new(top_stmt)))
                            .collect::<Vec<_>>(),
                        error: Box::new(error),
                    });
                }
            }
        }
        Ok(top_stmts)
    }

    pub(crate) fn try_top_stmt(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("top-level statement")
            .or_else(|| self.try_preexe())
            .or_else(|| self.try_stmt())
            .unwrap()
    }

    pub(crate) fn try_bodystmt(&mut self) -> ParseResult<Box<Node>> {
        let (compstmt, rescue_bodies, opt_else, opt_ensure) = self
            .all_of("bodystmt")
            .and(|| self.try_compstmt())
            .and(|| self.try_opt_rescue())
            .and(|| self.try_opt_else())
            .and(|| self.try_opt_ensure())
            .unwrap()?;

        Ok(Builder::<C>::begin_body(
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
            .unwrap()?;
        if stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::<C>::compstmt(stmts)))
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
        if let Some((mod_t, expr)) = self.try_stmt_tail().ignore_lookaheads()? {
            match mod_t.kind {
                TokenKind::kIF => stmt = todo!("{:?} {:?} {:?}", stmt, mod_t, expr),
                TokenKind::kUNLESS => stmt = todo!("{:?} {:?} {:?}", stmt, mod_t, expr),
                TokenKind::kWHILE => stmt = todo!("{:?} {:?} {:?}", stmt, mod_t, expr),
                TokenKind::kUNTIL => stmt = todo!("{:?} {:?} {:?}", stmt, mod_t, expr),
                _ => unreachable!("stmt_tail handles only if/unless/while/until modifiers"),
            }
        }
        Ok(stmt)
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
                    .unwrap()
            })
            .or_else(|| {
                self.all_of("unless_mod expr")
                    .and(|| self.try_token(TokenKind::kUNLESS))
                    .and(|| self.try_expr_value())
                    .unwrap()
            })
            .or_else(|| {
                self.all_of("while_mod expr")
                    .and(|| self.try_token(TokenKind::kWHILE))
                    .and(|| self.try_expr_value())
                    .unwrap()
            })
            .or_else(|| {
                self.all_of("until_mod expr")
                    .and(|| self.try_token(TokenKind::kUNTIL))
                    .and(|| self.try_expr_value())
                    .unwrap()
            })
            .unwrap()
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
