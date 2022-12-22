use crate::token::TokenKind;

macro_rules! left_assoc {
    ($n:expr) => {
        Some(($n * 2, $n * 2 + 1))
    };
}

macro_rules! right_assoc {
    ($n:expr) => {
        Some(($n * 2 + 1, $n * 2))
    };
}

macro_rules! non_assoc {
    ($n:expr) => {
        left_assoc!($n)
    };
}

impl TokenKind {
    pub(crate) fn precedence(&self) -> Option<(u8, u8)> {
        use TokenKind::*;

        match self {
            kIF | kUNLESS | kWHILE | kUNTIL | kIN => non_assoc!(1),
            kOR | kAND => left_assoc!(2),
            kNOT => right_assoc!(3),
            kDEFINED => non_assoc!(4),
            tEQL | tOP_ASGN => right_assoc!(5),
            kRESCUE => left_assoc!(6),
            tEH | tCOLON => right_assoc!(7),
            tDOT2 | tDOT3 => non_assoc!(8),
            tOROP => left_assoc!(9),
            tANDOP => left_assoc!(10),
            tCMP | tEQ | tEQQ | tNEQ | tMATCH | tNMATCH => non_assoc!(11),
            tGT | tGEQ | tLT | tLEQ => left_assoc!(12),
            tPIPE | tCARET => left_assoc!(13),
            tAMPER => left_assoc!(14),
            tLSHFT | tRSHFT => left_assoc!(15),
            tPLUS | tMINUS => left_assoc!(16),
            tSTAR | tDIVIDE | tPERCENT => left_assoc!(17),
            tUMINUS_NUM | tUMINUS => right_assoc!(18),
            tDSTAR => right_assoc!(19),
            tBANG | tTILDE | tUPLUS => right_assoc!(20),
            _ => None,
        }
    }
}
