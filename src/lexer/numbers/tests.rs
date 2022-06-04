use crate::lexer::assert_lex;

mod prefix_tests {
    use super::*;

    assert_lex!(
        test_tINTEGER_hexadecimal_prefix_lower,
        b"0x42",
        tINTEGER,
        b"0x42",
        0..4
    );
    assert_lex!(
        test_tINTEGER_hexadecimal_prefix_upper,
        b"0X42",
        tINTEGER,
        b"0X42",
        0..4
    );
    assert_lex!(
        test_tINTEGER_binary_prefix_lower,
        b"0b1010",
        tINTEGER,
        b"0b1010",
        0..6
    );
    assert_lex!(
        test_tINTEGER_binary_prefix_upper,
        b"0B1010",
        tINTEGER,
        b"0B1010",
        0..6
    );
    assert_lex!(
        test_tINTEGER_decimal_prefix_lower,
        b"0d192",
        tINTEGER,
        b"0d192",
        0..5
    );
    assert_lex!(
        test_tINTEGER_decimal_prefix_upper,
        b"0D192",
        tINTEGER,
        b"0D192",
        0..5
    );
    assert_lex!(
        test_tINTEGER_octal_prefix_underscore,
        b"0_12",
        tINTEGER,
        b"0_12",
        0..4
    );
    assert_lex!(
        test_tINTEGER_octal_prefix_lower,
        b"0o12",
        tINTEGER,
        b"0o12",
        0..4
    );
    assert_lex!(
        test_tINTEGER_octal_prefix_upper,
        b"0O12",
        tINTEGER,
        b"0O12",
        0..4
    );
}

mod basic_decimal_tests {
    use super::*;

    assert_lex!(test_tINTEGER_decimal, b"42", tINTEGER, b"42", 0..2);
}

mod underscore_tests {
    use super::*;

    assert_lex!(
        test_tINTEGER_hexadecimal_with_underscores,
        b"0x1_2",
        tINTEGER,
        b"0x1_2",
        0..5
    );

    assert_lex!(
        test_tINTEGER_binary_with_underscores,
        b"0b1_0",
        tINTEGER,
        b"0b1_0",
        0..5
    );

    assert_lex!(
        test_tINTEGER_decimal_with_underscores,
        b"0d8_9",
        tINTEGER,
        b"0d8_9",
        0..5
    );

    assert_lex!(
        test_tINTEGER_octal_with_underscores,
        b"02_7",
        tINTEGER,
        b"02_7",
        0..4
    );
}

mod trailing_underscore_tests {
    use super::*;

    assert_lex!(
        test_tINTEGER_hexadecimal_with_trailing_underscore,
        b"0x1_",
        tINTEGER,
        b"0x1",
        0..3
    );

    assert_lex!(
        test_tINTEGER_binary_with_trailing_underscore,
        b"0b1_",
        tINTEGER,
        b"0b1",
        0..3
    );

    assert_lex!(
        test_tINTEGER_decimal_with_trailing_underscore,
        b"0d8_",
        tINTEGER,
        b"0d8",
        0..3
    );

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
    assert_lex!(test_tFLOAT_plain, b"12.34", tFLOAT, b"12.34", 0..5);

    assert_lex!(test_tFLOAT_int_lower_e, b"1e3", tFLOAT, b"1e3", 0..3);
    assert_lex!(test_tFLOAT_int_plus_lower_e, b"1e+3", tFLOAT, b"1e+3", 0..4);
    assert_lex!(
        test_tFLOAT_int_minus_lower_e,
        b"1e-3",
        tFLOAT,
        b"1e-3",
        0..4
    );
    assert_lex!(test_tFLOAT_float_lower_e, b"1.2e3", tFLOAT, b"1.2e3", 0..5);
    assert_lex!(
        test_tFLOAT_float_plus_lower_e,
        b"1.2e+3",
        tFLOAT,
        b"1.2e+3",
        0..6
    );
    assert_lex!(
        test_tFLOAT_float_minus_lower_e,
        b"1.2e-3",
        tFLOAT,
        b"1.2e-3",
        0..6
    );

    assert_lex!(test_tFLOAT_int_upper_e, b"1E3", tFLOAT, b"1E3", 0..3);
    assert_lex!(test_tFLOAT_int_plus_upper_e, b"1E+3", tFLOAT, b"1E+3", 0..4);
    assert_lex!(
        test_tFLOAT_int_minus_upper_e,
        b"1E-3",
        tFLOAT,
        b"1E-3",
        0..4
    );
    assert_lex!(test_tFLOAT_float_upper_e, b"1.2E3", tFLOAT, b"1.2E3", 0..5);
    assert_lex!(
        test_tFLOAT_float_plus_upper_e,
        b"1.2E+3",
        tFLOAT,
        b"1.2E+3",
        0..6
    );
    assert_lex!(
        test_tFLOAT_float_minus_upper_e,
        b"1.2E-3",
        tFLOAT,
        b"1.2E-3",
        0..6
    );
}

mod rational_tests {
    use super::*;

    assert_lex!(test_tRATIONAL_int, b"1r", tRATIONAL, b"1r", 0..2);
    assert_lex!(test_tRATIONAL_float, b"1.2r", tRATIONAL, b"1.2r", 0..4);
}

mod imaginary_tests {
    use super::*;

    assert_lex!(test_tIMAGINARY_int, b"1i", tIMAGINARY, b"1i", 0..2);
    assert_lex!(test_tIMAGINARY_float, b"1.2i", tIMAGINARY, b"1.2i", 0..4);
}

mod rational_and_imaginary_tests {
    use super::*;

    assert_lex!(
        test_tIMAGINARY_rational_int,
        b"1ri",
        tIMAGINARY,
        b"1ri",
        0..3
    );
    assert_lex!(
        test_tIMAGINARY_rational_float,
        b"1.2ri",
        tIMAGINARY,
        b"1.2ri",
        0..5
    );
}
