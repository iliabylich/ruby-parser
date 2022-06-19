mod slash_u;
pub(crate) use slash_u::{SlashU, SlashUError, SlashUPerCodepointError};

mod slash_octal;
pub(crate) use slash_octal::{SlashOctal, SlashOctalError};

mod slash_x;
pub(crate) use slash_x::{SlashX, SlashXError};

mod slash_meta_ctrl;
pub(crate) use slash_meta_ctrl::{SlashMetaCtrl, SlashMetaCtrlError};

mod slash_byte;
pub(crate) use slash_byte::{SlashByte, SlashByteError};

pub(crate) enum Escape {
    SlashU(SlashU),
    SlashOctal(SlashOctal),
    SlashX(SlashX),
    SlashMetaCtrl(SlashMetaCtrl),
    SlashByte(SlashByte),
}

pub(crate) enum EscapeError {
    SlashUError(SlashUError),
    SlashOctalError(SlashOctalError),
    SlashXError(SlashXError),
    SlashMetaCtrlError(SlashMetaCtrlError),
    SlashByteError(SlashByteError),
}

use crate::lexer::buffer::{Buffer, Lookahead};

impl<'a> Lookahead<'a> for Escape {
    type Output = Result<Option<Self>, EscapeError>;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
        // check \u
        let maybe_slash_u = SlashU::lookahead(buffer, start).map_err(EscapeError::SlashUError)?;
        if let Some(slash_u) = maybe_slash_u {
            return Ok(Some(Escape::SlashU(slash_u)));
        }

        // check \777
        let maybe_slash_octal =
            SlashOctal::lookahead(buffer, start).map_err(EscapeError::SlashOctalError)?;
        if let Some(slash_octal) = maybe_slash_octal {
            return Ok(Some(Escape::SlashOctal(slash_octal)));
        }

        // check \xFF
        let maybe_slash_x = SlashX::lookahead(buffer, start).map_err(EscapeError::SlashXError)?;
        if let Some(slash_x) = maybe_slash_x {
            return Ok(Some(Escape::SlashX(slash_x)));
        }

        // check \C-\M-f
        let maybe_slash_meta_ctrl =
            SlashMetaCtrl::lookahead(buffer, start).map_err(EscapeError::SlashMetaCtrlError)?;
        if let Some(slash_meta_ctrl) = maybe_slash_meta_ctrl {
            return Ok(Some(Escape::SlashMetaCtrl(slash_meta_ctrl)));
        }

        // check \<byte>
        let maybe_slash_byte =
            SlashByte::lookahead(buffer, start).map_err(EscapeError::SlashByteError)?;
        if let Some(slash_byte) = maybe_slash_byte {
            return Ok(Some(Escape::SlashByte(slash_byte)));
        }

        Ok(None)
    }
}

pub(crate) fn unescape_byte(byte: u8) -> u8 {
    match byte {
        b'a' => 7,      // ?\a
        b'b' => 8,      // ?\b
        b'e' => 27,     // ?\e
        b'f' => 12,     // ?\f
        b'n' => 10,     // ?\n
        b'r' => 13,     // ?\r
        b's' => 32,     // ?\s
        b't' => 9,      // ?\t
        b'v' => 11,     // ?\v
        b'\\' => b'\\', // ?\\,
        _ => byte,
    }
}
