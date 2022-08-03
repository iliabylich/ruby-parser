use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_method(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("method definition")
            .or_else(|| {
                let ((def_t, name_t), args, body, end_t) = self
                    .all_of("instance method definition")
                    .and(|| self.parse_defn_head())
                    .and(|| self.parse_f_arglist())
                    .and(|| self.try_bodystmt())
                    .and(|| self.parse_k_end())
                    .stop()?;

                todo!("{:?} {:?} {:?} {:?} {:?}", def_t, name_t, args, body, end_t)
            })
            .or_else(|| {
                let ((def_t, singleton, op_t, name_t), args, body, end_t) = self
                    .all_of("singleton method definition")
                    .and(|| self.parse_defs_head())
                    .and(|| self.parse_f_arglist())
                    .and(|| self.try_bodystmt())
                    .and(|| self.parse_k_end())
                    .stop()?;

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
            })
            .stop()
    }

    pub(crate) fn parse_defn_head(&mut self) -> ParseResult<(Token, Token)> {
        self.all_of("instance method definition start")
            .and(|| self.parse_k_def())
            .and(|| self.parse_def_name())
            .stop()
    }

    pub(crate) fn parse_defs_head(&mut self) -> ParseResult<(Token, Box<Node>, Token, Token)> {
        self.all_of("singleton method definition start")
            .and(|| self.parse_k_def())
            .and(|| self.parse_singleton())
            .and(|| self.parse_dot_or_colon())
            .and(|| self.parse_def_name())
            .stop()
    }

    fn parse_k_def(&mut self) -> ParseResult<Token> {
        self.parse_token(TokenKind::kDEF)
    }

    fn parse_f_arglist(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.parse_f_arglist")
    }

    fn parse_singleton(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("singleton")
            .or_else(|| self.parse_var_ref())
            .or_else(|| {
                let (lparen_t, expr, rparen_t) = self
                    .all_of("(expr)")
                    .and(|| self.parse_token(TokenKind::tLPAREN))
                    .and(|| self.parse_expr())
                    .and(|| self.parse_rparen())
                    .stop()?;
                todo!("{:?} {:?} {:?}", lparen_t, expr, rparen_t)
            })
            .stop()
    }
}
