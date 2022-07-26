use crate::{
    builder::{Builder, Constructor},
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_gvar(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::tGVAR)
            .map(|gvar_t| Builder::<C>::gvar(gvar_t, self.buffer()))
    }

    pub(crate) fn try_back_ref(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::tBACK_REF)
            .map(|back_ref_t| Builder::<C>::back_ref(back_ref_t, self.buffer()))
    }

    pub(crate) fn try_nth_ref(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::tNTH_REF)
            .map(|nth_ref_t| Builder::<C>::nth_ref(nth_ref_t, self.buffer()))
    }

    pub(crate) fn try_lvar(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::tIDENTIFIER)
            .map(|ident_t| Builder::<C>::lvar(ident_t, self.buffer()))
    }

    pub(crate) fn try_ivar(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::tIVAR)
            .map(|ident_t| Builder::<C>::ivar(ident_t, self.buffer()))
    }

    pub(crate) fn try_cvar(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::tCVAR)
            .map(|ident_t| Builder::<C>::cvar(ident_t, self.buffer()))
    }

    pub(crate) fn try_t_const(&mut self) -> ParseResult<Box<Node>> {
        self.try_token(TokenKind::tCONSTANT)
            .map(|ident_t| Builder::<C>::const_(ident_t, self.buffer()))
    }

    pub(crate) fn try_const_or_identifier(&mut self) -> ParseResult<Token> {
        self.one_of("const or identifier")
            .or_else(|| self.try_token(TokenKind::tCONSTANT))
            .or_else(|| self.try_token(TokenKind::tIDENTIFIER))
            .stop()
    }
}

#[test]
fn test_gvar() {
    use crate::{loc::loc, nodes::Gvar, string_content::StringContent, Node, RustParser};
    let mut parser = RustParser::new(b"$foo");
    assert_eq!(
        parser.try_gvar(),
        Ok(Box::new(Node::Gvar(Gvar {
            name: StringContent::from("$foo"),
            expression_l: loc!(0, 4)
        })))
    );
}

#[test]
fn test_back_ref() {
    use crate::{loc::loc, nodes::BackRef, string_content::StringContent, Node, RustParser};
    let mut parser = RustParser::new(b"$+");
    assert_eq!(
        parser.try_back_ref(),
        Ok(Box::new(Node::BackRef(BackRef {
            name: StringContent::from("$+"),
            expression_l: loc!(0, 2)
        })))
    );
}

#[test]
fn test_nth_ref() {
    use crate::{loc::loc, nodes::NthRef, string_content::StringContent, Node, RustParser};
    let mut parser = RustParser::new(b"$1");
    assert_eq!(
        parser.try_nth_ref(),
        Ok(Box::new(Node::NthRef(NthRef {
            name: StringContent::from("1"),
            expression_l: loc!(0, 2)
        })))
    );
}

#[test]
fn test_lvar() {
    use crate::{loc::loc, nodes::Lvar, string_content::StringContent, Node, RustParser};
    let mut parser = RustParser::new(b"foo");
    assert_eq!(
        parser.try_lvar(),
        Ok(Box::new(Node::Lvar(Lvar {
            name: StringContent::from("foo"),
            expression_l: loc!(0, 3)
        })))
    );
}

#[test]
fn test_ivar() {
    use crate::{loc::loc, nodes::Ivar, string_content::StringContent, Node, RustParser};
    let mut parser = RustParser::new(b"@foo");
    assert_eq!(
        parser.try_ivar(),
        Ok(Box::new(Node::Ivar(Ivar {
            name: StringContent::from("@foo"),
            expression_l: loc!(0, 4)
        })))
    );
}

#[test]
fn test_cvar() {
    use crate::{loc::loc, nodes::Cvar, string_content::StringContent, Node, RustParser};
    let mut parser = RustParser::new(b"@@foo");
    assert_eq!(
        parser.try_cvar(),
        Ok(Box::new(Node::Cvar(Cvar {
            name: StringContent::from("@@foo"),
            expression_l: loc!(0, 5)
        })))
    );
}

#[test]
fn test_const() {
    use crate::{loc::loc, nodes::Const, string_content::StringContent, Node, RustParser};
    let mut parser = RustParser::new(b"Foo");
    assert_eq!(
        parser.try_t_const(),
        Ok(Box::new(Node::Const(Const {
            name: StringContent::from("Foo"),
            scope: None,
            double_colon_l: None,
            name_l: loc!(0, 3),
            expression_l: loc!(0, 3)
        })))
    );
}
