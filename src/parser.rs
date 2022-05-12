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

        self.parse_expression()
    }

    pub fn parse_expression(&mut self) -> Node {
        let lhs = self.parse_primary();
        self.parse_expression_1(lhs, 0)
    }

    fn parse_primary(&mut self) -> Node {
        match self.lexer.current_token() {
            Token::Lparen => {
                self.lexer.get_next_token();
                let inner = self.parse_expression();
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

            other => panic!("parse_primary: expected Number or Lparen, got {:?}", other),
        }
    }

    fn parse_expression_1(&mut self, mut lhs: Node, min_prec: u8) -> Node {
        let mut lookahead = self.lexer.current_token();
        while lookahead.is_bin_op() && lookahead.precedence() >= min_prec {
            let op = lookahead;
            self.lexer.get_next_token();
            let mut rhs = self.parse_primary();
            lookahead = self.lexer.current_token();
            while lookahead.is_bin_op()
                && (lookahead.precedence() > op.precedence()
                    || (lookahead.precedence().is_right_associative()
                        && lookahead.precedence() == op.precedence()))
            {
                rhs = self.parse_expression_1(
                    rhs,
                    op.precedence().as_number()
                        + if lookahead.precedence() > op.precedence() {
                            1
                        } else {
                            0
                        },
                );
                lookahead = self.lexer.current_token();
            }
            lhs = match op {
                Token::Plus => Node::Plus(Box::new(lhs), Box::new(rhs)),
                Token::Minus => Node::Minus(Box::new(lhs), Box::new(rhs)),
                Token::Mult => Node::Mult(Box::new(lhs), Box::new(rhs)),
                Token::Div => Node::Minus(Box::new(lhs), Box::new(rhs)),
                Token::Pow => Node::Pow(Box::new(lhs), Box::new(rhs)),
                other => unreachable!("expected bin op, got {:?}", other),
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
