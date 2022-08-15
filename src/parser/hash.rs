use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseError, ParseResult, Parser},
    token::TokenKind,
    Node,
};

impl Parser {
    pub(crate) fn parse_hash(&mut self) -> ParseResult<Box<Node>> {
        let (begin_t, pairs, end_t) = all_of!(
            "hash",
            self.try_token(TokenKind::tLCURLY),
            parse_assoc_list(self),
            self.expect_token(TokenKind::tRCURLY),
        )?;

        Ok(Builder::associate(Some(begin_t), pairs, Some(end_t)))
    }

    pub(crate) fn parse_assocs(&mut self) -> ParseResult<Vec<Node>> {
        let mut nodes = vec![];
        let mut commas = vec![];

        let assoc = self.parse_assoc()?;
        nodes.push(*assoc);

        loop {
            if self.current_token().is(TokenKind::tCOMMA) {
                commas.push(self.current_token());
                self.skip_token();
            } else {
                break;
            }

            match self.parse_assoc() {
                Ok(node) => nodes.push(*node),
                Err(error) => return Err(ParseError::seq_error("assocs", (nodes, commas), error)),
            }
        }

        Ok(nodes)
    }

    pub(crate) fn parse_assoc(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("assoc")
            .or_else(|| {
                let (key_t, value) = all_of!(
                    "tLABEL arg_value",
                    self.try_token(TokenKind::tLABEL),
                    self.parse_arg_value(),
                )?;

                Ok(Builder::pair_keyword(key_t, value, self.buffer()))
            })
            .or_else(|| {
                let key_t = self.try_token(TokenKind::tLABEL)?;
                Ok(Builder::pair_label(key_t, self.buffer()))
            })
            .or_else(|| {
                let (begin_t, parts, end_t, value) = all_of!(
                    "tSTRING_BEG string_contents tLABEL_END arg_value",
                    self.try_token(TokenKind::tSTRING_BEG),
                    self.parse_string_contents(),
                    self.expect_token(TokenKind::tLABEL_END),
                    self.parse_arg_value(),
                )?;

                Ok(Builder::pair_quoted(begin_t, parts, end_t, value))
            })
            .or_else(|| {
                let (dstar_t, value) = all_of!(
                    "tDSTAR arg_value",
                    self.try_token(TokenKind::tDSTAR),
                    self.parse_arg_value(),
                )?;

                Ok(Builder::kwsplat(dstar_t, value))
            })
            .or_else(|| {
                let (key, assoc_t, value) = all_of!(
                    "arg_value tASSOC arg_value",
                    self.parse_arg_value(),
                    self.expect_token(TokenKind::tASSOC),
                    self.parse_arg_value(),
                )?;

                Ok(Builder::pair(key, assoc_t, value))
            })
            .stop()
    }
}

fn parse_assoc_list(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    parser
        .one_of("assoc list")
        .or_else(|| {
            let (assocs, _trailer) = all_of!(
                "assics trailer",
                parser.parse_assocs(),
                Ok(parser.try_trailer()),
            )?;
            Ok(assocs)
        })
        .or_else(|| Ok(vec![]))
        .stop()
}

#[test]
fn test_hash() {
    use crate::testing::assert_parses;

    assert_parses!(
        parse_hash,
        b"{ a: 1, :b => 2, c => 3 }",
        r#"
s(:hash,
  s(:pair,
    s(:sym, "a:"),
    s(:int, "1")),
  s(:pair,
    s(:sym, "b"),
    s(:int, "2")),
  s(:pair,
    s(:lvar, "c"),
    s(:int, "3")))
        "#
    );
}
