use crate::bin_op::BinOp;
use crate::lexer::Lexer;
use crate::node::Node;
use crate::token::TokenValue;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    debug: bool,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            lexer: Lexer::new(input),
            debug: false,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self.lexer = self.lexer.debug();
        self
    }

    pub fn parse(&mut self) -> Node<'a> {
        self.parse_expression()
    }

    pub fn parse_expression(&mut self) -> Node<'a> {
        let lhs = self.parse_primary();
        self.parse_expression_1(lhs, 0)
    }

    fn parse_primary(&mut self) -> Node<'a> {
        match self.lexer.current_token().value() {
            TokenValue::tLPAREN => {
                self.lexer.skip_token();
                let inner = self.parse_expression();
                if self.lexer.current_token().value() != TokenValue::tRPAREN {
                    panic!("parse error: expected )")
                }
                self.lexer.skip_token();
                Node::Parenthesized(Box::new(inner))
            }

            TokenValue::tEOF => panic!("EOF"),
            TokenValue::Error(c) => panic!("Tokenizer error: {}", c),

            TokenValue::tINTEGER => {
                let loc = self.lexer.current_token().loc();
                let node = Node::Number(self.lexer.buffer.slice(loc.begin(), loc.end()));
                self.lexer.skip_token();
                node
            }

            other => panic!("parse_primary: expected Number or Lparen, got {:?}", other),
        }
    }

    fn parse_expression_1(&mut self, mut lhs: Node<'a>, min_prec: u8) -> Node<'a> {
        let mut lookahead = self.lexer.current_token();
        while let Ok(bin_op) = BinOp::try_from(lookahead.value()) {
            if bin_op.precedence() < min_prec {
                break;
            }
            self.lexer.skip_token();
            let mut rhs = self.parse_primary();
            lookahead = self.lexer.current_token();
            while let Ok(lookahead_bin_op) = BinOp::try_from(lookahead.value()) {
                if !(lookahead_bin_op.precedence() > bin_op.precedence()
                    || (lookahead_bin_op.precedence().is_right_associative()
                        && lookahead_bin_op.precedence() == bin_op.precedence()))
                {
                    break;
                }
                rhs = self.parse_expression_1(
                    rhs,
                    bin_op.precedence().as_number()
                        + if lookahead_bin_op.precedence() > bin_op.precedence() {
                            1
                        } else {
                            0
                        },
                );
                lookahead = self.lexer.current_token();
            }
            lhs = match bin_op {
                BinOp::tPLUS => Node::Plus(Box::new(lhs), Box::new(rhs)),
                BinOp::tMINUS => Node::Minus(Box::new(lhs), Box::new(rhs)),
                BinOp::tSTAR => Node::Mult(Box::new(lhs), Box::new(rhs)),
                BinOp::tDIVIDE => Node::Minus(Box::new(lhs), Box::new(rhs)),
                BinOp::tPOW => Node::Pow(Box::new(lhs), Box::new(rhs)),
                _ => unimplemented!("bin_op {:?}", bin_op),
            };
        }

        lhs
    }
}

#[test]
fn test_parse() {
    let ast = Parser::new(b"22 + 3 ** 4 * (2 + 2) - 1").debug().parse();
    assert_eq!(
        ast,
        Node::Minus(
            Box::new(Node::Plus(
                Box::new(Node::Number(b"22")),
                Box::new(Node::Mult(
                    Box::new(Node::Pow(
                        Box::new(Node::Number(b"3")),
                        Box::new(Node::Number(b"4"))
                    )),
                    Box::new(Node::Parenthesized(Box::new(Node::Plus(
                        Box::new(Node::Number(b"2")),
                        Box::new(Node::Number(b"2"))
                    ))))
                ))
            )),
            Box::new(Node::Number(b"1"))
        )
    );
    assert_eq!(ast.eval(), 345);
}
