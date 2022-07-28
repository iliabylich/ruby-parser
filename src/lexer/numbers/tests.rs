use crate::{testing::assert_lex, token::token};

mod prefix_tests {
    use super::*;

    #[test]
    fn test_tINTEGER_hexadecimal_prefix_lower() {
        assert_lex!(b"0x42", token!(tINTEGER, loc!(0, 4)));
    }
    #[test]
    fn test_tINTEGER_hexadecimal_prefix_upper() {
        assert_lex!(b"0X42", token!(tINTEGER, loc!(0, 4)));
    }
    #[test]
    fn test_tINTEGER_binary_prefix_lower() {
        assert_lex!(b"0b1010", token!(tINTEGER, loc!(0, 6)));
    }
    #[test]
    fn test_tINTEGER_binary_prefix_upper() {
        assert_lex!(b"0B1010", token!(tINTEGER, loc!(0, 6)));
    }
    #[test]
    fn test_tINTEGER_decimal_prefix_lower() {
        assert_lex!(b"0d192", token!(tINTEGER, loc!(0, 5)));
    }
    #[test]
    fn test_tINTEGER_decimal_prefix_upper() {
        assert_lex!(b"0D192", token!(tINTEGER, loc!(0, 5)));
    }
    #[test]
    fn test_tINTEGER_octal_prefix_underscore() {
        assert_lex!(b"0_12", token!(tINTEGER, loc!(0, 4)));
    }
    #[test]
    fn test_tINTEGER_octal_prefix_lower() {
        assert_lex!(b"0o12", token!(tINTEGER, loc!(0, 4)));
    }
    #[test]
    fn test_tINTEGER_octal_prefix_upper() {
        assert_lex!(b"0O12", token!(tINTEGER, loc!(0, 4)));
    }
}

mod basic_decimal_tests {
    use super::*;

    #[test]
    fn test_tINTEGER_decimal() {
        assert_lex!(b"42", token!(tINTEGER, loc!(0, 2)));
    }
}

mod underscore_tests {
    use super::*;

    #[test]
    fn test_tINTEGER_hexadecimal_with_underscores() {
        assert_lex!(b"0x1_2", token!(tINTEGER, loc!(0, 5)));
    }

    #[test]
    fn test_tINTEGER_binary_with_underscores() {
        assert_lex!(b"0b1_0", token!(tINTEGER, loc!(0, 5)));
    }

    #[test]
    fn test_tINTEGER_decimal_with_underscores() {
        assert_lex!(b"0d8_9", token!(tINTEGER, loc!(0, 5)));
    }

    #[test]
    fn test_tINTEGER_octal_with_underscores() {
        assert_lex!(b"02_7", token!(tINTEGER, loc!(0, 4)));
    }
}

mod trailing_underscore_tests {
    use super::*;

    #[test]
    fn test_tINTEGER_hexadecimal_with_trailing_underscore() {
        assert_lex!(b"0x1_", token!(tINTEGER, loc!(0, 3)));
    }

    #[test]
    fn test_tINTEGER_binary_with_trailing_underscore() {
        assert_lex!(b"0b1_", token!(tINTEGER, loc!(0, 3)));
    }

    #[test]
    fn test_tINTEGER_decimal_with_trailing_underscore() {
        assert_lex!(b"0d8_", token!(tINTEGER, loc!(0, 3)));
    }

    // TODO: this test should report "trailing `_' in number",
    //       currently it panics
    // assert_lex!(
    //     test_tINTEGER_octal_with_trailing_underscore,
    //     b"02_",
    //     tINTEGER,
    //     b"02",
    //     0..2
    // );
}

mod float_tests {
    use super::*;
    #[test]
    fn test_tFLOAT_plain() {
        assert_lex!(b"12.34", token!(tFLOAT, loc!(0, 5)));
    }

    #[test]
    fn test_tFLOAT_int_lower_e() {
        assert_lex!(b"1e3", token!(tFLOAT, loc!(0, 3)));
    }
    #[test]
    fn test_tFLOAT_int_plus_lower_e() {
        assert_lex!(b"1e+3", token!(tFLOAT, loc!(0, 4)));
    }
    #[test]
    fn test_tFLOAT_int_minus_lower_e() {
        assert_lex!(b"1e-3", token!(tFLOAT, loc!(0, 4)));
    }
    #[test]
    fn test_tFLOAT_float_lower_e() {
        assert_lex!(b"1.2e3", token!(tFLOAT, loc!(0, 5)));
    }
    #[test]
    fn test_tFLOAT_float_plus_lower_e() {
        assert_lex!(b"1.2e+3", token!(tFLOAT, loc!(0, 6)));
    }
    #[test]
    fn test_tFLOAT_float_minus_lower_e() {
        assert_lex!(b"1.2e-3", token!(tFLOAT, loc!(0, 6)));
    }

    #[test]
    fn test_tFLOAT_int_upper_e() {
        assert_lex!(b"1E3", token!(tFLOAT, loc!(0, 3)));
    }
    #[test]
    fn test_tFLOAT_int_plus_upper_e() {
        assert_lex!(b"1E+3", token!(tFLOAT, loc!(0, 4)));
    }
    #[test]
    fn test_tFLOAT_int_minus_upper_e() {
        assert_lex!(b"1E-3", token!(tFLOAT, loc!(0, 4)));
    }
    #[test]
    fn test_tFLOAT_float_upper_e() {
        assert_lex!(b"1.2E3", token!(tFLOAT, loc!(0, 5)));
    }
    #[test]
    fn test_tFLOAT_float_plus_upper_e() {
        assert_lex!(b"1.2E+3", token!(tFLOAT, loc!(0, 6)));
    }
    #[test]
    fn test_tFLOAT_float_minus_upper_e() {
        assert_lex!(b"1.2E-3", token!(tFLOAT, loc!(0, 6)));
    }
}

mod rational_tests {
    use super::*;

    #[test]
    fn test_tRATIONAL_int() {
        assert_lex!(b"1r", token!(tRATIONAL, loc!(0, 2)));
    }
    #[test]
    fn test_tRATIONAL_float() {
        assert_lex!(b"1.2r", token!(tRATIONAL, loc!(0, 4)));
    }
}

mod imaginary_tests {
    use super::*;

    #[test]
    fn test_tIMAGINARY_int() {
        assert_lex!(b"1i", token!(tIMAGINARY, loc!(0, 2)));
    }
    #[test]
    fn test_tIMAGINARY_float() {
        assert_lex!(b"1.2i", token!(tIMAGINARY, loc!(0, 4)));
    }
}

mod rational_and_imaginary_tests {
    use super::*;

    #[test]
    fn test_tIMAGINARY_rational_int() {
        assert_lex!(b"1ri", token!(tIMAGINARY, loc!(0, 3)));
    }
    #[test]
    fn test_tIMAGINARY_rational_float() {
        assert_lex!(b"1.2ri", token!(tIMAGINARY, loc!(0, 5)));
    }
}
