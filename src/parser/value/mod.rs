use crate::{builder::Builder, parser::base::Rule, Node, Parser, TokenKind};

mod operators;
use operators::{binary_operator_power, postfix_operator_power, prefix_operator_power};

mod builders;
use builders::{build_binary_op, build_postfix_op, build_prefix_op};

mod value0;
use value0::Value0;

mod call_tail;
use call_tail::CallTail;

pub(crate) struct Value;
impl Rule for Value {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        Value0::starts_now(parser) || prefix_operator_power(parser.current_token()).is_some()
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        Self::parse_bp(parser, 0)
    }
}

impl Value {
    fn parse_lhs(parser: &mut Parser) -> Box<Node> {
        if parser.current_token().is(TokenKind::tLPAREN) {
            let begin_t = parser.take_token();
            let lhs = Self::parse_bp(parser, 0);
            let end_t = parser.expect_token(TokenKind::tRPAREN);
            Builder::begin(begin_t, vec![*lhs], end_t)
        } else if let Some((_, r_bp)) = prefix_operator_power(parser.current_token()) {
            let op_t = parser.take_token();
            let rhs = Self::parse_bp(parser, r_bp);
            build_prefix_op(op_t, rhs, parser)
        } else {
            let value0 = Value0::parse(parser);
            // TODO: repeat CallTail
            value0
        }
    }

    fn parse_with_lhs(parser: &mut Parser, mut lhs: Box<Node>, min_bp: u8) -> Box<Node> {
        loop {
            let op_t = parser.current_token();

            if op_t.is(TokenKind::tEOF) {
                break;
            }

            if let Some((l_bp, _)) = postfix_operator_power(op_t) {
                if l_bp < min_bp {
                    break;
                }
                parser.skip_token();

                lhs = build_postfix_op(op_t, lhs, parser);
                continue;
            }

            if let Some((l_bp, r_bp)) = binary_operator_power(op_t) {
                if l_bp < min_bp {
                    break;
                }
                parser.skip_token();

                lhs = build_binary_op(op_t, lhs, parser, r_bp);
                continue;
            }

            break;
        }

        lhs
    }

    fn parse_bp(parser: &mut Parser, min_bp: u8) -> Box<Node> {
        let lhs = Self::parse_lhs(parser);

        Self::parse_with_lhs(parser, lhs, min_bp)
    }
}
