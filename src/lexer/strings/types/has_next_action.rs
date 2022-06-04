use crate::lexer::strings::action::NextAction;

pub(crate) trait HasNextAction {
    fn next_action_mut(&mut self) -> &mut NextAction;
}
