use crate::{Parser, Token, TokenKind};

impl Parser {
    pub(crate) fn try_token2(&mut self, kind: TokenKind) -> Option<Token> {
        if self.current_token().is(kind) {
            let token = self.current_token();
            self.skip_token();
            Some(token)
        } else {
            None
        }
    }

    pub(crate) fn try_operation_t(&mut self) -> Option<Token> {
        None.or_else(|| try_id_or_const_t(self))
            .or_else(|| self.try_token2(TokenKind::tFID))
    }

    pub(crate) fn try_operation2_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_operation_t())
            .or_else(|| try_op_t(self))
    }

    pub(crate) fn try_operation3_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token2(TokenKind::tIDENTIFIER))
            .or_else(|| self.try_token2(TokenKind::tFID))
            .or_else(|| try_op_t(self))
    }

    pub(crate) fn try_fname_t(&mut self) -> Option<Token> {
        None.or_else(|| try_id_or_const_t(self))
            .or_else(|| self.try_token2(TokenKind::tFID))
            .or_else(|| try_op_t(self))
            .or_else(|| try_reswods_t(self))
    }

    pub(crate) fn try_simple_numeric_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token2(TokenKind::tINTEGER))
            .or_else(|| self.try_token2(TokenKind::tFLOAT))
            .or_else(|| self.try_token2(TokenKind::tRATIONAL))
            .or_else(|| self.try_token2(TokenKind::tIMAGINARY))
    }

    pub(crate) fn try_user_variable_t(&mut self) -> Option<Token> {
        None.or_else(|| try_id_or_const_t(self))
            .or_else(|| try_nonlocal_var_t(self))
    }

    pub(crate) fn try_keyword_variable_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token2(TokenKind::kNIL))
            .or_else(|| self.try_token2(TokenKind::kSELF))
            .or_else(|| self.try_token2(TokenKind::kTRUE))
            .or_else(|| self.try_token2(TokenKind::kFALSE))
            .or_else(|| self.try_token2(TokenKind::k__FILE__))
            .or_else(|| self.try_token2(TokenKind::k__LINE__))
            .or_else(|| self.try_token2(TokenKind::k__ENCODING__))
    }

    pub(crate) fn try_var_ref_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_user_variable_t())
            .or_else(|| self.try_keyword_variable_t())
    }

    pub(crate) fn try_backref_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token2(TokenKind::tNTH_REF))
            .or_else(|| self.try_token2(TokenKind::tBACK_REF))
    }

    pub(crate) fn try_cname_t(&mut self) -> Option<Token> {
        try_id_or_const_t(self)
    }

    pub(crate) fn try_string_dvar_t(&mut self) -> Option<Token> {
        None.or_else(|| try_nonlocal_var_t(self))
            .or_else(|| self.try_fname_t())
    }

    pub(crate) fn try_sym_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_fname_t())
            .or_else(|| try_nonlocal_var_t(self))
    }

    pub(crate) fn try_call_op_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token2(TokenKind::tDOT))
            .or_else(|| self.try_token2(TokenKind::tANDDOT))
    }

    pub(crate) fn try_call_opt2_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_call_op_t())
            .or_else(|| self.try_token2(TokenKind::tCOLON2))
    }

    pub(crate) fn try_method_name_t(&mut self) -> Option<Token> {
        try_id_or_const_t(self)
    }

    pub(crate) fn try_do_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_term_t())
            .or_else(|| self.try_token2(TokenKind::kDO))
    }

    pub(crate) fn try_term_t(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token2(TokenKind::tSEMI))
            .or_else(|| self.try_token2(TokenKind::tNL))
    }
}

fn try_op_t(parser: &mut Parser) -> Option<Token> {
    None.or_else(|| parser.try_token2(TokenKind::tPIPE))
        .or_else(|| parser.try_token2(TokenKind::tCARET))
        .or_else(|| parser.try_token2(TokenKind::tAMPER))
        .or_else(|| parser.try_token2(TokenKind::tCMP))
        .or_else(|| parser.try_token2(TokenKind::tEQ))
        .or_else(|| parser.try_token2(TokenKind::tEQQ))
        .or_else(|| parser.try_token2(TokenKind::tMATCH))
        .or_else(|| parser.try_token2(TokenKind::tNMATCH))
        .or_else(|| parser.try_token2(TokenKind::tGT))
        .or_else(|| parser.try_token2(TokenKind::tGEQ))
        .or_else(|| parser.try_token2(TokenKind::tLT))
        .or_else(|| parser.try_token2(TokenKind::tLEQ))
        .or_else(|| parser.try_token2(TokenKind::tNEQ))
        .or_else(|| parser.try_token2(TokenKind::tLSHFT))
        .or_else(|| parser.try_token2(TokenKind::tRSHFT))
        .or_else(|| parser.try_token2(TokenKind::tPLUS))
        .or_else(|| parser.try_token2(TokenKind::tMINUS))
        .or_else(|| parser.try_token2(TokenKind::tSTAR))
        .or_else(|| parser.try_token2(TokenKind::tSTAR))
        .or_else(|| parser.try_token2(TokenKind::tDIVIDE))
        .or_else(|| parser.try_token2(TokenKind::tPERCENT))
        .or_else(|| parser.try_token2(TokenKind::tDSTAR))
        .or_else(|| parser.try_token2(TokenKind::tBANG))
        .or_else(|| parser.try_token2(TokenKind::tTILDE))
        .or_else(|| parser.try_token2(TokenKind::tUPLUS))
        .or_else(|| parser.try_token2(TokenKind::tUMINUS))
        .or_else(|| parser.try_token2(TokenKind::tAREF))
        .or_else(|| parser.try_token2(TokenKind::tASET))
        .or_else(|| parser.try_token2(TokenKind::tBACK_REF))
}

fn try_reswods_t(parser: &mut Parser) -> Option<Token> {
    None.or_else(|| parser.try_token2(TokenKind::k__LINE__))
        .or_else(|| parser.try_token2(TokenKind::k__FILE__))
        .or_else(|| parser.try_token2(TokenKind::k__ENCODING__))
        .or_else(|| parser.try_token2(TokenKind::klBEGIN))
        .or_else(|| parser.try_token2(TokenKind::klEND))
        .or_else(|| parser.try_token2(TokenKind::kALIAS))
        .or_else(|| parser.try_token2(TokenKind::kAND))
        .or_else(|| parser.try_token2(TokenKind::kBEGIN))
        .or_else(|| parser.try_token2(TokenKind::kBREAK))
        .or_else(|| parser.try_token2(TokenKind::kCASE))
        .or_else(|| parser.try_token2(TokenKind::kCLASS))
        .or_else(|| parser.try_token2(TokenKind::kDEF))
        .or_else(|| parser.try_token2(TokenKind::kDEFINED))
        .or_else(|| parser.try_token2(TokenKind::kDO))
        .or_else(|| parser.try_token2(TokenKind::kELSE))
        .or_else(|| parser.try_token2(TokenKind::kELSIF))
        .or_else(|| parser.try_token2(TokenKind::kEND))
        .or_else(|| parser.try_token2(TokenKind::kENSURE))
        .or_else(|| parser.try_token2(TokenKind::kFALSE))
        .or_else(|| parser.try_token2(TokenKind::kFOR))
        .or_else(|| parser.try_token2(TokenKind::kIN))
        .or_else(|| parser.try_token2(TokenKind::kMODULE))
        .or_else(|| parser.try_token2(TokenKind::kNEXT))
        .or_else(|| parser.try_token2(TokenKind::kNIL))
        .or_else(|| parser.try_token2(TokenKind::kNOT))
        .or_else(|| parser.try_token2(TokenKind::kOR))
        .or_else(|| parser.try_token2(TokenKind::kREDO))
        .or_else(|| parser.try_token2(TokenKind::kRESCUE))
        .or_else(|| parser.try_token2(TokenKind::kRETRY))
        .or_else(|| parser.try_token2(TokenKind::kRETURN))
        .or_else(|| parser.try_token2(TokenKind::kSELF))
        .or_else(|| parser.try_token2(TokenKind::kSUPER))
        .or_else(|| parser.try_token2(TokenKind::kTHEN))
        .or_else(|| parser.try_token2(TokenKind::kTRUE))
        .or_else(|| parser.try_token2(TokenKind::kUNDEF))
        .or_else(|| parser.try_token2(TokenKind::kWHEN))
        .or_else(|| parser.try_token2(TokenKind::kYIELD))
        .or_else(|| parser.try_token2(TokenKind::kIF))
        .or_else(|| parser.try_token2(TokenKind::kUNLESS))
        .or_else(|| parser.try_token2(TokenKind::kWHILE))
        .or_else(|| parser.try_token2(TokenKind::kUNTIL))
}

fn try_nonlocal_var_t(parser: &mut Parser) -> Option<Token> {
    None.or_else(|| parser.try_token2(TokenKind::tIVAR))
        .or_else(|| parser.try_token2(TokenKind::tGVAR))
        .or_else(|| parser.try_token2(TokenKind::tCVAR))
}

fn try_id_or_const_t(parser: &mut Parser) -> Option<Token> {
    None.or_else(|| parser.try_token2(TokenKind::tIDENTIFIER))
        .or_else(|| parser.try_token2(TokenKind::tCONSTANT))
}
