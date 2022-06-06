mod uninitialized;
pub(crate) use uninitialized::Uninitialized;

mod integer_prefix;
pub(crate) use integer_prefix::IntegerPrefix;

mod integer;
pub(crate) use integer::Integer;

mod float;
pub(crate) use float::Float;

mod imaginary;
pub(crate) use imaginary::Imaginary;

mod rational;
pub(crate) use rational::Rational;

macro_rules! try_sub_parser {
    ($fn:expr, $buffer:expr, $start:expr, $number:expr) => {
        if let Some(len) = $fn($buffer, $start) {
            $buffer.set_pos($buffer.pos() + len.get());

            $number.end += len.get();
            true
        } else {
            false
        }
    };
}
pub(crate) use try_sub_parser;

#[derive(Debug, Clone, Copy)]
pub(crate) enum State {
    Uninitialized(Uninitialized),
    IntegerPrefix(IntegerPrefix),
    Integer(Integer),
    Float(Float),
    Imaginary(Imaginary),
    Rational(Rational),
}
