#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Interpolation {
    Available { enabled: bool, curly_nest: usize },
    Disabled,
}

impl Interpolation {
    pub(crate) fn available(curly_nest: usize) -> Self {
        Self::Available {
            enabled: false,
            curly_nest,
        }
    }
}
