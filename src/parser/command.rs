use crate::{
    builder::{ArgsType, Builder, KeywordCmd},
    parser::{
        macros::{all_of, one_of},
        ParseResult, Parser,
    },
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_command(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "command",
            checkpoint = self.new_checkpoint(),
            {
                let (fcall, (command_args, cmd_brace_block)) = all_of!(
                    "fcall",
                    self.parse_fcall(),
                    parse_command_args_and_cmd_brace_block(self),
                )?;

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
            },
            {
                let (primary, call_op_t, op_t, (command_args, cmd_brace_block)) = all_of!(
                    "primary call_op2 operation command_args",
                    self.parse_primary_value(),
                    self.parse_call_op2(),
                    self.parse_operation2(),
                    parse_command_args_and_cmd_brace_block(self),
                )?;

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
            },
            {
                let (super_t, args) = all_of!(
                    "super args",
                    self.try_token(TokenKind::kSUPER),
                    self.parse_command_args(),
                )?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Super,
                    super_t,
                    None,
                    args,
                    None,
                ))
            },
            {
                let (yield_t, args) = all_of!(
                    "yield args",
                    self.try_token(TokenKind::kYIELD),
                    self.parse_command_args(),
                )?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Yield,
                    yield_t,
                    None,
                    args,
                    None,
                ))
            },
            {
                let (return_t, args) =
                    all_of!("return args", self.parse_k_return(), self.parse_call_args(),)?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Return,
                    return_t,
                    None,
                    args,
                    None,
                ))
            },
            {
                let (break_t, args) = all_of!(
                    "break args",
                    self.try_token(TokenKind::kBREAK),
                    self.parse_call_args(),
                )?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Break,
                    break_t,
                    None,
                    args,
                    None,
                ))
            },
            {
                let (next_t, args) = all_of!(
                    "next args",
                    self.try_token(TokenKind::kNEXT),
                    self.parse_call_args(),
                )?;

                Ok(Builder::keyword_cmd(
                    KeywordCmd::Next,
                    next_t,
                    None,
                    args,
                    None,
                ))
            },
        )
    }

    pub(crate) fn parse_command_args(&mut self) -> ParseResult<Vec<Node>> {
        self.parse_call_args()
    }

    pub(crate) fn parse_brace_body(&mut self) -> ParseResult<(ArgsType, Option<Box<Node>>)> {
        let (args, body) = all_of!(
            "brace block body",
            self.parse_opt_block_param(),
            self.try_compstmt(),
        )?;

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
        one_of!(
            "call args",
            checkpoint = self.new_checkpoint(),
            {
                let command = self.parse_command()?;
                // self.value_expr(&command);
                Ok(vec![*command])
            },
            {
                let ((mut args, _comma_t), assocs, mut opt_block_arg) = all_of!(
                    "args tCOMMA assocs opt_block_arg",
                    parse_args_t_comma(self),
                    self.parse_assocs(),
                    self.parse_opt_block_arg(),
                )?;

                let hash = Builder::associate(None, assocs, None);
                args.push(*hash);
                args.append(&mut opt_block_arg);
                Ok(args)
            },
            {
                let (mut args, mut opt_block_arg) = all_of!(
                    "args opt_block_arg",
                    self.parse_args(),
                    self.parse_opt_block_arg(),
                )?;

                args.append(&mut opt_block_arg);
                Ok(args)
            },
            {
                let (assocs, mut opt_block_arg) = all_of!(
                    "assocs opt_block_arg",
                    self.parse_assocs(),
                    self.parse_opt_block_arg(),
                )?;

                let hash = Builder::associate(None, assocs, None);
                let mut nodes = Vec::with_capacity(1 + opt_block_arg.len());
                nodes.push(*hash);
                nodes.append(&mut opt_block_arg);
                Ok(nodes)
            },
            {
                let block_arg = self.parse_block_arg()?;
                Ok(vec![*block_arg])
            },
        )
    }

    // This rule can be `none`
    pub(crate) fn parse_opt_call_args(&mut self) -> ParseResult<Vec<Node>> {
        one_of!(
            "opt call args",
            checkpoint = self.new_checkpoint(),
            self.parse_call_args(),
            {
                let (args, _comma_t) = parse_args_t_comma(self)?;
                Ok(args)
            },
            {
                let ((mut args, _), (assocs, _)) = all_of!(
                    "args tCOMMA assocs tCOMMA",
                    parse_args_t_comma(self),
                    parse_assocs_t_comma(self),
                )?;
                let hash = Builder::associate(None, assocs, None);
                args.push(*hash);
                Ok(args)
            },
            {
                let (assocs, _) = parse_assocs_t_comma(self)?;
                Ok(assocs)
            },
            Ok(vec![]),
        )
    }
}

type CmdBraceBlock = (Token, ArgsType, Option<Box<Node>>, Token);

fn parse_command_args_and_cmd_brace_block(
    parser: &mut Parser,
) -> ParseResult<(Vec<Node>, Option<CmdBraceBlock>)> {
    all_of!(
        "command_args [+ cmd_brace_block]",
        parser.parse_command_args(),
        one_of!(
            "maybe command_args",
            checkpoint = parser.new_checkpoint(),
            parse_cmd_brace_block(parser).map(|block| Some(block)),
            Ok(None),
        ),
    )
}

fn parse_cmd_brace_block(parser: &mut Parser) -> ParseResult<CmdBraceBlock> {
    let (begin_t, (args_type, brace_body), end_t) = all_of!(
        "cmd brace block",
        parser.try_token(TokenKind::tLCURLY),
        parser.parse_brace_body(),
        parser.expect_token(TokenKind::tRCURLY),
    )?;

    Ok((begin_t, args_type, brace_body, end_t))
}

fn parse_args_t_comma(parser: &mut Parser) -> ParseResult<(Vec<Node>, Token)> {
    all_of!(
        "args tCOMMA",
        parser.parse_args(),
        parser.expect_token(TokenKind::tCOMMA),
    )
}

fn parse_assocs_t_comma(parser: &mut Parser) -> ParseResult<(Vec<Node>, Token)> {
    all_of!(
        "assocs tCOMMA",
        parser.parse_assocs(),
        parser.expect_token(TokenKind::tCOMMA),
    )
}
