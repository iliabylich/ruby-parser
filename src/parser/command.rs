use crate::{
    builder::{Builder, Constructor},
    parser::{ParseError, Parser},
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_command(&mut self) -> Result<Box<Node>, ParseError> {
        let checkpoint = self.new_checkpoint();

        let maybe_call_with_command_args: Result<Box<Node>, _> = self
            .one_of("maybe command call with command args")
            .or_else(|| {
                let fcall = self.try_fcall()?;
                let command_args = self.try_command_args()?;
                todo!("call_method {:?} {:?}", fcall, command_args)
            })
            .or_else(|| {
                let primary_value = self.try_primary_value()?;
                let op_t = self
                    .one_of("::CONSTANT or ::method")
                    .or_else(|| self.try_token(TokenKind::tCOLON2))
                    .or_else(|| self.try_operation2())
                    .done()?;

                let operation2 = self.try_operation2()?;
                let command_args = self.try_command_args()?;
                todo!(
                    "return call_method {:?} {:?} {:?} {:?}",
                    primary_value,
                    op_t,
                    operation2,
                    command_args
                )
            })
            .or_else(|| {
                let super_t = self.try_token(TokenKind::kSUPER)?;
                let command_args = self.try_command_args()?;
                todo!("super {:?} {:?}", super_t, command_args)
            })
            .or_else(|| {
                let yield_t = self.try_token(TokenKind::kYIELD)?;
                let command_args = self.try_command_args();
                todo!("yield {:?} {:?}", yield_t, command_args)
            })
            .done();

        if let Ok(call_with_command_args) = maybe_call_with_command_args {
            match &*call_with_command_args {
                Node::Super(_) | Node::ZSuper(_) | Node::Yield(_) => {
                    // these nodes can't take block
                    return Ok(call_with_command_args);
                }
                _ => {
                    if let Ok(cmd_brace_block) = try_cmd_brace_block(self) {
                        panic!(
                            "block_call {:?} {:?}",
                            call_with_command_args, cmd_brace_block
                        )
                    }
                }
            }
        }

        let keyword_t = self
            .one_of("keyword command")
            .or_else(|| self.try_k_return())
            .or_else(|| self.try_token(TokenKind::kBREAK))
            .or_else(|| self.try_token(TokenKind::kNEXT))
            .done()?;
        let call_args = self.try_call_args()?;
        todo!("keyword_cmd {:?} {:?}", keyword_t, call_args)
    }

    pub(crate) fn try_command_args(&mut self) -> Result<Vec<Node>, ParseError> {
        self.try_call_args()
    }

    pub(crate) fn try_brace_body(&mut self) -> Result<Option<Box<Node>>, ParseError> {
        todo!("parser.try_brace_body")
    }

    // This rule can be `none`
    pub(crate) fn try_call_args(&mut self) -> Result<Vec<Node>, ParseError> {
        todo!("parser.try_call_args")
    }
}

#[derive(Debug)]
struct CmdBraceBlock {
    begin_t: Token,
    brace_body: Option<Box<Node>>,
    end_t: Token,
}

fn try_cmd_brace_block<C: Constructor>(
    parser: &mut Parser<C>,
) -> Result<CmdBraceBlock, ParseError> {
    let begin_t = parser.try_token(TokenKind::tLCURLY)?;
    let brace_body = parser.try_brace_body()?;
    let end_t = parser.expect_token(TokenKind::tRCURLY);
    Ok(CmdBraceBlock {
        begin_t,
        brace_body,
        end_t,
    })
}
