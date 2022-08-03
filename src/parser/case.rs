use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_case(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("case expr")
            .or_else(|| {
                let (case_t, expr, _terms, body, end_t) = self
                    .all_of("k_case expr_value opt_terms case_body k_end")
                    .and(|| self.parse_k_case())
                    .and(|| self.parse_expr_value())
                    .and(|| self.parse_opt_terms())
                    .and(|| self.parse_case_body())
                    .and(|| self.parse_k_end())
                    .stop()?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?}",
                    case_t,
                    expr,
                    _terms,
                    body,
                    end_t
                )
            })
            .or_else(|| {
                let (case_t, _opt_terms, body, end_t) = self
                    .all_of("k_case opt_terms case_body k_end")
                    .and(|| self.parse_k_case())
                    .and(|| self.parse_opt_terms())
                    .and(|| self.parse_case_body())
                    .and(|| self.parse_k_end())
                    .stop()?;

                todo!("{:?} {:?} {:?} {:?}", case_t, _opt_terms, body, end_t)
            })
            .or_else(|| {
                let (case_t, expr, _opt_terms, p_case_body, end_t) = self
                    .all_of("k_case expr_value opt_terms p_case_body k_end")
                    .and(|| self.parse_k_case())
                    .and(|| self.parse_expr_value())
                    .and(|| self.parse_opt_terms())
                    .and(|| self.parse_p_case_body())
                    .and(|| self.parse_k_end())
                    .stop()?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?}",
                    case_t,
                    expr,
                    _opt_terms,
                    p_case_body,
                    end_t
                )
            })
            .stop()
    }

    fn parse_case_args(&mut self) {
        todo!("parser.parse_case_args")
    }
    fn parse_case_body(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.parse_case_body")
    }
    fn parse_cases(&mut self) {
        todo!("parser.parse_cases")
    }

    fn parse_k_case(&mut self) -> ParseResult<Token> {
        self.parse_token(TokenKind::kCASE)
    }
}
