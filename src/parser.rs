use crate::lexer::Lexer;
use crate::node::Node;
use crate::token::{BinOp, TokenValue};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    debug: bool,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            lexer: Lexer::new(s),
            debug: false,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self.lexer = self.lexer.debug();
        self
    }

    pub fn parse(&mut self) -> Box<Node<'a>> {
        // Initiate tokenizer
        self.lexer.get_next_token();

        self.parse_expression()
    }

    pub fn parse_expression(&mut self) -> Box<Node<'a>> {
        let lhs = self.parse_primary();
        self.parse_expression_1(lhs, 0)
    }

    fn parse_primary(&mut self) -> Box<Node<'a>> {
        match self.lexer.current_token().value() {
            TokenValue::tLPAREN => {
                self.lexer.get_next_token();
                let inner = self.parse_expression();
                if self.lexer.current_token().value() != TokenValue::tRPAREN {
                    panic!("parse error: expected )")
                }
                self.lexer.get_next_token();
                Box::new(Node::Parenthesized(inner))
            }

            TokenValue::tEOF => panic!("EOF"),
            TokenValue::Error(c) => panic!("Tokenizer error: {}", c),

            TokenValue::tINTEGER(n) => {
                self.lexer.get_next_token();
                Box::new(Node::Number(n))
            }

            other => panic!("parse_primary: expected Number or Lparen, got {:?}", other),
        }
    }

    fn parse_expression_1(&mut self, mut lhs: Box<Node<'a>>, min_prec: u8) -> Box<Node<'a>> {
        let mut lookahead = self.lexer.current_token();
        while let TokenValue::BinOp(bin_op) = lookahead.value() {
            if bin_op.precedence() < min_prec {
                break;
            }
            self.lexer.get_next_token();
            let mut rhs = self.parse_primary();
            lookahead = self.lexer.current_token();
            while let TokenValue::BinOp(lookahead_bin_op) = lookahead.value() {
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
                BinOp::tPLUS => Box::new(Node::Plus(lhs, rhs)),
                BinOp::tMINUS => Box::new(Node::Minus(lhs, rhs)),
                BinOp::tSTAR => Box::new(Node::Mult(lhs, rhs)),
                BinOp::tDIVIDE => Box::new(Node::Minus(lhs, rhs)),
                BinOp::tPOW => Box::new(Node::Pow(lhs, rhs)),
                _ => unimplemented!("bin_op {:?}", bin_op),
            };
        }

        lhs
    }
}

#[test]
fn test_parse() {
    let ast = Parser::new("22 + 3 ** 4 * (2 + 2) - 1").debug().parse();
    assert_eq!(
        ast,
        Box::new(Node::Minus(
            Box::new(Node::Plus(
                Box::new(Node::Number("22")),
                Box::new(Node::Mult(
                    Box::new(Node::Pow(
                        Box::new(Node::Number("3")),
                        Box::new(Node::Number("4"))
                    )),
                    Box::new(Node::Parenthesized(Box::new(Node::Plus(
                        Box::new(Node::Number("2")),
                        Box::new(Node::Number("2"))
                    ))))
                ))
            )),
            Box::new(Node::Number("1"))
        ))
    );
    assert_eq!(ast.eval(), 345);
}
