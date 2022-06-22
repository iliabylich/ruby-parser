use crate::{
    builder::{CString, Constructor},
    lexer::buffer::Buffer,
    nodes::*,
    Loc, Node, Token,
};
use std::ffi::c_void;

pub(crate) struct Builder<C: Constructor = RustConstructor> {
    phantom: std::marker::PhantomData<C>,
}
pub struct RustConstructor;

impl Constructor for RustConstructor {
    fn gvar_node(name: CString, loc: Loc) -> *mut c_void {
        Box::into_raw(Box::new(Node::Gvar(Gvar {
            name: name.into(),
            expression_l: loc,
        }))) as *mut c_void
    }

    fn back_ref_node(name: CString, loc: Loc) -> *mut c_void {
        Box::into_raw(Box::new(Node::BackRef(BackRef {
            name: name.into(),
            expression_l: loc,
        }))) as *mut c_void
    }

    fn nth_ref_node(name: CString, loc: Loc) -> *mut c_void {
        Box::into_raw(Box::new(Node::NthRef(NthRef {
            name: name.into(),
            expression_l: loc,
        }))) as *mut c_void
    }
}

impl<C: Constructor> Builder<C> {
    pub(crate) fn gvar<'a>(token: Token<'a>, buffer: &Buffer) -> Box<Node<'a>> {
        let loc = token.loc();
        let name = CString::from(buffer.slice(loc.start, loc.end).unwrap());
        unsafe { Box::from_raw(C::gvar_node(name, loc) as *mut Node<'a>) }
    }

    pub(crate) fn back_ref<'a>(token: Token<'a>) -> Box<Node<'a>> {
        let expression_l = token.loc();
        Box::new(Node::BackRef(BackRef {
            name: String::from("foo"),
            expression_l,
        }))
    }

    pub(crate) fn alias<'a>(
        alias_t: Token<'a>,
        to: Box<Node<'a>>,
        from: Box<Node<'a>>,
    ) -> Box<Node<'a>> {
        let keyword_l = alias_t.loc();
        let expression_l = keyword_l.join(from.expression());
        Box::new(Node::Alias(Alias {
            to,
            from,
            keyword_l,
            expression_l,
        }))
    }
}
