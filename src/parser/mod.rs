use crate::builder::{Constructor, RustConstructor};
use crate::lexer::Lexer;
use crate::nodes::Node;
use crate::token::{Token, TokenValue};

pub struct Parser<'a, C: Constructor = RustConstructor> {
    lexer: Lexer<'a>,
    debug: bool,
    phantom: std::marker::PhantomData<C>,
}
pub type RustParser<'a> = Parser<'a, RustConstructor>;

macro_rules! current_token_is {
    ($parser:expr, $token:pat) => {
        matches!($parser.current_token().value(), $token)
    };
}
pub(crate) use current_token_is;

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            lexer: Lexer::new(input),
            debug: false,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self.lexer = self.lexer.debug();
        self
    }

    pub(crate) fn current_token(&mut self) -> &Token<'a> {
        self.lexer.current_token()
    }
    pub(crate) fn skip_token(&mut self) {
        self.lexer.skip_token()
    }
    pub(crate) fn take_token(&mut self) -> Token<'a> {
        self.lexer.take_token()
    }

    pub(crate) fn expect_token(&mut self, expected: TokenValue<'a>) -> Token<'a> {
        if self.current_token().value() == &expected {
            self.take_token()
        } else {
            panic!(
                "expected token {:?}, got {:?}",
                expected,
                self.current_token()
            )
        }
    }

    pub(crate) fn try_token(&mut self, expected: TokenValue<'a>) -> Option<Token<'a>> {
        if self.current_token().value() == &expected {
            Some(self.take_token())
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Node<'a> {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Node<'a> {
        self.parse_top_compstmt()
    }
}

mod alias;
mod gvar;
mod postexe;
mod preexe;
mod stmt;
mod undef;

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    fn parse_command_asgn(&mut self) -> Option<Box<Node<'a>>> {
        todo!()
    }

    fn parse_command_rhs(&mut self) -> Node<'a> {
        todo!()
    }

    fn parse_expr(&mut self) -> Option<Box<Node<'a>>> {
        if let Some(command_call) = self.parse_command_call() {
            return Some(command_call);
        }

        todo!()
    }

    fn parse_def_name(&mut self) -> Option<Token<'a>> {
        self.parse_fname()
    }

    fn parse_defn_head(&mut self) -> (Token<'a>, Token<'a>) {
        todo!()
    }

    fn parse_defs_head(&mut self) -> (Token<'a>, Node<'a>, Token<'a>, Token<'a>) {
        todo!()
    }

    fn parse_expr_value(&mut self) -> Option<Box<Node<'a>>> {
        todo!()
    }

    fn parse_expr_value_do(&mut self) -> Option<(Node<'a>, Token<'a>)> {
        todo!()
    }

    fn parse_command_call(&mut self) -> Option<Box<Node<'a>>> {
        todo!()
    }

    fn parse_block_command(&mut self) {
        todo!()
    }

    fn cmd_brace_block(&mut self) {
        todo!()
    }

    fn parse_fcall(&mut self) {
        todo!()
    }

    fn parse_command(&mut self) {
        todo!()
    }

    fn parse_mlhs(&mut self) -> Option<Box<Node<'a>>> {
        todo!()
    }

    fn mlhs_inner(&mut self) {
        todo!()
    }

    fn mlhs_basic(&mut self) {
        todo!()
    }

    fn parse_mlhs_item(&mut self) {
        todo!()
    }
    fn parse_mlhs_head(&mut self) {
        todo!()
    }
    fn parse_mlhs_post(&mut self) {
        todo!()
    }
    fn parse_mlhs_node(&mut self) {
        todo!()
    }
    fn parse_lhs(&mut self) {
        todo!()
    }
    fn parse_cname(&mut self) {
        todo!()
    }
    fn parse_cpath(&mut self) {
        todo!()
    }
    fn parse_fname(&mut self) -> Option<Token<'a>> {
        self.try_token(TokenValue::tIDENTIFIER)
            .or_else(|| self.try_token(TokenValue::tCONSTANT))
            .or_else(|| self.try_token(TokenValue::tFID))
            .or_else(|| self.parse_op())
            .or_else(|| self.parse_reswords())
    }
    fn parse_fitem(&mut self) -> Option<Box<Node<'a>>> {
        self.parse_fname()
            .map(|token| panic!("symbol_internal {:?}", token))
            .or_else(|| self.parse_symbol())
    }

    fn parse_op(&mut self) -> Option<Token<'a>> {
        None.or_else(|| self.try_token(TokenValue::tPIPE))
            .or_else(|| self.try_token(TokenValue::tCARET))
            .or_else(|| self.try_token(TokenValue::tAMPER))
            .or_else(|| self.try_token(TokenValue::tCMP))
            .or_else(|| self.try_token(TokenValue::tEQ))
            .or_else(|| self.try_token(TokenValue::tEQQ))
            .or_else(|| self.try_token(TokenValue::tMATCH))
            .or_else(|| self.try_token(TokenValue::tNMATCH))
            .or_else(|| self.try_token(TokenValue::tGT))
            .or_else(|| self.try_token(TokenValue::tGEQ))
            .or_else(|| self.try_token(TokenValue::tLT))
            .or_else(|| self.try_token(TokenValue::tLEQ))
            .or_else(|| self.try_token(TokenValue::tNEQ))
            .or_else(|| self.try_token(TokenValue::tLSHFT))
            .or_else(|| self.try_token(TokenValue::tRSHFT))
            .or_else(|| self.try_token(TokenValue::tPLUS))
            .or_else(|| self.try_token(TokenValue::tMINUS))
            .or_else(|| self.try_token(TokenValue::tSTAR))
            .or_else(|| self.try_token(TokenValue::tSTAR))
            .or_else(|| self.try_token(TokenValue::tDIVIDE))
            .or_else(|| self.try_token(TokenValue::tPERCENT))
            .or_else(|| self.try_token(TokenValue::tPOW))
            .or_else(|| self.try_token(TokenValue::tDSTAR))
            .or_else(|| self.try_token(TokenValue::tBANG))
            .or_else(|| self.try_token(TokenValue::tTILDE))
            .or_else(|| self.try_token(TokenValue::tUPLUS))
            .or_else(|| self.try_token(TokenValue::tUMINUS))
            .or_else(|| self.try_token(TokenValue::tAREF))
            .or_else(|| self.try_token(TokenValue::tASET))
            .or_else(|| self.try_token(TokenValue::tBACK_REF))
    }
    fn parse_reswords(&mut self) -> Option<Token<'a>> {
        None.or_else(|| self.try_token(TokenValue::k__LINE__))
            .or_else(|| self.try_token(TokenValue::k__FILE__))
            .or_else(|| self.try_token(TokenValue::k__ENCODING__))
            .or_else(|| self.try_token(TokenValue::klBEGIN))
            .or_else(|| self.try_token(TokenValue::klEND))
            .or_else(|| self.try_token(TokenValue::kALIAS))
            .or_else(|| self.try_token(TokenValue::kAND))
            .or_else(|| self.try_token(TokenValue::kBEGIN))
            .or_else(|| self.try_token(TokenValue::kBREAK))
            .or_else(|| self.try_token(TokenValue::kCASE))
            .or_else(|| self.try_token(TokenValue::kCLASS))
            .or_else(|| self.try_token(TokenValue::kDEF))
            .or_else(|| self.try_token(TokenValue::kDEFINED))
            .or_else(|| self.try_token(TokenValue::kDO))
            .or_else(|| self.try_token(TokenValue::kELSE))
            .or_else(|| self.try_token(TokenValue::kELSIF))
            .or_else(|| self.try_token(TokenValue::kEND))
            .or_else(|| self.try_token(TokenValue::kENSURE))
            .or_else(|| self.try_token(TokenValue::kFALSE))
            .or_else(|| self.try_token(TokenValue::kFOR))
            .or_else(|| self.try_token(TokenValue::kIN))
            .or_else(|| self.try_token(TokenValue::kMODULE))
            .or_else(|| self.try_token(TokenValue::kNEXT))
            .or_else(|| self.try_token(TokenValue::kNIL))
            .or_else(|| self.try_token(TokenValue::kNOT))
            .or_else(|| self.try_token(TokenValue::kOR))
            .or_else(|| self.try_token(TokenValue::kREDO))
            .or_else(|| self.try_token(TokenValue::kRESCUE))
            .or_else(|| self.try_token(TokenValue::kRETRY))
            .or_else(|| self.try_token(TokenValue::kRETURN))
            .or_else(|| self.try_token(TokenValue::kSELF))
            .or_else(|| self.try_token(TokenValue::kSUPER))
            .or_else(|| self.try_token(TokenValue::kTHEN))
            .or_else(|| self.try_token(TokenValue::kTRUE))
            .or_else(|| self.try_token(TokenValue::kUNDEF))
            .or_else(|| self.try_token(TokenValue::kWHEN))
            .or_else(|| self.try_token(TokenValue::kYIELD))
            .or_else(|| self.try_token(TokenValue::kIF))
            .or_else(|| self.try_token(TokenValue::kUNLESS))
            .or_else(|| self.try_token(TokenValue::kWHILE))
            .or_else(|| self.try_token(TokenValue::kUNTIL))
    }
    fn parse_arg(&mut self) {
        todo!()
    }
    fn parse_relop(&mut self) {
        todo!()
    }
    fn parse_rel_expr(&mut self) {
        todo!()
    }
    fn parse_arg_value(&mut self) {
        todo!()
    }
    fn parse_aref_args(&mut self) {
        todo!()
    }
    fn parse_arg_rhs(&mut self) {
        todo!()
    }
    fn parse_paren_args(&mut self) {
        todo!()
    }
    fn parse_opt_paren_args(&mut self) {
        todo!()
    }
    fn parse_opt_call_args(&mut self) {
        todo!()
    }
    fn parse_call_args(&mut self) {
        todo!()
    }
    fn parse_command_args(&mut self) {
        todo!()
    }
    fn parse_block_arg(&mut self) {
        todo!()
    }
    fn parse_opt_block_arg(&mut self) {
        todo!()
    }
    fn parse_args(&mut self) {
        todo!()
    }
    fn parse_mrhs_arg(&mut self) -> Option<Box<Node<'a>>> {
        todo!()
    }
    fn parse_mrhs(&mut self) -> Option<Box<Node<'a>>> {
        todo!()
    }
    fn parse_primary(&mut self) {
        todo!()
    }
    fn parse_primary_value(&mut self) {
        todo!()
    }
    fn parse_k_begin(&mut self) {
        todo!()
    }
    fn parse_k_if(&mut self) {
        todo!()
    }
    fn parse_k_unless(&mut self) {
        todo!()
    }
    fn parse_k_while(&mut self) {
        todo!()
    }
    fn parse_k_until(&mut self) {
        todo!()
    }
    fn parse_k_case(&mut self) {
        todo!()
    }
    fn parse_k_for(&mut self) {
        todo!()
    }
    fn parse_k_class(&mut self) {
        todo!()
    }
    fn parse_k_module(&mut self) {
        todo!()
    }
    fn parse_k_def(&mut self) {
        todo!()
    }
    fn parse_k_do(&mut self) {
        todo!()
    }
    fn parse_k_do_block(&mut self) {
        todo!()
    }
    fn parse_k_rescue(&mut self) {
        todo!()
    }
    fn parse_k_ensure(&mut self) {
        todo!()
    }
    fn parse_k_when(&mut self) {
        todo!()
    }
    fn parse_k_else(&mut self) {
        todo!()
    }
    fn parse_k_elsif(&mut self) {
        todo!()
    }
    fn parse_k_end(&mut self) {
        todo!()
    }
    fn parse_k_return(&mut self) {
        todo!()
    }
    fn parse_then(&mut self) {
        todo!()
    }
    fn parse_do(&mut self) {
        todo!()
    }
    fn parse_if_tail(&mut self) {
        todo!()
    }
    fn parse_opt_else(&mut self) {
        todo!()
    }
    fn parse_for_var(&mut self) {
        todo!()
    }
    fn parse_f_marg(&mut self) {
        todo!()
    }
    fn parse_f_marg_list(&mut self) {
        todo!()
    }
    fn parse_f_margs(&mut self) {
        todo!()
    }
    fn parse_f_rest_marg(&mut self) {
        todo!()
    }
    fn parse_f_any_kwrest(&mut self) {
        todo!()
    }
    fn parse_f_eq(&mut self) {
        todo!()
    }
    fn parse_block_args_tail(&mut self) {
        todo!()
    }
    fn parse_opt_block_args_tail(&mut self) {
        todo!()
    }
    fn parse_excessed_comma(&mut self) {
        todo!()
    }
    fn parse_block_param(&mut self) {
        todo!()
    }
    fn parse_opt_block_param(&mut self) {
        todo!()
    }
    fn parse_block_param_def(&mut self) {
        todo!()
    }
    fn parse_opt_bv_decl(&mut self) {
        todo!()
    }
    fn parse_bv_decls(&mut self) {
        todo!()
    }
    fn parse_bvar(&mut self) {
        todo!()
    }
    fn parse_lambda(&mut self) {
        todo!()
    }
    fn parse_f_larglist(&mut self) {
        todo!()
    }
    fn parse_lambda_body(&mut self) {
        todo!()
    }
    fn parse_do_block(&mut self) {
        todo!()
    }
    fn parse_block_call(&mut self) {
        todo!()
    }
    fn parse_method_call(&mut self) {
        todo!()
    }
    fn parse_brace_block(&mut self) {
        todo!()
    }
    fn parse_brace_body(&mut self) {
        todo!()
    }
    fn parse_do_body(&mut self) {
        todo!()
    }
    fn parse_case_args(&mut self) {
        todo!()
    }
    fn parse_case_body(&mut self) {
        todo!()
    }
    fn parse_cases(&mut self) {
        todo!()
    }
    fn parse_p_case_body(&mut self) {
        todo!()
    }
    fn parse_p_cases(&mut self) {
        todo!()
    }
    fn parse_p_top_expr(&mut self) {
        todo!()
    }
    fn parse_p_top_expr_body(&mut self) {
        todo!()
    }
    fn parse_p_expr(&mut self) {
        todo!()
    }
    fn parse_p_as(&mut self) {
        todo!()
    }
    fn parse_p_alt(&mut self) {
        todo!()
    }
    fn parse_p_lparen(&mut self) {
        todo!()
    }
    fn parse_p_lbracket(&mut self) {
        todo!()
    }
    fn parse_p_expr_basic(&mut self) {
        todo!()
    }
    fn parse_p_args(&mut self) {
        todo!()
    }
    fn parse_p_args_head(&mut self) {
        todo!()
    }
    fn parse_p_args_tail(&mut self) {
        todo!()
    }
    fn parse_p_find(&mut self) {
        todo!()
    }
    fn parse_p_rest(&mut self) {
        todo!()
    }
    fn parse_p_args_post(&mut self) {
        todo!()
    }
    fn parse_p_arg(&mut self) {
        todo!()
    }
    fn parse_p_kwargs(&mut self) {
        todo!()
    }
    fn parse_p_kwarg(&mut self) {
        todo!()
    }
    fn parse_p_kw(&mut self) {
        todo!()
    }
    fn parse_p_kw_label(&mut self) {
        todo!()
    }
    fn parse_p_kwrest(&mut self) {
        todo!()
    }
    fn parse_p_kwnorest(&mut self) {
        todo!()
    }
    fn parse_p_any_kwrest(&mut self) {
        todo!()
    }
    fn parse_p_value(&mut self) {
        todo!()
    }
    fn parse_p_primitive(&mut self) {
        todo!()
    }
    fn parse_p_variable(&mut self) {
        todo!()
    }
    fn parse_p_var_ref(&mut self) {
        todo!()
    }
    fn parse_p_expr_ref(&mut self) {
        todo!()
    }
    fn parse_p_const(&mut self) {
        todo!()
    }
    fn parse_opt_rescue(&mut self) {
        todo!()
    }
    fn parse_exc_list(&mut self) {
        todo!()
    }
    fn parse_exc_var(&mut self) {
        todo!()
    }
    fn parse_opt_ensure(&mut self) {
        todo!()
    }
    fn parse_literal(&mut self) {
        todo!()
    }
    fn parse_strings(&mut self) {
        todo!()
    }
    fn parse_string(&mut self) {
        todo!()
    }
    fn parse_string1(&mut self) {
        todo!()
    }
    fn parse_xstring(&mut self) {
        todo!()
    }
    fn parse_regexp(&mut self) {
        todo!()
    }
    fn parse_words(&mut self) {
        todo!()
    }
    fn parse_word_list(&mut self) {
        todo!()
    }
    fn parse_word(&mut self) {
        todo!()
    }
    fn parse_symbols(&mut self) {
        todo!()
    }
    fn parse_symbol_list(&mut self) {
        todo!()
    }
    fn parse_qwords(&mut self) {
        todo!()
    }
    fn parse_qsymbols(&mut self) {
        todo!()
    }
    fn parse_qword_list(&mut self) {
        todo!()
    }
    fn parse_qsym_list(&mut self) {
        todo!()
    }
    fn parse_string_contents(&mut self) {
        todo!()
    }
    fn parse_xstring_contents(&mut self) {
        todo!()
    }
    fn parse_regexp_contents(&mut self) {
        todo!()
    }
    fn parse_string_content(&mut self) {
        todo!()
    }
    fn parse_string_dvar(&mut self) {
        todo!()
    }
    fn parse_symbol(&mut self) -> Option<Box<Node<'a>>> {
        if self.current_token().value() != &TokenValue::tSYMBEG {
            return None;
        }

        let t_symbeg = self.take_token();

        // try plain symbol (`sym`)
        let sym = self
            .parse_fname()
            .or_else(|| self.try_token(TokenValue::tIVAR))
            .or_else(|| self.try_token(TokenValue::tGVAR))
            .or_else(|| self.try_token(TokenValue::tCVAR));
        if let Some(sym) = sym {
            panic!("symbol {:?} {:?}", t_symbeg, sym)
        }

        // otherwise read `dsym`

        let string_contents = self.parse_string_contents();
        let t_string_end = self.expect_token(TokenValue::tSTRING_END);
        panic!(
            "symbol_compose {:?} {:?} {:?}",
            t_symbeg, string_contents, t_string_end
        )
    }
    fn parse_numeric(&mut self) {
        todo!()
    }
    fn parse_simple_numeric(&mut self) {
        todo!()
    }
    fn parse_nonlocal_var(&mut self) {
        todo!()
    }
    fn parse_user_variable(&mut self) {
        todo!()
    }
    fn parse_keyword_variable(&mut self) {
        todo!()
    }
    fn parse_var_ref(&mut self) {
        todo!()
    }
    fn parse_var_lhs(&mut self) {
        todo!()
    }
    fn parse_backref(&mut self) {
        todo!()
    }
    fn parse_superclass(&mut self) {
        todo!()
    }
    fn parse_f_opt_paren_args(&mut self) {
        todo!()
    }
    fn parse_f_paren_args(&mut self) {
        todo!()
    }
    fn parse_f_arglist(&mut self) {
        todo!()
    }
    fn parse_args_tail(&mut self) {
        todo!()
    }
    fn parse_opt_args_tail(&mut self) {
        todo!()
    }
    fn parse_f_args(&mut self) {
        todo!()
    }
    fn parse_args_forward(&mut self) {
        todo!()
    }
    fn parse_f_bad_arg(&mut self) {
        todo!()
    }
    fn parse_f_norm_arg(&mut self) {
        todo!()
    }
    fn parse_f_arg_asgn(&mut self) {
        todo!()
    }
    fn parse_f_arg_item(&mut self) {
        todo!()
    }
    fn parse_f_arg(&mut self) {
        todo!()
    }
    fn parse_f_label(&mut self) {
        todo!()
    }
    fn parse_f_kw(&mut self) {
        todo!()
    }
    fn parse_f_block_kw(&mut self) {
        todo!()
    }
    fn parse_f_block_kwarg(&mut self) {
        todo!()
    }
    fn parse_f_kwarg(&mut self) {
        todo!()
    }
    fn parse_kwrest_mark(&mut self) {
        todo!()
    }
    fn parse_f_no_kwarg(&mut self) {
        todo!()
    }
    fn parse_f_kwrest(&mut self) {
        todo!()
    }
    fn parse_f_opt(&mut self) {
        todo!()
    }
    fn parse_f_block_opt(&mut self) {
        todo!()
    }
    fn parse_f_block_optarg(&mut self) {
        todo!()
    }
    fn parse_f_optarg(&mut self) {
        todo!()
    }
    fn parse_restarg_mark(&mut self) {
        todo!()
    }
    fn parse_f_rest_arg(&mut self) {
        todo!()
    }
    fn parse_blkarg_mark(&mut self) {
        todo!()
    }
    fn parse_f_block_arg(&mut self) {
        todo!()
    }
    fn parse_opt_f_block_arg(&mut self) {
        todo!()
    }
    fn parse_singleton(&mut self) {
        todo!()
    }
    fn parse_assoc_list(&mut self) {
        todo!()
    }
    fn parse_assocs(&mut self) {
        todo!()
    }
    fn parse_assoc(&mut self) {
        todo!()
    }
    fn parse_operation(&mut self) {
        todo!()
    }
    fn parse_operation2(&mut self) {
        todo!()
    }
    fn parse_operation3(&mut self) {
        todo!()
    }
    fn parse_dot_or_colon(&mut self) {
        todo!()
    }
    fn parse_call_op(&mut self) {
        todo!()
    }
    fn parse_call_op2(&mut self) {
        todo!()
    }
    fn parse_opt_terms(&mut self) {
        while matches!(
            self.current_token().value(),
            TokenValue::tSEMI | TokenValue::tNL
        ) {
            self.skip_token()
        }
    }
    fn parse_opt_nl(&mut self) {
        todo!()
    }
    fn parse_rparen(&mut self) {
        todo!()
    }
    fn parse_rbracket(&mut self) {
        todo!()
    }
    fn parse_rbrace(&mut self) {
        todo!()
    }
    fn parse_trailer(&mut self) {
        todo!()
    }
    fn parse_term(&mut self) {
        todo!()
    }
    fn parse_terms(&mut self) {
        todo!()
    }
    fn parse_none(&mut self) {
        todo!()
    }
}
