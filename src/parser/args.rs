use crate::{
    builder::Builder,
    parser::{
        macros::{all_of, maybe, one_of, separated_by},
        ParseResult,
    },
    Node, Parser, Token, TokenKind,
};

type ParenArgs = (Token, Vec<Node>, Token);

impl Parser {
    pub(crate) fn parse_paren_args(&mut self) -> ParseResult<ParenArgs> {
        one_of!(
            "paren args",
            checkpoint = self.new_checkpoint(),
            {
                let (lparen_t, args, rparen_t) = all_of!(
                    "tLPAREN2 opt_call_args rparen",
                    self.try_token(TokenKind::tLPAREN),
                    self.parse_opt_call_args(),
                    self.parse_rparen(),
                )?;
                Ok((lparen_t, args, rparen_t))
            },
            {
                let (lparen_t, mut args, _comma_t, args_forward_t, rparen_t) = all_of!(
                    "tLPAREN2 args tCOMMA args_forward rparen",
                    self.try_token(TokenKind::tLPAREN),
                    self.parse_args(),
                    self.expect_token(TokenKind::tCOMMA),
                    self.parse_args_forward(),
                    self.parse_rparen(),
                )?;

                let forwarded_args = Builder::forwarded_args(args_forward_t);
                args.push(*forwarded_args);

                Ok((lparen_t, args, rparen_t))
            },
            {
                let (lparen_t, args_forward_t, rparen_t) = all_of!(
                    "tLPAREN2 args_forward rparen",
                    self.try_token(TokenKind::tLPAREN),
                    self.parse_args_forward(),
                    self.parse_rparen(),
                )?;

                let forwarded_args = Builder::forwarded_args(args_forward_t);
                let args = vec![*forwarded_args];
                Ok((lparen_t, args, rparen_t))
            },
        )
    }

    pub(crate) fn parse_opt_paren_args(&mut self) -> ParseResult<Option<ParenArgs>> {
        maybe!(self.parse_paren_args())
    }

    pub(crate) fn parse_f_paren_args(&mut self) -> ParseResult<Option<Box<Node>>> {
        let (lparen_t, args, rparen_t) = all_of!(
            "tLPAREN2 f_args rparen",
            self.try_token(TokenKind::tLPAREN),
            self.parse_f_args(),
            self.parse_rparen(),
        )?;

        Ok(Builder::args(Some(lparen_t), args, Some(rparen_t)))
    }

    pub(crate) fn parse_f_args(&mut self) -> ParseResult<Vec<Node>> {
        let args = maybe!(separated_by!(
            "f_args",
            checkpoint = self.new_checkpoint(),
            item = parse_arg(self),
            sep = self.try_token(TokenKind::tCOMMA)
        ))?;

        match args {
            Some((args, _commas)) => Ok(args),
            None => Ok(vec![]),
        }
    }
}

fn parse_arg(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "argument",
        checkpoint = parser.new_checkpoint(),
        optarg(parser),
        kwoptarg(parser),
        kwarg(parser),
        arg(parser),
        // TODO: mlhs_arg(parser),
        restarg(parser),
        kwrestarg(parser),
        blokarg(parser),
    )
}

fn optarg(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (name_t, eql_t, default) = all_of!(
        "f_arg_asgn f_eq arg_value",
        parse_f_arg_asgn(parser),
        parse_f_eq(parser),
        parser.parse_arg_value(),
    )?;

    Ok(Builder::optarg(name_t, eql_t, default, parser.buffer()))
}
fn kwoptarg(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let (name_t, default) = all_of!("kwoptarg", parse_f_label(parser), parser.parse_arg_value(),)?;

    Ok(Builder::kwoptarg(name_t, default, parser.buffer()))
}
fn kwarg(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let name_t = parse_f_label(parser)?;
    Ok(Builder::kwarg(name_t, parser.buffer()))
}
fn arg(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let name_t = parse_f_arg_asgn(parser)?;
    Ok(Builder::arg(name_t, parser.buffer()))
}
fn restarg(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let star_t = parser.try_token(TokenKind::tSTAR)?;

    let mut name_t = None;
    if parser.current_token().is(TokenKind::tIDENTIFIER) {
        name_t = Some(parser.current_token());
        parser.skip_token();
    }

    Ok(Builder::restarg(star_t, name_t, parser.buffer()))
}
fn kwrestarg(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let dstar_t = parser.try_token(TokenKind::tDSTAR)?;

    let mut name_t = None;
    if parser.current_token().is(TokenKind::tIDENTIFIER) {
        name_t = Some(parser.current_token());
        parser.skip_token();
    }

    Ok(Builder::kwrestarg(dstar_t, name_t, parser.buffer()))
}
fn blokarg(parser: &mut Parser) -> ParseResult<Box<Node>> {
    let amper_t = parser.try_token(TokenKind::tAMPER)?;

    let mut name_t = None;
    if parser.current_token().is(TokenKind::tIDENTIFIER) {
        name_t = Some(parser.current_token());
        parser.skip_token();
    }

    Ok(Builder::blockarg(amper_t, name_t, parser.buffer()))
}

fn parse_f_bad_arg(parser: &mut Parser) -> ParseResult<Token> {
    one_of!(
        "f_bad_arg",
        checkpoint = parser.new_checkpoint(),
        parser.try_token(TokenKind::tCONSTANT),
        parser.try_token(TokenKind::tIVAR),
        parser.try_token(TokenKind::tGVAR),
        parser.try_token(TokenKind::tCVAR),
    )

    // TODO: report diagnostic
}
fn parse_f_norm_arg(parser: &mut Parser) -> ParseResult<Token> {
    one_of!(
        "f_norm_arg",
        checkpoint = parser.new_checkpoint(),
        parse_f_bad_arg(parser),
        {
            let ident_t = parser.try_token(TokenKind::tIDENTIFIER)?;
            // TODO: declare var
            Ok(ident_t)
        },
    )
}
fn parse_f_arg_asgn(parser: &mut Parser) -> ParseResult<Token> {
    let name_t = parse_f_norm_arg(parser)?;
    // TODO: current_arg_stack.set(...)
    Ok(name_t)
}
fn parse_f_eq(parser: &mut Parser) -> ParseResult<Token> {
    let eql_t = parser.expect_token(TokenKind::tEQL)?;
    // TODO: track in_argdef
    Ok(eql_t)
}

fn parse_f_label(parser: &mut Parser) -> ParseResult<Token> {
    let label_t = parser.try_token(TokenKind::tLABEL)?;
    // TODO: declare var
    Ok(label_t)
}

#[cfg(test)]
mod tests {
    macro_rules! assert_parses_f_paren_args {
        ($src:expr, $expected:expr) => {{
            use crate::{
                parser::{ParseResult, Parser},
                Node,
            };

            let src: &[u8] = $src;
            let mut parser = Parser::new(src).debug();
            let parsed: ParseResult<Option<Box<Node>>> = parser.parse_f_paren_args();

            let ast;
            match parsed {
                Ok(node) => ast = node,
                Err(err) => {
                    eprintln!("{}", err.render());
                    panic!("expected Ok(node), got Err()")
                }
            }

            let ast = match ast {
                Some(ast) => ast,
                None => {
                    panic!("expected some AST to ber returned, got None")
                }
            };

            let expected: &str = $expected;
            dbg!(&ast);
            assert_eq!(ast.inspect(0), expected.trim_start().trim_end());
            assert!(parser.state.inner.buffer.is_eof())
        }};
    }

    #[test]
    fn test_paren_args_empty() {
        assert_parses_f_paren_args!(b"()", "s(:args)")
    }

    #[test]
    fn test_paren_args_full() {
        assert_parses_f_paren_args!(
            concat!(
                "(",
                "req1, req2,",
                "opt1 = 1, opt2 = 2,",
                "*rest,",
                "kw1:, kw2:,",
                "kwopt1: 3, kwopt2: 4,",
                "**kwrest,",
                "&blk",
                ")"
            )
            .as_bytes(),
            r#"
s(:args,
  s(:arg, "req1"),
  s(:arg, "req2"),
  s(:optarg, "opt1",
    s(:int, "1")),
  s(:optarg, "opt2",
    s(:int, "2")),
  s(:restarg, "rest"),
  s(:kwarg, "kw1:"),
  s(:kwarg, "kw2:"),
  s(:kwoptarg, "kwopt1:",
    s(:int, "3")),
  s(:kwoptarg, "kwopt2:",
    s(:int, "4")),
  s(:kwrestarg, "kwrest"),
  s(:blockarg, "blk"))
            "#
        )
    }
}
