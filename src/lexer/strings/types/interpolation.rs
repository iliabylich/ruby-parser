#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Interpolation {
    pub(crate) enabled: bool,
    pub(crate) curly_nest: usize,
}

impl Interpolation {
    pub(crate) fn new(curly_nest: usize) -> Self {
        Self {
            enabled: false,
            curly_nest,
        }
    }
}
