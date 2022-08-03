use crate::{
    builder::{CString, Constructor},
    nodes::{BackRef, Gvar, NthRef},
    Loc, Node,
};
use std::ffi::c_void;

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
