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
            parse_top_stmts(self),
            self.parse_opt_terms(),
        )?;

        if top_stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::compstmt(top_stmts)))
        }
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
        let (_pre_terms, stmts, _post_terms) = all_of!(
            "compstmt",
            self.parse_opt_terms(),
            parse_stmts(self),
            self.parse_opt_terms(),
        )?;
        if stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::compstmt(stmts)))
        }
    }

    pub(crate) fn parse_stmt(&mut self) -> ParseResult<Box<Node>> {
        let (mut stmt, tail) =
            all_of!("stmt", parse_stmt_head(self), maybe!(parse_stmt_tail(self)),)?;

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
}

// This rule can be `none`
fn parse_top_stmts(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let top_stmts = maybe!(separated_by!(
        "top_stmts",
        checkpoint = parser.new_checkpoint(),
        item = parse_top_stmt(parser),
        sep = parser.parse_terms()
    ))?;

    match top_stmts {
        Some((top_stmts, _)) => Ok(top_stmts),
        None => Ok(vec![]),
    }
}

fn parse_top_stmt(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "top-level statement",
        checkpoint = parser.new_checkpoint(),
        parser.parse_preexe(),
        parser.parse_stmt(),
    )
}

fn parse_stmt_or_begin(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "stmt_or_begin",
        checkpoint = parser.new_checkpoint(),
        parser.parse_stmt(),
        parser.parse_preexe(),
    )
}
// This rule can be `none`
fn parse_stmts(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let stmts = maybe!(separated_by!(
        "stmts",
        checkpoint = parser.new_checkpoint(),
        item = parse_stmt_or_begin(parser),
        sep = parser.parse_terms()
    ))?;

    match stmts {
        Some((stmts, _)) => Ok(stmts),
        None => Ok(vec![]),
    }
}

fn parse_stmt_head(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "stmt head",
        /* Alias */
        /* Undef */
        parser.parse_postexe(),
        all_of!("endless def", parser.try_token(TokenKind::kDEF),)
            .map(|_| todo!("handle endless def")),
        parse_assignment(parser),
        parser.parse_expr(),
    )
}

fn parse_stmt_tail(parser: &mut Parser) -> ParseResult<(Token, Box<Node>)> {
    one_of!(
        "stmt tail",
        checkpoint = parser.new_checkpoint(),
        all_of!(
            "if_mod expr",
            parser.try_token(TokenKind::kIF),
            parser.parse_expr_value(),
        ),
        all_of!(
            "unless_mod expr",
            parser.try_token(TokenKind::kUNLESS),
            parser.parse_expr_value(),
        ),
        all_of!(
            "while_mod expr",
            parser.try_token(TokenKind::kWHILE),
            parser.parse_expr_value(),
        ),
        all_of!(
            "until_mod expr",
            parser.try_token(TokenKind::kUNTIL),
            parser.parse_expr_value(),
        ),
    )
}

fn parse_rescue_stmt(_parser: &mut Parser) -> ParseResult<(Token, Box<Node>)> {
    todo!()
}

fn parse_assignment(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "assignment",
        checkpoint = parser.new_checkpoint(),
        parse_simple_assignment(parser),
        parse_mass_assignment(parser),
        {
            let (lhs, op_t, rhs) = all_of!(
                "operation assignment",
                {
                    one_of!(
                        "operation assignment lhs",
                        checkpoint = parser.new_checkpoint(),
                        {
                            let (primary_value, op_t, id_t) = all_of!(
                                "primary call_op2 tIDENTIFIER",
                                parser.parse_primary_value(),
                                parser.parse_call_op2(),
                                parser.parse_const_or_identifier(),
                            )?;
                            panic!(
                                "primary_value call_op tIDENTIFIER {:?} {:?} {:?}",
                                primary_value, op_t, id_t
                            )
                        },
                        {
                            let (primary_value, lbrack_t, opt_call_args, rbrack_t) = all_of!(
                                "primary [ args ]",
                                parser.parse_primary_value(),
                                parser.expect_token(TokenKind::tLBRACK),
                                parser.parse_opt_call_args(),
                                parser.parse_rparen(),
                            )?;
                            todo!(
                                "{:?} {:?} {:?} {:?}",
                                primary_value,
                                lbrack_t,
                                opt_call_args,
                                rbrack_t
                            )
                        },
                        parser.parse_var_lhs(),
                        parser.parse_back_ref(),
                    )
                },
                parser.expect_token(TokenKind::tOP_ASGN),
                parser.parse_command_rhs(),
            )?;

            todo!("{:?} {:?} {:?}", lhs, op_t, rhs)
        },
    )
}

fn parse_mass_assignment(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (mlhs, eql_t, rhs) = all_of!(
        "mass-assignment",
        parser.parse_mlhs(),
        parser.expect_token(TokenKind::tEQL),
        {
            one_of!(
                "mass-assginemtn rhs",
                checkpoint = parser.new_checkpoint(),
                parser.parse_command_call(),
                {
                    let (value, rescue) =
                        all_of!("mrhs_arg [rescue stmt]", parser.parse_mrhs_arg(), {
                            let maybe_rescut_stmt = one_of!(
                                "[rescue stmt]",
                                checkpoint = parser.new_checkpoint(),
                                parse_rescue_stmt(parser).map(|data| Some(data)),
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

fn parse_simple_assignment(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (lhs, eql_t, rhs) = all_of!(
        "simple assignment",
        parser.parse_lhs(),
        parser.expect_token(TokenKind::tEQL),
        {
            one_of!(
                "simple assignment rhs",
                checkpoint = parser.new_checkpoint(),
                parser.parse_command_call(),
                parser.parse_command_rhs(),
            )
        },
    )?;

    todo!("{:?} {:?} {:?}", lhs, eql_t, rhs)
}
