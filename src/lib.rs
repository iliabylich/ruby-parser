mod token;
use token::OpPrecedence;
pub use token::Token;

mod node;
pub use node::Node;

pub struct Parser {
    input: Vec<u8>,
    pos: usize,
    current_token: Token,
    debug: bool,
}

impl Parser {
    pub fn new(s: &str) -> Self {
        Self {
            input: s.as_bytes().to_vec(),
            pos: 0,
            current_token: Token::None,
            debug: false,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn parse(&mut self) -> Node {
        // Initiate tokenizer
        self.get_next_token();

        self.parse_expression(1)
    }

    pub fn parse_expression(&mut self, min_prec: u8) -> Node {
        let mut lhs = match self.current_token {
            Token::Lparen => {
                self.get_next_token();
                let inner = self.parse_expression(1);
                if self.current_token != Token::Rparen {
                    panic!("parse error: expected )")
                }
                self.get_next_token();
                Node::Parenthesized(Box::new(inner))
            }

            Token::EOF => panic!("EOF"),
            Token::Error(c) => panic!("Tokenizer error: {}", c),

            Token::Number(n) => {
                self.get_next_token();
                Node::Number(n)
            }

            other => panic!("Expected Number or Lparen, got {:?}", other),
        };

        loop {
            let token = self.current_token;
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
                    self.get_next_token();
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

    fn get_next_token(&mut self) {
        // skip whitespaces
        while self.current_byte() == Some(b' ') {
            self.pos += 1;
        }

        let token = match self.current_byte() {
            None => Token::EOF,
            Some(b'+') => {
                self.pos += 1;
                Token::Plus
            }
            Some(b'-') => {
                self.pos += 1;
                Token::Minus
            }
            Some(b'*') => {
                self.pos += 1;
                match self.current_byte() {
                    Some(b'*') => {
                        self.pos += 1;
                        Token::Pow
                    }
                    _ => Token::Mult,
                }
            }
            Some(b'/') => {
                self.pos += 1;
                Token::Div
            }
            Some(b'(') => {
                self.pos += 1;
                Token::Lparen
            }
            Some(b')') => {
                self.pos += 1;
                Token::Rparen
            }
            Some(byte) if byte.is_ascii_digit() => {
                let start = self.pos;
                self.pos += 1;
                while let Some(byte) = self.current_byte() {
                    if !byte.is_ascii_digit() {
                        break;
                    }
                    self.pos += 1;
                }
                let num = &self.input[start..self.pos];
                let num = unsafe { std::str::from_utf8_unchecked(num) }
                    .parse::<u32>()
                    .unwrap();
                Token::Number(num)
            }
            Some(byte) => Token::Error(byte as char),
        };

        if self.debug {
            println!("Reading token {:?}", token);
        }
        self.current_token = token
    }

    fn current_byte(&mut self) -> Option<u8> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
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
