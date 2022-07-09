use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::{Token, TokenKind},
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_command(&mut self) -> Option<Box<Node>> {
        let checkpoint = self.new_checkpoint();

        let maybe_call_with_command_args = None::<Box<Node>>
            .or_else(|| {
                let checkpoint = self.new_checkpoint();
                let fcall = self.try_fcall()?;
                if let Some(command_args) = self.try_command_args() {
                    todo!("call_method {:?} {:?}", fcall, command_args)
                } else {
                    self.restore_checkpoint(checkpoint);
                    None
                }
            })
            .or_else(|| {
                let primary_value = self.try_primary_value()?;
                let maybe_op_t = None
                    .or_else(|| self.try_token(TokenKind::tCOLON2))
                    .or_else(|| self.try_operation2());
                if let Some(op_t) = maybe_op_t {
                    if let Some(operation2) = self.try_operation2() {
                        if let Some(command_args) = self.try_command_args() {
                            todo!(
                                "return call_method {:?} {:?} {:?} {:?}",
                                primary_value,
                                op_t,
                                operation2,
                                command_args
                            )
                        }
                    }
                }

                self.restore_checkpoint(checkpoint);
                None
            })
            .or_else(|| {
                let checkpoint = self.new_checkpoint();
                let super_t = self.try_token(TokenKind::kSUPER)?;
                if let Some(command_args) = self.try_command_args() {
                    todo!("super {:?} {:?}", super_t, command_args)
                } else {
                    self.restore_checkpoint(checkpoint);
                    None
                }
            })
            .or_else(|| {
                let checkpoint = self.new_checkpoint();
                let yield_t = self.try_token(TokenKind::kYIELD)?;
                if let Some(command_args) = self.try_command_args() {
                    todo!("yield {:?} {:?}", yield_t, command_args)
                } else {
                    self.restore_checkpoint(checkpoint);
                    None
                }
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
            .or_else(|| self.try_token(TokenKind::kBREAK))
            .or_else(|| self.try_token(TokenKind::kNEXT))?;
        if let Some(call_args) = self.try_call_args() {
            todo!("keyword_cmd {:?} {:?}", keyword_t, call_args)
        } else {
            self.restore_checkpoint(checkpoint);
            None
        }
    }

    // This rule can be `none`
    pub(crate) fn try_command_args(&mut self) -> Option<Vec<Node>> {
        self.try_call_args()
    }

    pub(crate) fn try_brace_body(&mut self) -> Option<Box<Node>> {
        todo!("parser.try_brace_body")
    }

    // This rule can be `none`
    pub(crate) fn try_call_args(&mut self) -> Option<Vec<Node>> {
        todo!("parser.try_call_args")
    }
}

#[derive(Debug)]
struct CmdBraceBlock {
    begin_t: Token,
    brace_body: Option<Box<Node>>,
    end_t: Token,
}

fn try_cmd_brace_block<'a, C: Constructor>(parser: &mut Parser<'a, C>) -> Option<CmdBraceBlock> {
    let begin_t = parser.try_token(TokenKind::tLCURLY)?;
    if let Some(brace_body) = parser.try_brace_body() {
        let end_t = parser.expect_token(TokenKind::tRCURLY);
        Some(CmdBraceBlock {
            begin_t,
            brace_body: Some(brace_body),
            end_t,
        })
    } else {
        let end_t = parser.expect_token(TokenKind::tRCURLY);
        Some(CmdBraceBlock {
            begin_t,
            brace_body: None,
            end_t,
        })
    }
}
