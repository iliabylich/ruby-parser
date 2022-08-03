use crate::{
    builder::{ArgsType, Builder, KeywordCmd},
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_command(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("command")
            .or_else(|| {
                let (fcall, (command_args, cmd_brace_block)) = self
                    .all_of("fcall")
                    .and(|| self.parse_fcall())
                    .and(|| parse_command_args_and_cmd_brace_block(self))
                    .stop()?;

                let method_call = Builder::call_method(
                    None,
                    None,
                    Some(fcall),
                    None,
                    command_args,
                    None,
                    self.buffer(),
                );
                if let Some((begin_t, args_type, body, end_t)) = cmd_brace_block {
                    Ok(Builder::block(method_call, begin_t, args_type, body, end_t))
                } else {
                    Ok(method_call)
                }
            })
            .or_else(|| {
                let (primary, call_op_t, op_t, (command_args, cmd_brace_block)) = self
                    .all_of("primary call_op2 operation command_args")
                    .and(|| self.parse_primary_value())
                    .and(|| self.parse_call_op2())
                    .and(|| self.parse_operation2())
                    .and(|| parse_command_args_and_cmd_brace_block(self))
                    .stop()?;

                let method_call = Builder::call_method(
                    Some(primary),
                    Some(call_op_t),
                    Some(op_t),
                    None,
                    command_args,
                    None,
                    self.buffer(),
                );

                if let Some((begin_t, args_type, body, end_t)) = cmd_brace_block {
                    Ok(Builder::block(method_call, begin_t, args_type, body, end_t))
                } else {
                    Ok(method_call)
                }
            })
            .or_else(|| {
                let (super_t, args) = self
                    .all_of("super args")
                    .and(|| self.try_token(TokenKind::kSUPER))
                    .and(|| self.parse_command_args())
                    .stop()?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Super,
                    super_t,
                    None,
                    args,
                    None,
                ))
            })
            .or_else(|| {
                let (yield_t, args) = self
                    .all_of("yield args")
                    .and(|| self.try_token(TokenKind::kYIELD))
                    .and(|| self.parse_command_args())
                    .stop()?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Yield,
                    yield_t,
                    None,
                    args,
                    None,
                ))
            })
            .or_else(|| {
                let (return_t, args) = self
                    .all_of("return args")
                    .and(|| self.parse_k_return())
                    .and(|| self.parse_call_args())
                    .stop()?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Return,
                    return_t,
                    None,
                    args,
                    None,
                ))
            })
            .or_else(|| {
                let (break_t, args) = self
                    .all_of("break args")
                    .and(|| self.try_token(TokenKind::kBREAK))
                    .and(|| self.parse_call_args())
                    .stop()?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Break,
                    break_t,
                    None,
                    args,
                    None,
                ))
            })
            .or_else(|| {
                let (next_t, args) = self
                    .all_of("next args")
                    .and(|| self.try_token(TokenKind::kNEXT))
                    .and(|| self.parse_call_args())
                    .stop()?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Next,
                    next_t,
                    None,
                    args,
                    None,
                ))
            })
            .stop()
    }

    pub(crate) fn parse_command_args(&mut self) -> ParseResult<Vec<Node>> {
        self.parse_call_args()
    }

    pub(crate) fn parse_brace_body(&mut self) -> ParseResult<(ArgsType, Option<Box<Node>>)> {
        let (args, body) = self
            .all_of("brace block body")
            .and(|| self.try_opt_block_param())
            .and(|| self.try_compstmt())
            .stop()?;

        // TODO: this should be more dynamic
        // let args_type = if self.max_numparam_stack.has_numparams() {
        //     ArgsType::Numargs(self.max_numparam_stack.top() as u8)
        // } else {
        //     ArgsType::Args(args)
        // };
        let args_type = ArgsType::Args(args);
        Ok((args_type, body))
    }

    // This rule can be `none`
    pub(crate) fn parse_call_args(&mut self) -> ParseResult<Vec<Node>> {
        self.one_of("call args")
            .or_else(|| {
                let command = self.parse_command()?;
                // self.value_expr(&command);
                Ok(vec![*command])
            })
            .or_else(|| {
                let ((mut args, _comma_t), assocs, mut opt_block_arg) = self
                    .all_of("args tCOMMA assocs opt_block_arg")
                    .and(|| parse_args_t_comma(self))
                    .and(|| self.parse_assocs())
                    .and(|| self.parse_opt_block_arg())
                    .stop()?;

                let hash = Builder::associate(None, assocs, None);
                args.push(*hash);
                args.append(&mut opt_block_arg);
                Ok(args)
            })
            .or_else(|| {
                let (mut args, mut opt_block_arg) = self
                    .all_of("args opt_block_arg")
                    .and(|| self.parse_args())
                    .and(|| self.parse_opt_block_arg())
                    .stop()?;

                args.append(&mut opt_block_arg);
                Ok(args)
            })
            .or_else(|| {
                let (assocs, mut opt_block_arg) = self
                    .all_of("assocs opt_block_arg")
                    .and(|| self.parse_assocs())
                    .and(|| self.parse_opt_block_arg())
                    .stop()?;

                let hash = Builder::associate(None, assocs, None);
                let mut nodes = Vec::with_capacity(1 + opt_block_arg.len());
                nodes.push(*hash);
                nodes.append(&mut opt_block_arg);
                Ok(nodes)
            })
            .or_else(|| {
                let block_arg = self.parse_block_arg()?;
                Ok(vec![*block_arg])
            })
            .stop()
    }

    // This rule can be `none`
    pub(crate) fn parse_opt_call_args(&mut self) -> ParseResult<Vec<Node>> {
        self.one_of("opt call args")
            .or_else(|| self.parse_call_args())
            .or_else(|| {
                let (args, _comma_t) = parse_args_t_comma(self)?;
                Ok(args)
            })
            .or_else(|| {
                let ((mut args, _), (assocs, _)) = self
                    .all_of("args tCOMMA assocs tCOMMA")
                    .and(|| parse_args_t_comma(self))
                    .and(|| parse_assocs_t_comma(self))
                    .stop()?;
                let hash = Builder::associate(None, assocs, None);
                args.push(*hash);
                Ok(args)
            })
            .or_else(|| {
                let (assocs, _) = parse_assocs_t_comma(self)?;
                Ok(assocs)
            })
            .or_else(|| Ok(vec![]))
            .stop()
    }
}

type CmdBraceBlock = (Token, ArgsType, Option<Box<Node>>, Token);

fn parse_command_args_and_cmd_brace_block(
    parser: &mut Parser,
) -> ParseResult<(Vec<Node>, Option<CmdBraceBlock>)> {
    parser
        .all_of("command_args [+ cmd_brace_block]")
        .and(|| parser.parse_command_args())
        .and(|| {
            parser
                .one_of("maybe command_args")
                .or_else(|| parse_cmd_brace_block(parser).map(|block| Some(block)))
                .or_else(|| Ok(None))
                .stop()
        })
        .stop()
}

fn parse_cmd_brace_block(parser: &mut Parser) -> ParseResult<CmdBraceBlock> {
    let (begin_t, (args_type, brace_body), end_t) = parser
        .all_of("cmd brace block")
        .and(|| parser.try_token(TokenKind::tLCURLY))
        .and(|| parser.parse_brace_body())
        .and(|| parser.expect_token(TokenKind::tRCURLY))
        .stop()?;

    Ok((begin_t, args_type, brace_body, end_t))
}

fn parse_args_t_comma(parser: &mut Parser) -> ParseResult<(Vec<Node>, Token)> {
    parser
        .all_of("args tCOMMA")
        .and(|| parser.parse_args())
        .and(|| parser.expect_token(TokenKind::tCOMMA))
        .stop()
}

fn parse_assocs_t_comma(parser: &mut Parser) -> ParseResult<(Vec<Node>, Token)> {
    parser
        .all_of("assocs tCOMMA")
        .and(|| parser.parse_assocs())
        .and(|| parser.expect_token(TokenKind::tCOMMA))
        .stop()
}
