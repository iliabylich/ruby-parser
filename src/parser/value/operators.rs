use crate::{Token, TokenKind};

pub(crate) fn prefix_operator_power(token: Token) -> Option<(u8, u8)> {
    match token.kind {
        TokenKind::tDOT2
        | TokenKind::tDOT3
        | TokenKind::tPLUS
        | TokenKind::tMINUS
        | TokenKind::tBANG
        | TokenKind::tTILDE
        | TokenKind::kNOT
        | TokenKind::kDEFINED => token.kind.precedence(),
        _ => None,
    }
}

pub(crate) fn binary_operator_power(token: Token) -> Option<(u8, u8)> {
    match token.kind {
        // Assignments that are parsed as "binary operators"
        TokenKind::tEQL | TokenKind::tOP_ASGN => return token.kind.precedence(),
        _ => {}
    }
    match token.kind {
        // Standard binary operators
        TokenKind::tDOT2
        | TokenKind::tDOT3
        | TokenKind::tPLUS
        | TokenKind::tMINUS
        | TokenKind::tSTAR
        | TokenKind::tDIVIDE
        | TokenKind::tPERCENT
        | TokenKind::tDSTAR
        | TokenKind::tPIPE
        | TokenKind::tCARET
        | TokenKind::tAMPER
        | TokenKind::tCMP
        | TokenKind::tEQ
        | TokenKind::tEQQ
        | TokenKind::tNEQ
        | TokenKind::tMATCH
        | TokenKind::tNMATCH
        | TokenKind::tLSHFT
        | TokenKind::tRSHFT
        | TokenKind::tANDOP
        | TokenKind::tOROP
        | TokenKind::tGT
        | TokenKind::tLT
        | TokenKind::tGEQ
        | TokenKind::tLEQ => return token.kind.precedence(),
        _ => {}
    }

    match token.kind {
        // Ternary operator that is also a "binary operator"
        TokenKind::tEH => return token.kind.precedence(),
        _ => {}
    }

    match token.kind {
        // Logical 'and'/'or'
        TokenKind::kOR | TokenKind::kAND => return token.kind.precedence(),
        _ => {}
    }

    match token.kind {
        // One-line pattern-matching
        TokenKind::tASSOC | TokenKind::kIN => return token.kind.precedence(),
        _ => {}
    }

    match token.kind {
        // Some keywords modifiers are also a "binary operators"
        TokenKind::kIF
        | TokenKind::kUNLESS
        | TokenKind::kWHILE
        | TokenKind::kUNTIL
        | TokenKind::kRESCUE => token.kind.precedence(),
        _ => None,
    }
}

pub(crate) fn postfix_operator_power(_token: Token) -> Option<(u8, u8)> {
    None
}
