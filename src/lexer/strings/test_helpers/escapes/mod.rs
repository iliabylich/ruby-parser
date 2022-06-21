mod slash_u;
pub(crate) use slash_u::*;

mod slash_octal;
pub(crate) use slash_octal::*;

mod slash_x;
pub(crate) use slash_x::*;

mod slash_meta_control;
pub(crate) use slash_meta_control::*;

mod slash_byte;
pub(crate) use slash_byte::*;

macro_rules! assert_emits_escape_sequence {
    (literal = $literal:expr) => {
        assert_emits_escaped_slash_u!(literal = $literal);
        assert_emits_escaped_slash_octal!(literal = $literal);
        assert_emits_escaped_slash_x!(literal = $literal);
        assert_emits_escaped_slash_meta_control!(literal = $literal);
        assert_emits_escaped_slash_byte!(literal = $literal);
    };
}
pub(crate) use assert_emits_escape_sequence;

macro_rules! assert_ignores_escape_sequence {
    (literal = $literal:expr) => {
        assert_ignores_escaped_slash_u!(literal = $literal);
        assert_ignores_escaped_slash_octal!(literal = $literal);
        assert_ignores_escaped_slash_x!(literal = $literal);
        assert_ignores_escaped_slash_meta_control!(literal = $literal);
        assert_ignores_escaped_slash_byte!(literal = $literal);
    };
}
pub(crate) use assert_ignores_escape_sequence;
