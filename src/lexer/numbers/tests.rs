use crate::lexer::assert_lex;

mod prefix_tests {
    use super::*;

    assert_lex!(
        test_tINTEGER_hexadecimal_prefix_lower,
        b"0x42",
        tINTEGER,
        None,
        0..4
    );
    assert_lex!(
        test_tINTEGER_hexadecimal_prefix_upper,
        b"0X42",
        tINTEGER,
        None,
        0..4
    );
    assert_lex!(
        test_tINTEGER_binary_prefix_lower,
        b"0b1010",
        tINTEGER,
        None,
        0..6
    );
    assert_lex!(
        test_tINTEGER_binary_prefix_upper,
        b"0B1010",
        tINTEGER,
        None,
        0..6
    );
    assert_lex!(
        test_tINTEGER_decimal_prefix_lower,
        b"0d192",
        tINTEGER,
        None,
        0..5
    );
    assert_lex!(
        test_tINTEGER_decimal_prefix_upper,
        b"0D192",
        tINTEGER,
        None,
        0..5
    );
    assert_lex!(
        test_tINTEGER_octal_prefix_underscore,
        b"0_12",
        tINTEGER,
        None,
        0..4
    );
    assert_lex!(
        test_tINTEGER_octal_prefix_lower,
        b"0o12",
        tINTEGER,
        None,
        0..4
    );
    assert_lex!(
        test_tINTEGER_octal_prefix_upper,
        b"0O12",
        tINTEGER,
        None,
        0..4
    );
}

mod basic_decimal_tests {
    use super::*;

    assert_lex!(test_tINTEGER_decimal, b"42", tINTEGER, None, 0..2);
}

mod underscore_tests {
    use super::*;

    assert_lex!(
        test_tINTEGER_hexadecimal_with_underscores,
        b"0x1_2",
        tINTEGER,
        None,
        0..5
    );

    assert_lex!(
        test_tINTEGER_binary_with_underscores,
        b"0b1_0",
        tINTEGER,
        None,
        0..5
    );

    assert_lex!(
        test_tINTEGER_decimal_with_underscores,
        b"0d8_9",
        tINTEGER,
        None,
        0..5
    );

    assert_lex!(
        test_tINTEGER_octal_with_underscores,
        b"02_7",
        tINTEGER,
        None,
        0..4
    );
}

mod trailing_underscore_tests {
    use super::*;

    assert_lex!(
        test_tINTEGER_hexadecimal_with_trailing_underscore,
        b"0x1_",
        tINTEGER,
        None,
        0..3
    );

    assert_lex!(
        test_tINTEGER_binary_with_trailing_underscore,
        b"0b1_",
        tINTEGER,
        None,
        0..3
    );

    assert_lex!(
        test_tINTEGER_decimal_with_trailing_underscore,
        b"0d8_",
        tINTEGER,
        None,
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
    assert_lex!(test_tFLOAT_plain, b"12.34", tFLOAT, None, 0..5);

    assert_lex!(test_tFLOAT_int_lower_e, b"1e3", tFLOAT, None, 0..3);
    assert_lex!(test_tFLOAT_int_plus_lower_e, b"1e+3", tFLOAT, None, 0..4);
    assert_lex!(test_tFLOAT_int_minus_lower_e, b"1e-3", tFLOAT, None, 0..4);
    assert_lex!(test_tFLOAT_float_lower_e, b"1.2e3", tFLOAT, None, 0..5);
    assert_lex!(
        test_tFLOAT_float_plus_lower_e,
        b"1.2e+3",
        tFLOAT,
        None,
        0..6
    );
    assert_lex!(
        test_tFLOAT_float_minus_lower_e,
        b"1.2e-3",
        tFLOAT,
        None,
        0..6
    );

    assert_lex!(test_tFLOAT_int_upper_e, b"1E3", tFLOAT, None, 0..3);
    assert_lex!(test_tFLOAT_int_plus_upper_e, b"1E+3", tFLOAT, None, 0..4);
    assert_lex!(test_tFLOAT_int_minus_upper_e, b"1E-3", tFLOAT, None, 0..4);
    assert_lex!(test_tFLOAT_float_upper_e, b"1.2E3", tFLOAT, None, 0..5);
    assert_lex!(
        test_tFLOAT_float_plus_upper_e,
        b"1.2E+3",
        tFLOAT,
        None,
        0..6
    );
    assert_lex!(
        test_tFLOAT_float_minus_upper_e,
        b"1.2E-3",
        tFLOAT,
        None,
        0..6
    );
}

mod rational_tests {
    use super::*;

    assert_lex!(test_tRATIONAL_int, b"1r", tRATIONAL, None, 0..2);
    assert_lex!(test_tRATIONAL_float, b"1.2r", tRATIONAL, None, 0..4);
}

mod imaginary_tests {
    use super::*;

    assert_lex!(test_tIMAGINARY_int, b"1i", tIMAGINARY, None, 0..2);
    assert_lex!(test_tIMAGINARY_float, b"1.2i", tIMAGINARY, None, 0..4);
}

mod rational_and_imaginary_tests {
    use super::*;

    assert_lex!(test_tIMAGINARY_rational_int, b"1ri", tIMAGINARY, None, 0..3);
    assert_lex!(
        test_tIMAGINARY_rational_float,
        b"1.2ri",
        tIMAGINARY,
        None,
        0..5
    );
}
