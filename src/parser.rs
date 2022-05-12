use crate::Lexer;
use crate::Node;
use crate::{token::OpPrecedence, Token};

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

        self.parse_expression(1)
    }

    pub fn parse_expression(&mut self, min_prec: u8) -> Node {
        let mut lhs = match self.lexer.current_token() {
            Token::Lparen => {
                self.lexer.get_next_token();
                let inner = self.parse_expression(1);
                if self.lexer.current_token() != Token::Rparen {
                    panic!("parse error: expected )")
                }
                self.lexer.get_next_token();
                Node::Parenthesized(Box::new(inner))
            }

            Token::EOF => panic!("EOF"),
            Token::Error(c) => panic!("Tokenizer error: {}", c),

            Token::Number(n) => {
                self.lexer.get_next_token();
                Node::Number(n)
            }

            other => panic!("Expected Number or Lparen, got {:?}", other),
        };

        loop {
            let token = self.lexer.current_token();
            match token {
                Token::EOF => break,
                Token::Error(c) => panic!("Tokenizer error during bino RHS parsing: {}", c),

                invalid if !invalid.is_bin_op() || invalid.precedence().number() < min_prec => {
                    break;
                }

                next_bin_op if next_bin_op.is_bin_op() => {
                    debug_assert!(next_bin_op.is_bin_op());

                    let next_min_prec = match next_bin_op.precedence() {
                        OpPrecedence::Left(prec) => prec + 1,
                        OpPrecedence::Right(prec) => prec,
                        OpPrecedence::Unknown => unreachable!(),
                    };
                    self.lexer.get_next_token();
                    let rhs = self.parse_expression(next_min_prec);

                    lhs = match next_bin_op {
                        Token::Plus => Node::Plus(Box::new(lhs), Box::new(rhs)),
                        Token::Minus => Node::Minus(Box::new(lhs), Box::new(rhs)),
                        Token::Mult => Node::Mult(Box::new(lhs), Box::new(rhs)),
                        Token::Div => Node::Minus(Box::new(lhs), Box::new(rhs)),
                        Token::Pow => Node::Pow(Box::new(lhs), Box::new(rhs)),
                        other => unreachable!("expected bin op, got {:?}", other),
                    };
                }

                unsupported => panic!("Unsupported token {:?}", unsupported),
            }
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
