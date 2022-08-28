use crate::{
    parser::{
        macros::{all_of, one_of},
        ParseResult, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_method(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "method definition",
            checkpoint = self.new_checkpoint(),
            {
                let ((def_t, name_t), args, body, end_t) = all_of!(
                    "instance method definition",
                    self.parse_defn_head(),
                    parse_f_arglist(self),
                    self.try_bodystmt(),
                    self.parse_k_end(),
                )?;

                todo!("{:?} {:?} {:?} {:?} {:?}", def_t, name_t, args, body, end_t)
            },
            {
                let ((def_t, singleton, op_t, name_t), args, body, end_t) = all_of!(
                    "singleton method definition",
                    self.parse_defs_head(),
                    parse_f_arglist(self),
                    self.try_bodystmt(),
                    self.parse_k_end(),
                )?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                    def_t,
                    singleton,
                    op_t,
                    name_t,
                    args,
                    body,
                    end_t
                )
            },
        )
    }

    pub(crate) fn parse_defn_head(&mut self) -> ParseResult<(Token, Token)> {
        all_of!(
            "instance method definition start",
            parse_k_def(self),
            self.parse_def_name(),
        )
    }

    pub(crate) fn parse_defs_head(&mut self) -> ParseResult<(Token, Box<Node>, Token, Token)> {
        all_of!(
            "singleton method definition start",
            parse_k_def(self),
            parse_singleton(self),
            self.parse_dot_or_colon(),
            self.parse_def_name(),
        )
    }
}

fn parse_k_def(parser: &mut Parser) -> ParseResult<Token> {
    parser.try_token(TokenKind::kDEF)
}

fn parse_f_arglist(_parser: &mut Parser) -> ParseResult<Box<Node>> {
    todo!("parser.parse_f_arglist")
}

fn parse_singleton(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "singleton",
        checkpoint = parser.new_checkpoint(),
        parser.parse_var_ref(),
        {
            let (lparen_t, expr, rparen_t) = all_of!(
                "(expr)",
                parser.try_token(TokenKind::tLPAREN),
                parser.parse_expr(),
                parser.parse_rparen(),
            )?;
            todo!("{:?} {:?} {:?}", lparen_t, expr, rparen_t)
        },
    )
}
