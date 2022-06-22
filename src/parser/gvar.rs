use super::*;
use crate::builder::{Builder, Constructor};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn parse_gvar(&mut self) -> Option<Box<Node<'a>>> {
        if self.current_token().value() != &TokenValue::tGVAR {
            return None;
        }

        let t_gvar = self.take_token();
        Some(Builder::<C>::gvar(
            t_gvar,
            self.lexer.buffer.for_lookahead(),
        ))
    }

    pub(crate) fn parse_back_ref(&mut self) -> Option<Box<Node<'a>>> {
        if self.current_token().value() != &TokenValue::tBACK_REF {
            return None;
        }

        let t_back_ref = self.take_token();
        Some(Builder::<C>::back_ref(
            t_back_ref,
            self.lexer.buffer.for_lookahead(),
        ))
    }

    pub(crate) fn parse_nth_ref(&mut self) -> Option<Box<Node<'a>>> {
        todo!()
    }
}

#[test]
fn test_gvar() {
    use crate::{loc::loc, nodes::Gvar, Node, RustParser};
    let mut parser = RustParser::new(b"$foo");
    assert_eq!(
        parser.parse(),
        Node::Gvar(Gvar {
            name: String::from("$foo"),
            expression_l: loc!(0, 4)
        })
    );
}

#[test]
fn test_back_ref() {
    use crate::{loc::loc, nodes::BackRef, Node, RustParser};
    let mut parser = RustParser::new(b"$+");
    assert_eq!(
        parser.parse(),
        Node::BackRef(BackRef {
            name: String::from("$+"),
            expression_l: loc!(0, 2)
        })
    );
}

#[test]
fn test_nth_ref() {
    use crate::{loc::loc, nodes::NthRef, Node, RustParser};
    let mut parser = RustParser::new(b"$1");
    assert_eq!(
        parser.parse(),
        Node::NthRef(NthRef {
            name: String::from("$1"),
            expression_l: loc!(0, 2)
        })
    );
}
