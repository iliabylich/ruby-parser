use crate::{
    builder::Builder,
    lexer::strings::{
        literal::StringLiteral,
        types::{Interpolation, Regexp as RegexpLiteral, StringInterp},
    },
    parser::{
        variables::{BackRef, Cvar, Gvar, Ivar},
        AtLeastOnce, ExactToken, FnameT, ParseResult, Repeat1, Rule, SeparatedBy, SimpleNumericT,
    },
    token::token,
    Node, Parser, TokenKind,
};

pub(crate) struct Literal;
impl Rule for Literal {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        Numeric::starts_now(parser)
            || Symbol::starts_now(parser)
            || Strings::starts_now(parser)
            || XString::starts_now(parser)
            || Regexp::starts_now(parser)
            || Words::starts_now(parser)
            || QWords::starts_now(parser)
            || Symbols::starts_now(parser)
            || QSymbols::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if Numeric::starts_now(parser) {
            Numeric::parse(parser)
        } else if Symbol::starts_now(parser) {
            Symbol::parse(parser)
        } else if Strings::starts_now(parser) {
            Strings::parse(parser)
        } else if XString::starts_now(parser) {
            XString::parse(parser)
        } else if Regexp::starts_now(parser) {
            Regexp::parse(parser)
        } else if Words::starts_now(parser) {
            Words::parse(parser)
        } else if QWords::starts_now(parser) {
            QWords::parse(parser)
        } else if Symbols::starts_now(parser) {
            Symbols::parse(parser)
        } else if QSymbols::starts_now(parser) {
            QSymbols::parse(parser)
        } else {
            unreachable!()
        }
    }
}

struct Numeric;
impl Rule for Numeric {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tUMINUS_NUM) || SimpleNumericT::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let unary_t = if parser.current_token().is(TokenKind::tUMINUS_NUM) {
            Some(parser.take_token())
        } else {
            None
        };

        let mut number = if SimpleNumericT::starts_now(parser) {
            let numeric_t = SimpleNumericT::parse(parser).unwrap();

            match numeric_t.kind {
                TokenKind::tINTEGER => Builder::integer(numeric_t, parser.buffer()),
                TokenKind::tFLOAT => Builder::float(numeric_t, parser.buffer()),
                TokenKind::tRATIONAL => Builder::rational(numeric_t, parser.buffer()),
                TokenKind::tIMAGINARY => Builder::complex(numeric_t, parser.buffer()),
                _ => panic!("wrong token type"),
            }
        } else {
            panic!("expected numeric literal")
        };

        if let Some(unary_t) = unary_t {
            number = Builder::unary_num(unary_t, number, parser.buffer());
        }

        Ok(number)
    }
}

#[test]
fn test_integer_numeric() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Numeric, b"42", r#"s(:int, "42")"#);
    assert_parses_rule!(Numeric, b"-42", r#"s(:int, "-42")"#);
}

#[test]
fn test_float_numeric() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Numeric, b"42.5", r#"s(:float, "42.5")"#);
    assert_parses_rule!(Numeric, b"-42.5", r#"s(:float, "-42.5")"#);
}

#[test]
fn test_rational_numeric() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Numeric, b"42r", r#"s(:rational, "42r")"#);
    assert_parses_rule!(Numeric, b"-42r", r#"s(:rational, "-42r")"#);
}

#[test]
fn test_complex_numeric() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(Numeric, b"42i", r#"s(:complex, "42i")"#);
    assert_parses_rule!(Numeric, b"-42i", r#"s(:complex, "-42i")"#);
}

pub(crate) struct Symbol;
impl Rule for Symbol {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        SimpleSymbol::starts_now(parser) || QuotedSymbol::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if SimpleSymbol::starts_now(parser) {
            SimpleSymbol::parse(parser)
        } else if QuotedSymbol::starts_now(parser) {
            QuotedSymbol::parse(parser)
        } else {
            unreachable!()
        }
    }
}

struct SimpleSymbol;
impl Rule for SimpleSymbol {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tCOLON)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let colon_t = parser.take_token();

        let sym_t = if FnameT::starts_now(parser) {
            FnameT::parse(parser).unwrap()
        } else if parser.current_token().is_one_of([
            TokenKind::tIVAR,
            TokenKind::tCVAR,
            TokenKind::tGVAR,
        ]) {
            parser.take_token()
        } else {
            panic!("wrong token")
        };

        Ok(Builder::symbol(colon_t, sym_t, parser.buffer()))
    }
}

#[test]
fn test_simple_symbol() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(SimpleSymbol, b":foo", r#"s(:sym, "foo")"#);
    assert_parses_rule!(SimpleSymbol, b":@foo", r#"s(:sym, "@foo")"#);
    assert_parses_rule!(SimpleSymbol, b":@@foo", r#"s(:sym, "@@foo")"#);
    assert_parses_rule!(SimpleSymbol, b":$foo", r#"s(:sym, "$foo")"#);
}

struct QuotedSymbol;
impl Rule for QuotedSymbol {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tSYMBEG, TokenKind::tDSYMBEG])
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.take_token();

        let parts = StringContents::parse(parser).unwrap();

        let end_t = if parser.current_token().is(TokenKind::tSTRING_END) {
            parser.take_token()
        } else {
            panic!("wrong token type")
        };

        Ok(Builder::symbol_compose(begin_t, parts, end_t))
    }
}

#[test]
fn test_quoted_symbol() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(QuotedSymbol, b":'foo'", r#"s(:sym, "foo")"#);
    assert_parses_rule!(QuotedSymbol, b":\"foo\"", r#"s(:sym, "foo")"#);
    // assert_parses_rule!(QuotedSymbol, b":\"foo#{42}bar\"", r#"TODO"#);
}

struct Strings;
impl Rule for Strings {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tCHAR) || String1::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if parser.current_token().is(TokenKind::tCHAR) {
            let char_t = parser.take_token();
            Ok(Builder::character(char_t))
        } else {
            let parts = AtLeastOnce::<String1>::parse(parser)?;
            Ok(Builder::string_compose(None, parts, None))
        }
    }
}
#[test]
fn test_strings() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Strings,
        b"'foo' 'bar'",
        r#"
s(:dstr,
  s(:str, "foo"),
  s(:str, "bar"))
        "#
    );
}

struct String1;
impl Rule for String1 {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tSTRING_BEG)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.take_token();
        let parts = StringContents::parse(parser)?;
        let end_t = parser
            .expect_token(TokenKind::tSTRING_END)
            .expect("wrong token type");
        Ok(Builder::string_compose(Some(begin_t), parts, Some(end_t)))
    }
}

#[test]
fn test_string1() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(String1, b"'foo'", r#"s(:str, "foo")"#);
}

struct XString;
impl Rule for XString {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        let current_token = parser.current_token();

        if current_token.is(TokenKind::tXSTRING_BEG) {
            return true;
        }
        if current_token.is(TokenKind::tIDENTIFIER)
            && parser
                .buffer()
                .slice(current_token.loc.start, current_token.loc.end)
                == Some(b"`")
        {
            // starts with `, if next token is tIDENTIFIER we are good to go
            parser.lexer.lookahead_is_identifier()
        } else {
            false
        }
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.current_token();

        if begin_t.is(TokenKind::tIDENTIFIER) {
            // manually push XString literal in lexer (yes, only parser know it)
            parser
                .lexer
                .string_literals
                .push(StringLiteral::StringInterp(StringInterp::new(
                    Interpolation::new(parser.lexer.curly_nest),
                    b'`',
                    b'`',
                )));

            // override token
            let token = token!(TokenKind::tXSTRING_BEG, begin_t.loc);
            parser.lexer.tokens[parser.lexer.token_idx] = token;
        }
        parser.skip_token();

        let parts = StringContents::parse(parser)?;
        let end_t = parser
            .expect_token(TokenKind::tSTRING_END)
            .expect("wrong token type");
        Ok(Builder::xstring_compose(begin_t, parts, end_t))
    }
}
#[test]
fn test_xstring() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        XString,
        b"`foo`",
        r#"
s(:xstr,
  s(:str, "foo"))
        "#
    );
    assert_parses_rule!(
        XString,
        b"%x{foo}",
        r#"
s(:xstr,
  s(:str, "foo"))
        "#
    );
}

struct Regexp;
impl Rule for Regexp {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser
            .current_token()
            .is_one_of([TokenKind::tREGEXP_BEG, TokenKind::tDIVIDE])
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.current_token();

        if begin_t.is(TokenKind::tDIVIDE) {
            // manually push XString literal in lexer (yes, only parser know it)
            parser
                .lexer
                .string_literals
                .push(StringLiteral::Regexp(RegexpLiteral::new(
                    b'/',
                    b'/',
                    parser.lexer.curly_nest,
                )));

            // override token
            let token = token!(TokenKind::tREGEXP_BEG, begin_t.loc);
            parser.lexer.tokens[parser.lexer.token_idx] = token;
        }
        parser.skip_token();

        let parts = StringContents::parse(parser)?;
        let end_t = parser
            .expect_token(TokenKind::tSTRING_END)
            .expect("wrong token type");

        let options = Builder::regexp_options(&end_t, parser.buffer());
        Ok(Builder::regexp_compose(begin_t, parts, end_t, options))
    }
}
#[test]
fn test_regexp() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Regexp,
        b"/foo/",
        r#"
s(:regexp,
  s(:str, "foo"),
  s(:regopt))
        "#
    );
    assert_parses_rule!(
        Regexp,
        b"/foo/xmi",
        r#"
s(:regexp,
  s(:str, "foo"),
  s(:regopt, "i", "m", "x"))
        "#
    );
    assert_parses_rule!(
        Regexp,
        b"%r{foo}",
        r#"
s(:regexp,
  s(:str, "foo"),
  s(:regopt))
        "#
    );
    assert_parses_rule!(
        Regexp,
        b"//",
        r#"
s(:regexp,
  s(:regopt))
        "#
    );
}

struct Words;
impl Rule for Words {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tWORDS_BEG)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.take_token();
        type SpToken = ExactToken<{ TokenKind::tSP as u8 }>;
        let (elements, _spaces) = SeparatedBy::<Word, SpToken>::parse(parser)?;
        let end_t = parser
            .expect_token(TokenKind::tSTRING_END)
            .expect("wrong token type");

        Ok(Builder::words_compose(begin_t, elements, end_t))
    }
}
#[test]
fn test_words() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Words,
        b"%W[foo bar]",
        r#"
s(:array,
  s(:str, "foo"),
  s(:str, "bar"))
        "#
    );
    assert_parses_rule!(Words, b"%W[]", r#"s(:array)"#);
}

struct Word;
impl Rule for Word {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        StringContent::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let parts = AtLeastOnce::<StringContent>::parse(parser)?;
        Ok(Builder::word(parts))
    }
}

struct Symbols;
impl Rule for Symbols {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tSYMBOLS_BEG)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.take_token();

        type StringToken = ExactToken<{ TokenKind::tSTRING_CONTENT as u8 }>;
        type SpToken = ExactToken<{ TokenKind::tSP as u8 }>;
        let (elements, _spaces) = SeparatedBy::<StringToken, SpToken>::parse(parser)?;
        let elements = elements
            .into_iter()
            .map(|token| *Builder::string_internal(token, parser.buffer()))
            .collect::<Vec<_>>();

        let end_t = parser
            .expect_token(TokenKind::tSTRING_END)
            .expect("wrong token type");

        Ok(Builder::symbols_compose(begin_t, elements, end_t))
    }
}

#[test]
fn test_symbols() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        Symbols,
        b"%I[foo bar]",
        r#"
s(:array,
  s(:sym, "foo"),
  s(:sym, "bar"))
        "#
    );
    assert_parses_rule!(Symbols, b"%I[]", r#"s(:array)"#);
}

struct QWords;
impl Rule for QWords {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tQWORDS_BEG)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.take_token();

        type StringToken = ExactToken<{ TokenKind::tSTRING_CONTENT as u8 }>;
        type SpToken = ExactToken<{ TokenKind::tSP as u8 }>;
        let (elements, _spaces) = SeparatedBy::<StringToken, SpToken>::parse(parser)?;
        let elements = elements
            .into_iter()
            .map(|token| *Builder::string_internal(token, parser.buffer()))
            .collect::<Vec<_>>();

        let end_t = parser
            .expect_token(TokenKind::tSTRING_END)
            .expect("wrong token type");

        Ok(Builder::words_compose(begin_t, elements, end_t))
    }
}
#[test]
fn test_qwords() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        QWords,
        b"%w[foo bar]",
        r#"
s(:array,
  s(:str, "foo"),
  s(:str, "bar"))
        "#
    );
    assert_parses_rule!(QWords, b"%w[]", r#"s(:array)"#);
}

struct QSymbols;
impl Rule for QSymbols {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tQSYMBOLS_BEG)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.take_token();

        type StringToken = ExactToken<{ TokenKind::tSTRING_CONTENT as u8 }>;
        type SpToken = ExactToken<{ TokenKind::tSP as u8 }>;
        let (elements, _spaces) = SeparatedBy::<StringToken, SpToken>::parse(parser)?;
        let elements = elements
            .into_iter()
            .map(|token| *Builder::symbol_internal(token, parser.buffer()))
            .collect::<Vec<_>>();

        let end_t = parser
            .expect_token(TokenKind::tSTRING_END)
            .expect("wrong token type");

        Ok(Builder::symbols_compose(begin_t, elements, end_t))
    }
}
#[test]
fn test_qsymbols() {
    use crate::testing::assert_parses_rule;
    assert_parses_rule!(
        QSymbols,
        b"%i[foo bar]",
        r#"
s(:array,
  s(:sym, "foo"),
  s(:sym, "bar"))
        "#
    );
    assert_parses_rule!(QSymbols, b"%i[]", r#"s(:array)"#);
}

pub(crate) struct StringContents;
impl Rule for StringContents {
    type Output = Vec<Node>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        Repeat1::<StringContent>::parse(parser)
    }
}

struct StringContent;
impl Rule for StringContent {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        PlainStringContent::starts_now(parser)
            || StringDvarContent::starts_now(parser)
            || InterpolatedStringContent::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if PlainStringContent::starts_now(parser) {
            PlainStringContent::parse(parser)
        } else if StringDvarContent::starts_now(parser) {
            StringDvarContent::parse(parser)
        } else if InterpolatedStringContent::starts_now(parser) {
            InterpolatedStringContent::parse(parser)
        } else {
            unreachable!()
        }
    }
}

struct PlainStringContent;
impl Rule for PlainStringContent {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tSTRING_CONTENT)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let string_t = parser.take_token();
        Ok(Builder::string_internal(string_t, parser.buffer()))
    }
}

struct StringDvarContent;
impl Rule for StringDvarContent {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tSTRING_DVAR)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let _ = parser.take_token();

        if Gvar::starts_now(parser) {
            Gvar::parse(parser)
        } else if Ivar::starts_now(parser) {
            Ivar::parse(parser)
        } else if Cvar::starts_now(parser) {
            Cvar::parse(parser)
        } else if BackRef::starts_now(parser) {
            BackRef::parse(parser)
        } else {
            panic!("wrong token type")
        }
    }
}

struct InterpolatedStringContent;
impl Rule for InterpolatedStringContent {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::tSTRING_DBEG)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let begin_t = parser.take_token();
        let compstmt = parse_compstmt(); // Compstmt::parse(parser)
        let end_t = if parser.current_token().is(TokenKind::tSTRING_DEND) {
            parser.take_token()
        } else {
            panic!("wrong token type")
        };

        let stmts = if let Some(compstmt) = compstmt {
            vec![*compstmt]
        } else {
            vec![]
        };

        Ok(Builder::begin(begin_t, stmts, end_t))
    }
}

fn parse_compstmt() -> Option<Box<Node>> {
    todo!()
}
