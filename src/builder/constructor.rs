use crate::{builder::CString, Loc};
use std::ffi::c_void;

pub trait Constructor {
    // Literals
    // fn nil_node(nil_l: Loc) -> *mut c_void;
    // fn true_node(true_l: Loc) -> *mut c_void;
    // fn false_node(false_l: Loc) -> *mut c_void;

    // Numerics
    // fn integer_node(integer_l: Loc) -> *mut c_void;
    // fn float_node(float_l: Loc) -> *mut c_void;
    // fn rational_node(rational_l: Loc) -> *mut c_void;
    // fn complex_node(complex_l: Loc) -> *mut c_void;

    // Special constants
    // fn __line__(line_l: Loc) -> *mut c_void;
    // fn __file__(file_l: Loc) -> *mut c_void;
    // fn __encoding__(encoding_l: Loc) -> *mut c_void;

    // Strings

    // Symbols

    // Executable string

    // Regular expressions

    // Arrays

    // Hashes

    // Ranges

    // Access
    // fn self_(name: *mut u8, loc: Loc) -> *mut c_void;
    // fn lvar(name: *mut u8, loc: Loc) -> *mut c_void;
    // fn ivar(name: *mut u8, loc: Loc) -> *mut c_void;
    fn gvar_node(name: CString, loc: Loc) -> *mut c_void;
    // fn cvar(name: *mut u8, loc: Loc) -> *mut c_void;
    fn back_ref_node(name: CString, loc: Loc) -> *mut c_void;
    fn nth_ref_node(name: CString, loc: Loc) -> *mut c_void;

    // Assignments

    // Class and module definition

    // Method (un)definition

    // Formatl arguments

    // Method calls

    // Logical operations: and, or

    // Conditionals

    // Case matching

    // Loops

    // Keywords

    // BEGIN, END

    // Exception handling

    // Expression grouping

    // Pattern matching
}
