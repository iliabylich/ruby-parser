use crate::{
    builder::Builder,
    loc::loc,
    parser::{
        macros::{all_of, maybe, one_of, separated_by},
        ParseResult,
    },
    token::token,
    Node, Parser, Token, TokenKind,
};

type ParenArgs = (Token, Vec<Node>, Token);
type OptParenArgs = (Option<Token>, Vec<Node>, Option<Token>);

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

    pub(crate) fn parse_opt_paren_args(&mut self) -> ParseResult<OptParenArgs> {
        let maybe_paren_args = maybe!(self.parse_paren_args())?;

        match maybe_paren_args {
            Some((begin_t, args, end_t)) => Ok((Some(begin_t), args, Some(end_t))),
            None => Ok((None, vec![], None)),
        }
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

    pub(crate) fn parse_opt_block_param(&mut self) -> ParseResult<Option<Box<Node>>> {
        one_of!(
            "opt_block_param",
            checkpoint = self.new_checkpoint(),
            parse_block_param_def(self),
            Ok(None),
        )
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

    // TODO: report bad argument
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

fn parse_block_param(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let args = parser.parse_f_args()?;
    let excessed_comma_t = parser.try_token(TokenKind::tCOMMA).ok();
    if args.len() == 1 && matches!(&args[0], Node::Arg(_)) && excessed_comma_t.is_none() {
        // TODO: rewrite arg -> procarg0
    }
    Ok(args)
}

fn parse_block_param_def(parser: &mut Parser) -> ParseResult<Option<Box<Node>>> {
    one_of!(
        "block_param_def",
        checkpoint = parser.new_checkpoint(),
        {
            let orop_t = parser.try_token(TokenKind::tOROP)?;
            let begin_t = token!(tPIPE, loc!(orop_t.loc.start, orop_t.loc.start + 1));
            let end_t = token!(tPIPE, loc!(orop_t.loc.start + 1, orop_t.loc.start + 2));
            Ok(Builder::args(Some(begin_t), vec![], Some(end_t)))
        },
        {
            let (begin_t, args, end_t) = all_of!(
                "tPIPE opt_bv_decl tPIPE",
                parser.try_token(TokenKind::tPIPE),
                parse_opt_bv_decl(parser),
                parser.expect_token(TokenKind::tPIPE),
            )?;

            Ok(Builder::args(Some(begin_t), args, Some(end_t)))
        },
        {
            let (begin_t, mut args, mut bv_args, end_t) = all_of!(
                "tPIPE block_param opt_bv_decl tPIPE",
                parser.try_token(TokenKind::tPIPE),
                parse_block_param(parser),
                parse_opt_bv_decl(parser),
                parser.expect_token(TokenKind::tPIPE),
            )?;

            args.append(&mut bv_args);

            Ok(Builder::args(Some(begin_t), args, Some(end_t)))
        },
    )
}
fn parse_opt_bv_decl(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    one_of!(
        "opt_bv_decl",
        checkpoint = parser.new_checkpoint(),
        {
            let (_opt_nl1, _semi_t, args, _opt_nl2) = all_of!(
                "opt_nl tSEMI bv_decls opt_nl",
                parser.try_opt_nl(),
                parser.expect_token(TokenKind::tSEMI),
                parse_bv_decls(parser),
                parser.try_opt_nl(),
            )?;

            Ok(args)
        },
        {
            let _opt_nl = parser.try_opt_nl()?;
            Ok(vec![])
        },
    )
}
fn parse_bv_decls(parser: &mut Parser) -> ParseResult<Vec<Node>> {
    let (args, _commas) = separated_by!(
        "bv_decls",
        checkpoint = parser.new_checkpoint(),
        item = parse_bvar(parser),
        sep = parser.try_token(TokenKind::tCOMMA)
    )?;
    Ok(args)
}
fn parse_bvar(parser: &mut Parser) -> ParseResult<Box<Node>> {
    one_of!(
        "bvar",
        {
            let name_t = parser.try_token(TokenKind::tIDENTIFIER)?;
            Ok(Builder::shadowarg(name_t, parser.buffer()))
        },
        {
            let name_t = parse_f_bad_arg(parser)?;
            Ok(Builder::shadowarg(name_t, parser.buffer()))
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses_some;

    #[test]
    fn test_paren_args_empty() {
        assert_parses_some!(Parser::parse_f_paren_args, b"()", "s(:args)")
    }

    #[test]
    fn test_paren_args_full() {
        assert_parses_some!(
            Parser::parse_f_paren_args,
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

    #[test]
    fn test_opt_block_param_empty() {
        assert_parses_some!(Parser::parse_opt_block_param, b"||", "s(:args)")
    }

    #[test]
    fn test_opt_block_param_full() {
        assert_parses_some!(
            Parser::parse_opt_block_param,
            concat!(
                "|",
                "req1, req2,",
                "opt1 = 1, opt2 = 2,",
                "*rest,",
                "kw1:, kw2:,",
                "kwopt1: 3, kwopt2: 4,",
                "**kwrest,",
                "&blk",
                "; shadowarg1, shadowarg2",
                "|"
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
  s(:blockarg, "blk"),
  s(:shadowarg, "shadowarg1"),
  s(:shadowarg, "shadowarg2"))
            "#
        )
    }

    #[test]
    fn test_opt_block_param_shadowargs_only() {
        assert_parses_some!(
            Parser::parse_opt_block_param,
            b"|;a, b|",
            r#"
s(:args,
  s(:shadowarg, "a"),
  s(:shadowarg, "b"))
            "#
        )
    }
}
