use crate::lexer::strings::types::Interpolation;

pub(crate) trait HasInterpolation {
    fn interpolation(&self) -> &Interpolation;
    fn interpolation_mut(&mut self) -> &mut Interpolation;
}
