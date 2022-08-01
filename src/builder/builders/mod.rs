use crate::builder::{Constructor, RustConstructor};

mod helpers;

mod access;
mod arguments;
mod arrays;
mod assignments;
mod begin_end;
mod case_matching;
mod class_and_module;
mod conditionals;
mod exceptions;
mod executable_string;
mod expr_grouping;
mod hashes;
mod keywords;
mod logical_ops;
mod loops;
mod method_calls;
mod method_definition;
mod numerics;
mod pattern_matching;
mod ranges;
mod regexp;
mod singletons;
mod special_constants;
mod strings;
mod symbols;

pub(crate) struct Builder<C: Constructor = RustConstructor> {
    phantom: std::marker::PhantomData<C>,
}
