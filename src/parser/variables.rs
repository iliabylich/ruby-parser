use super::*;
use crate::builder::{Builder, Constructor};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn parse_gvar(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::tGVAR)
            .map(|gvar_t| Builder::<C>::gvar(gvar_t, self.lexer.buffer.for_lookahead()))
    }

    pub(crate) fn parse_back_ref(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::tBACK_REF)
            .map(|back_ref_t| Builder::<C>::back_ref(back_ref_t, self.lexer.buffer.for_lookahead()))
    }

    pub(crate) fn parse_nth_ref(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::tNTH_REF)
            .map(|nth_ref_t| Builder::<C>::nth_ref(nth_ref_t, self.lexer.buffer.for_lookahead()))
    }

    pub(crate) fn parse_lvar(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::tIDENTIFIER)
            .map(|ident_t| Builder::<C>::lvar(ident_t, self.lexer.buffer.for_lookahead()))
    }

    pub(crate) fn parse_ivar(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::tIVAR)
            .map(|ident_t| Builder::<C>::ivar(ident_t, self.lexer.buffer.for_lookahead()))
    }

    pub(crate) fn parse_cvar(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::tCVAR)
            .map(|ident_t| Builder::<C>::cvar(ident_t, self.lexer.buffer.for_lookahead()))
    }

    pub(crate) fn parse_t_const(&mut self) -> Option<Box<Node<'a>>> {
        self.try_token(TokenValue::tCONSTANT)
            .map(|ident_t| Builder::<C>::const_(ident_t, self.lexer.buffer.for_lookahead()))
    }
}

#[test]
fn test_gvar() {
    use crate::{loc::loc, nodes::Gvar, string_content::StringContent, Node, RustParser};
    let mut parser = RustParser::new(b"$foo");
    assert_eq!(
        parser.parse(),
        Node::Gvar(Gvar {
            name: StringContent::from("$foo"),
            expression_l: loc!(0, 4)
        })
    );
}

#[test]
fn test_back_ref() {
    use crate::{loc::loc, nodes::BackRef, string_content::StringContent, Node, RustParser};
    let mut parser = RustParser::new(b"$+");
    assert_eq!(
        parser.parse(),
        Node::BackRef(BackRef {
            name: StringContent::from("$+"),
            expression_l: loc!(0, 2)
        })
    );
}

#[test]
fn test_nth_ref() {
    use crate::{loc::loc, nodes::NthRef, string_content::StringContent, Node, RustParser};
    let mut parser = RustParser::new(b"$1");
    assert_eq!(
        parser.parse(),
        Node::NthRef(NthRef {
            name: StringContent::from("$1"),
            expression_l: loc!(0, 2)
        })
    );
}
