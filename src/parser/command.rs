use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn try_command(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("command")
            .or_else(|| {
                let (fcall, (command_args, cmd_brace_block)) = self
                    .all_of("fcall")
                    .and(|| self.try_fcall())
                    .and(|| try_command_args_and_cmd_brace_block(self))
                    .stop()?;
                #[allow(unreachable_code)]
                Ok(todo!(
                    "{:?} {:?} {:?}",
                    fcall,
                    command_args,
                    cmd_brace_block
                ))
            })
            .or_else(|| {
                let (primary, call_op_t, op_t, (command_args, cmd_brace_block)) = self
                    .all_of("primary call_op2 operation command_args")
                    .and(|| self.try_primary_value())
                    .and(|| self.try_call_op2())
                    .and(|| self.try_operation2())
                    .and(|| try_command_args_and_cmd_brace_block(self))
                    .stop()?;

                #[allow(unreachable_code)]
                Ok(todo!(
                    "{:?} {:?} {:?} {:?} {:?}",
                    primary,
                    call_op_t,
                    op_t,
                    command_args,
                    cmd_brace_block
                ))
            })
            .or_else(|| {
                let (super_t, args) = self
                    .all_of("super args")
                    .and(|| self.try_token(TokenKind::kSUPER))
                    .and(|| self.try_command_args())
                    .stop()?;

                #[allow(unreachable_code)]
                Ok(todo!("{:?} {:?}", super_t, args))
            })
            .or_else(|| {
                let (yield_t, args) = self
                    .all_of("yield args")
                    .and(|| self.try_token(TokenKind::kYIELD))
                    .and(|| self.try_command_args())
                    .stop()?;

                #[allow(unreachable_code)]
                Ok(todo!("{:?} {:?}", yield_t, args))
            })
            .or_else(|| {
                let (return_t, args) = self
                    .all_of("return args")
                    .and(|| self.try_k_return())
                    .and(|| self.try_call_args())
                    .stop()?;

                #[allow(unreachable_code)]
                Ok(todo!("{:?} {:?}", return_t, args))
            })
            .or_else(|| {
                let (break_t, args) = self
                    .all_of("break args")
                    .and(|| self.try_token(TokenKind::kBREAK))
                    .and(|| self.try_call_args())
                    .stop()?;

                #[allow(unreachable_code)]
                Ok(todo!("{:?} {:?}", break_t, args))
            })
            .or_else(|| {
                let (next_t, args) = self
                    .all_of("next args")
                    .and(|| self.try_token(TokenKind::kNEXT))
                    .and(|| self.try_call_args())
                    .stop()?;

                #[allow(unreachable_code)]
                Ok(todo!("{:?} {:?}", next_t, args))
            })
            .stop()
    }

    pub(crate) fn try_command_args(&mut self) -> ParseResult<Vec<Node>> {
        self.try_call_args()
    }

    pub(crate) fn try_brace_body(&mut self) -> ParseResult<Option<Box<Node>>> {
        todo!("parser.try_brace_body")
    }

    // This rule can be `none`
    pub(crate) fn try_call_args(&mut self) -> ParseResult<Vec<Node>> {
        todo!("parser.try_call_args")
    }

    // This rule can be `none`
    pub(crate) fn try_opt_call_args(&mut self) -> ParseResult<Vec<Node>> {
        todo!("parser.try_call_args")
    }
}

type CmdBraceBlock = (Token, Option<Box<Node>>, Token);

fn try_command_args_and_cmd_brace_block(
    parser: &mut Parser,
) -> ParseResult<(Vec<Node>, Option<CmdBraceBlock>)> {
    parser
        .all_of("command_args [+ cmd_brace_block]")
        .and(|| parser.try_command_args())
        .and(|| {
            parser
                .one_of("maybe command_args")
                .or_else(|| try_cmd_brace_block(parser).map(|block| Some(block)))
                .or_else(|| Ok(None))
                .stop()
        })
        .stop()
}

fn try_cmd_brace_block(parser: &mut Parser) -> ParseResult<CmdBraceBlock> {
    let (begin_t, brace_body, end_t) = parser
        .all_of("cmd brace block")
        .and(|| parser.try_token(TokenKind::tLCURLY))
        .and(|| parser.try_brace_body())
        .and(|| parser.expect_token(TokenKind::tRCURLY))
        .stop()?;

    Ok((begin_t, brace_body, end_t))
}
