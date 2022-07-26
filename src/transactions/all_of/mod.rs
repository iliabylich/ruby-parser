mod macros;
use macros::gen_all_of;

mod seq_error;
use seq_error::SeqError;

use crate::transactions::{ParseError, StepData};

pub(crate) struct AllOf0 {
    name: &'static str,
}
impl AllOf0 {
    pub(crate) fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub(crate) fn and<A, F>(self, f: F) -> AllOf1<A>
    where
        F: FnOnce() -> Result<A, ParseError>,
        StepData: From<A>,
    {
        let Self { name } = self;
        match f() {
            Ok(a) => AllOf1 { name, inner: Ok(a) },
            Err(error) => AllOf1 {
                name,
                inner: Err(SeqError {
                    steps: vec![],
                    error,
                }),
            },
        }
    }
}

gen_all_of!(AllOf1; generic = A; fields = a; next = AllOf2);
gen_all_of!(AllOf2; generic = A, B; fields = a, b; next = AllOf3);
gen_all_of!(AllOf3; generic = A, B, C; fields = a, b, c; next = AllOf4);
gen_all_of!(AllOf4; generic = A, B, C, D; fields = a, b, c, d; next = AllOf5);
gen_all_of!(AllOf5; generic = A, B, C, D, E; fields = a, b, c, d, e; next = AllOf6);
gen_all_of!(AllOf6; generic = A, B, C, D, E, F; fields = a, b, c, d, e, f; next = AllOf7);

#[allow(dead_code)]
pub(crate) struct AllOf7<A1, A2, A3, A4, A5, A6, A7> {
    name: &'static str,
    inner: Result<(A1, A2, A3, A4, A5, A6, A7), SeqError>,
}
