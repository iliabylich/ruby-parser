use crate::lexer::strings::action::NextAction;

mod string;
pub(crate) use string::String;

mod symbol;
pub(crate) use symbol::Symbol;

mod heredoc;
pub(crate) use heredoc::Heredoc;

mod regexp;
pub(crate) use regexp::Regexp;

mod sym_array;
pub(crate) use sym_array::SymArray;

mod word_array;
pub(crate) use word_array::WordArray;

pub(crate) trait StringLiteralAttributes<'a> {
    fn supports_interpolation(&self) -> bool;

    fn currently_in_interpolation(&self) -> bool;
    fn currently_in_interpolation_mut(&mut self) -> &mut bool;

    fn interpolation_started_with_curly_level(&self) -> usize;

    fn ends_with(&self) -> &'a [u8];

    fn next_action(&self) -> NextAction;
    fn next_action_mut(&mut self) -> &mut NextAction;
}

macro_rules! generate_default_string_literal_impl {
    ($type:tt) => {
        impl<'a> crate::lexer::strings::types::StringLiteralAttributes<'a> for $type<'a> {
            fn supports_interpolation(&self) -> bool {
                self.supports_interpolation
            }

            fn currently_in_interpolation(&self) -> bool {
                self.currently_in_interpolation
            }

            fn currently_in_interpolation_mut(&mut self) -> &mut bool {
                &mut self.currently_in_interpolation
            }

            fn interpolation_started_with_curly_level(&self) -> usize {
                self.interpolation_started_with_curly_level
            }

            fn ends_with(&self) -> &'a [u8] {
                self.ends_with
            }

            fn next_action(&self) -> NextAction {
                self.next_action
            }

            fn next_action_mut(&mut self) -> &mut NextAction {
                &mut self.next_action
            }
        }
    };
}
pub(crate) use generate_default_string_literal_impl;
