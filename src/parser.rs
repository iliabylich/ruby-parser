use crate::lexer::Lexer;
use crate::node::Node;
use crate::token::{BinOp, Token};

pub struct Parser {
    lexer: Lexer,
    debug: bool,
}

impl Parser {
    pub fn new(s: &str) -> Self {
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

    pub fn parse(&mut self) -> Node {
        // Initiate tokenizer
        self.lexer.get_next_token();

        self.parse_expression()
    }

    pub fn parse_expression(&mut self) -> Node {
        let lhs = self.parse_primary();
        self.parse_expression_1(lhs, 0)
    }

    fn parse_primary(&mut self) -> Node {
        match self.lexer.current_token() {
            Token::tLPAREN => {
                self.lexer.get_next_token();
                let inner = self.parse_expression();
                if self.lexer.current_token() != Token::tRPAREN {
                    panic!("parse error: expected )")
                }
                self.lexer.get_next_token();
                Node::Parenthesized(Box::new(inner))
            }

            Token::tEOF => panic!("EOF"),
            Token::Error(c) => panic!("Tokenizer error: {}", c),

            Token::tINTEGER(n) => {
                self.lexer.get_next_token();
                Node::Number(n)
            }

            other => panic!("parse_primary: expected Number or Lparen, got {:?}", other),
        }
    }

    fn parse_expression_1(&mut self, mut lhs: Node, min_prec: u8) -> Node {
        let mut lookahead = self.lexer.current_token();
        while let Token::BinOp(bin_op) = lookahead {
            if bin_op.precedence() < min_prec {
                break;
            }
            self.lexer.get_next_token();
            let mut rhs = self.parse_primary();
            lookahead = self.lexer.current_token();
            while let Token::BinOp(lookahead_bin_op) = lookahead {
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
            };
        }

        lhs
    }
}

#[test]
fn test_parse() {
    let ast = Parser::new("22 + 3 ** 4 * (2 + 2) - 1").parse();
    assert_eq!(
        ast,
        Node::Minus(
            Box::new(Node::Plus(
                Box::new(Node::Number(22)),
                Box::new(Node::Mult(
                    Box::new(Node::Pow(
                        Box::new(Node::Number(3)),
                        Box::new(Node::Number(4))
                    )),
                    Box::new(Node::Parenthesized(Box::new(Node::Plus(
                        Box::new(Node::Number(2)),
                        Box::new(Node::Number(2))
                    ))))
                ))
            )),
            Box::new(Node::Number(1))
        )
    );
    assert_eq!(ast.eval(), 345);
}
