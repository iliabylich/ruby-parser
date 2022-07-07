use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::{Token, TokenValue},
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_command(&mut self) -> Option<Box<Node<'a>>> {
        let checkpoint = self.new_checkpoint();

        let maybe_call_with_command_args = None::<Box<Node<'a>>>
            .or_else(|| {
                let fcall = self.try_fcall();
                let command_args = self.parse_command_args();
                todo!("call_method {:?} {:?}", fcall, command_args)
            })
            .or_else(|| {
                let primary_value = self.try_primary_value()?;
                let maybe_op_t = None
                    .or_else(|| self.try_token(TokenValue::tCOLON2))
                    .or_else(|| self.try_operation2());
                if let Some(op_t) = maybe_op_t {
                    if let Some(operation2) = self.try_operation2() {
                        let command_args = self.parse_command_args();
                        todo!(
                            "return call_method {:?} {:?} {:?} {:?}",
                            primary_value,
                            op_t,
                            operation2,
                            command_args
                        )
                    }
                }

                self.restore_checkpoint(checkpoint);
                None
            })
            .or_else(|| {
                let super_t = self.try_token(TokenValue::kSUPER)?;
                let command_args = self.parse_command_args();
                todo!("super {:?} {:?}", super_t, command_args)
            })
            .or_else(|| {
                let yield_t = self.try_token(TokenValue::kYIELD)?;
                let command_args = self.parse_command_args();
                todo!("yield {:?} {:?}", yield_t, command_args)
            });

        if let Some(call_with_command_args) = maybe_call_with_command_args {
            match &*call_with_command_args {
                Node::Super(_) | Node::ZSuper(_) | Node::Yield(_) => {
                    // these nodes can't take block
                    return Some(call_with_command_args);
                }
                _ => {
                    if let Some(cmd_brace_block) = try_cmd_brace_block(self) {
                        panic!(
                            "block_call {:?} {:?}",
                            call_with_command_args, cmd_brace_block
                        )
                    }
                }
            }
        }

        let checkpoint = self.new_checkpoint();
        let keyword_t = None
            .or_else(|| self.try_k_return())
            .or_else(|| self.try_token(TokenValue::kBREAK))
            .or_else(|| self.try_token(TokenValue::kNEXT))?;
        if let Some(call_args) = self.try_call_args() {
            todo!("keyword_cmd {:?} {:?}", keyword_t, call_args)
        } else {
            self.restore_checkpoint(checkpoint);
            None
        }
    }

    pub(crate) fn parse_command_args(&mut self) -> Vec<Node<'a>> {
        todo!("parser.parse_command_args")
    }

    pub(crate) fn try_brace_body(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_brace_body")
    }

    pub(crate) fn try_call_args(&mut self) -> Option<Vec<Node<'a>>> {
        todo!("parser.try_call_args")
    }
}

struct CommandTail<'a> {
    command_args: Vec<Node<'a>>,
    cmd_brace_block: Option<CmdBraceBlock<'a>>,
}

fn try_command_tail<'a, C: Constructor>(parser: &mut Parser<'a, C>) -> CommandTail<'a> {
    let command_args = parser.parse_command_args();
    let cmd_brace_block = try_cmd_brace_block(parser);
    CommandTail {
        command_args,
        cmd_brace_block,
    }
}

#[derive(Debug)]
struct CmdBraceBlock<'a> {
    begin_t: Token<'a>,
    brace_body: Option<Box<Node<'a>>>,
    end_t: Token<'a>,
}

fn try_cmd_brace_block<'a, C: Constructor>(
    parser: &mut Parser<'a, C>,
) -> Option<CmdBraceBlock<'a>> {
    let begin_t = parser.try_token(TokenValue::tLCURLY)?;
    if let Some(brace_body) = parser.try_brace_body() {
        let end_t = parser.expect_token(TokenValue::tRCURLY);
        Some(CmdBraceBlock {
            begin_t,
            brace_body: Some(brace_body),
            end_t,
        })
    } else {
        let end_t = parser.expect_token(TokenValue::tRCURLY);
        Some(CmdBraceBlock {
            begin_t,
            brace_body: None,
            end_t,
        })
    }
}