use crate::token::Token;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum StringExtendAction {
    EmitToken { token: Token },
    FoundStringEnd { token: Token },
    FoundInterpolation { token: Token },
    EmitEOF,
    ReadInterpolatedContent,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum NextAction {
    NoAction,
    OneAction(StringExtendAction),
    TwoActions(StringExtendAction, StringExtendAction),
}

impl<'a> NextAction {
    pub(crate) fn take(&mut self) -> Option<StringExtendAction> {
        match *self {
            Self::NoAction => None,
            Self::OneAction(action) => {
                *self = Self::NoAction;
                Some(action)
            }
            Self::TwoActions(first, second) => {
                *self = Self::OneAction(second);
                Some(first)
            }
        }
    }

    pub(crate) fn add(&mut self, action: StringExtendAction) {
        match self {
            Self::NoAction => {
                *self = Self::OneAction(action);
            }
            Self::OneAction(first) => {
                *self = Self::TwoActions(*first, action);
            }
            Self::TwoActions(_, _) => {
                unreachable!("The queue of string extend actions supports only 2 elements")
            }
        }
    }
}

impl Default for NextAction {
    fn default() -> Self {
        Self::NoAction
    }
}
