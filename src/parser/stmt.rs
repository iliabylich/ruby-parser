use crate::{
    builder::{Builder, LoopType},
    parser::{
        macros::{all_of, maybe, one_of},
        ParseResult, Parser,
    },
    token::TokenKind,
    Node, Token,
};

use super::macros::separated_by::separated_by;

impl Parser {
    pub(crate) fn try_top_compstmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let (top_stmts, _opt_terms) = all_of!(
            "top_compstmt",
            self.parse_top_stmts(),
            self.parse_opt_terms(),
        )?;

        if top_stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::compstmt(top_stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn parse_top_stmts(&mut self) -> ParseResult<Vec<Node>> {
        let (top_stmts, _terms) = separated_by!(
            "top_stmts",
            checkpoint = self.new_checkpoint(),
            item = self.parse_top_stmt(),
            sep = self.parse_terms()
        )?;
        Ok(top_stmts)
    }

    pub(crate) fn parse_top_stmt(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "top-level statement",
            checkpoint = self.new_checkpoint(),
            self.parse_preexe(),
            self.parse_stmt(),
        )
    }

    pub(crate) fn try_bodystmt(&mut self) -> ParseResult<Option<Box<Node>>> {
        let (compstmt, rescue_bodies, opt_else, opt_ensure) = all_of!(
            "bodystmt",
            self.try_compstmt(),
            self.parse_opt_rescue(),
            self.try_opt_else(),
            self.try_opt_ensure(),
        )?;

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
        let (stmts, _opt_terms) = all_of!("compstmt", self.parse_stmts(), self.parse_opt_terms(),)?;
        if stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::compstmt(stmts)))
        }
    }

    // This rule can be `none`
    pub(crate) fn parse_stmts(&mut self) -> ParseResult<Vec<Node>> {
        let (stmts, _terms) = separated_by!(
            "stmts",
            checkpoint = self.new_checkpoint(),
            item = self.parse_stmt_or_begin(),
            sep = self.parse_terms()
        )?;

        Ok(stmts)
    }

    fn parse_stmt_or_begin(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "stmt_or_begin",
            checkpoint = self.new_checkpoint(),
            self.parse_stmt(),
            self.parse_preexe(),
        )
    }

    #[allow(unreachable_code, unused_mut)]
    pub(crate) fn parse_stmt(&mut self) -> ParseResult<Box<Node>> {
        let (mut stmt, tail) = all_of!(
            "stmt",
            self.parse_stmt_head(),
            maybe!(self.parse_stmt_tail()),
        )?;

        if let Some((mod_t, expr)) = tail {
            stmt = match mod_t.kind {
                TokenKind::kIF => Builder::condition_mod(Some(stmt), None, mod_t, expr),
                TokenKind::kUNLESS => Builder::condition_mod(None, Some(stmt), mod_t, expr),
                TokenKind::kWHILE => Builder::loop_mod(LoopType::While, stmt, mod_t, expr),
                TokenKind::kUNTIL => Builder::loop_mod(LoopType::Until, stmt, mod_t, expr),
                _ => unreachable!("stmt_tail handles only if/unless/while/until modifiers"),
            }
        }

        Ok(stmt)
    }

    fn parse_stmt_head(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "stmt head",
            self.parse_alias(),
            self.parse_undef(),
            self.parse_postexe(),
            all_of!("endless def", self.try_token(TokenKind::kDEF),)
                .map(|_| todo!("handle endless def")),
            self.parse_assignment(),
            self.parse_expr(),
        )
    }

    fn parse_stmt_tail(&mut self) -> ParseResult<(Token, Box<Node>)> {
        one_of!(
            "stmt tail",
            checkpoint = self.new_checkpoint(),
            {
                all_of!(
                    "if_mod expr",
                    self.try_token(TokenKind::kIF),
                    self.parse_expr_value(),
                )
            },
            {
                all_of!(
                    "unless_mod expr",
                    self.try_token(TokenKind::kUNLESS),
                    self.parse_expr_value(),
                )
            },
            {
                all_of!(
                    "while_mod expr",
                    self.try_token(TokenKind::kWHILE),
                    self.parse_expr_value(),
                )
            },
            {
                all_of!(
                    "until_mod expr",
                    self.try_token(TokenKind::kUNTIL),
                    self.parse_expr_value(),
                )
            },
        )
    }

    fn rescue_stmt(&mut self) -> ParseResult<(Token, Box<Node>)> {
        todo!()
    }

    fn parse_assignment(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "assignment",
            checkpoint = self.new_checkpoint(),
            self.parse_mass_assignment(),
            self.parse_simple_assignment(),
            {
                let (lhs, op_t, rhs) = all_of!(
                    "operation assignment",
                    {
                        one_of!(
                            "operation assignment lhs",
                            checkpoint = self.new_checkpoint(),
                            {
                                let (primary_value, op_t, id_t) = all_of!(
                                    "primary call_op2 tIDENTIFIER",
                                    self.parse_primary_value(),
                                    self.parse_call_op2(),
                                    self.try_const_or_identifier(),
                                )?;
                                panic!(
                                    "primary_value call_op tIDENT {:?} {:?} {:?}",
                                    primary_value, op_t, id_t
                                )
                            },
                            {
                                let (primary_value, lbrack_t, opt_call_args, rbrack_t) = all_of!(
                                    "primary [ args ]",
                                    self.parse_primary_value(),
                                    self.expect_token(TokenKind::tLBRACK),
                                    self.parse_opt_call_args(),
                                    self.parse_rparen(),
                                )?;
                                todo!(
                                    "{:?} {:?} {:?} {:?}",
                                    primary_value,
                                    lbrack_t,
                                    opt_call_args,
                                    rbrack_t
                                )
                            },
                            self.parse_var_lhs(),
                            self.try_back_ref(),
                        )
                    },
                    self.expect_token(TokenKind::tOP_ASGN),
                    self.parse_command_rhs(),
                )?;

                todo!("{:?} {:?} {:?}", lhs, op_t, rhs)
            },
        )
    }

    fn parse_mass_assignment(&mut self) -> ParseResult<Box<Node>> {
        let (mlhs, eql_t, rhs) = all_of!(
            "mass-assignment",
            self.parse_mlhs(),
            self.expect_token(TokenKind::tEQL),
            {
                one_of!(
                    "mass-assginemtn rhs",
                    checkpoint = self.new_checkpoint(),
                    self.parse_command_call(),
                    {
                        let (value, rescue) =
                            all_of!("mrhs_arg [rescue stmt]", self.parse_mrhs_arg(), {
                                let maybe_rescut_stmt = one_of!(
                                    "[rescue stmt]",
                                    checkpoint = self.new_checkpoint(),
                                    self.rescue_stmt().map(|data| Some(data)),
                                    Ok(None),
                                )?;
                                #[allow(unreachable_code)]
                                Ok(todo!("{:?}", maybe_rescut_stmt) as Box<Node>)
                            },)?;
                        todo!("{:?} {:?}", value, rescue)
                    },
                )
            },
        )?;
        todo!("{:?} {:?} {:?}", mlhs, eql_t, rhs)
    }

    fn parse_simple_assignment(&mut self) -> ParseResult<Box<Node>> {
        let (lhs, eql_t, rhs) = all_of!(
            "simple assignment",
            self.parse_lhs(),
            self.expect_token(TokenKind::tEQL),
            {
                one_of!(
                    "simple assignment rhs",
                    checkpoint = self.new_checkpoint(),
                    self.parse_command_call(),
                    self.parse_command_rhs(),
                )
            },
        )?;

        todo!("{:?} {:?} {:?}", lhs, eql_t, rhs)
    }
}
