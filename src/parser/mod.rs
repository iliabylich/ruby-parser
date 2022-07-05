use crate::builder::{Builder, Constructor, RustConstructor};
use crate::lexer::{buffer::Buffer, Lexer};
use crate::nodes::Node;
use crate::token::{Token, TokenValue};

mod checkpoint;

mod alias;
mod array;
mod case;
mod class;
mod defined;
mod for_loop;
mod hash;
mod if_unless;
mod keyword_variable;
mod method;
mod mlhs;
mod module;
mod numeric;
mod opt_else;
mod opt_ensure;
mod opt_rescue;
mod postexe;
mod preexe;
mod primary;
mod regexp;
mod stmt;
mod string;
mod symbol;
mod undef;
mod variables;
mod while_until;
mod xstring;
mod yield_;

pub struct Parser<'a, C: Constructor = RustConstructor> {
    lexer: Lexer<'a>,
    debug: bool,
    phantom: std::marker::PhantomData<C>,
}
pub type RustParser<'a> = Parser<'a, RustConstructor>;

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
        if self.current_token().is(expected) {
            Some(self.take_token())
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Option<Box<Node<'a>>> {
        self.try_program()
    }

    fn try_program(&mut self) -> Option<Box<Node<'a>>> {
        self.try_top_compstmt()
    }

    pub(crate) fn buffer(&self) -> &Buffer<'a> {
        self.lexer.buffer.for_lookahead()
    }
}

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    fn try_command_rhs(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_command_rhs")
    }

    fn try_expr(&mut self) -> Option<Box<Node<'a>>> {
        if let Some(command_call) = self.try_command_call() {
            return Some(command_call);
        }

        todo!("parser.try_expr")
    }

    fn try_def_name(&mut self) -> Option<Token<'a>> {
        self.try_fname()
    }

    fn parse_defn_head(&mut self) -> (Token<'a>, Token<'a>) {
        todo!("parser.parse_defn_head")
    }

    fn parse_defs_head(&mut self) -> (Token<'a>, Node<'a>, Token<'a>, Token<'a>) {
        todo!("parser.parse_defs_head")
    }

    fn try_expr_value(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_expr_value")
    }

    fn try_expr_value_do(&mut self) -> Option<(Node<'a>, Token<'a>)> {
        todo!("parser.try_expr_value_do")
    }

    fn try_command_call(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_command_call")
    }

    fn parse_block_command(&mut self) {
        todo!("parser.parse_block_command")
    }

    fn parse_cmd_brace_block(&mut self) {
        todo!("parser.parse_cmd_brace_block")
    }

    fn try_fcall(&mut self) -> Option<Token<'a>> {
        todo!("parser.try_fcall")
    }

    fn parse_command(&mut self) {
        todo!("parser.parse_command")
    }

    fn parse_cname(&mut self) {
        todo!("parser.parse_cname")
    }
    fn parse_cpath(&mut self) {
        todo!("parser.parse_cpath")
    }
    fn try_fname(&mut self) -> Option<Token<'a>> {
        self.try_token(TokenValue::tIDENTIFIER)
            .or_else(|| self.try_token(TokenValue::tCONSTANT))
            .or_else(|| self.try_token(TokenValue::tFID))
            .or_else(|| self.try_op())
            .or_else(|| self.try_reswords())
    }
    fn try_fitem(&mut self) -> Option<Box<Node<'a>>> {
        self.try_fname()
            .map(|token| Builder::<C>::symbol_internal(token, self.buffer()))
            .or_else(|| self.try_symbol())
    }

    fn try_op(&mut self) -> Option<Token<'a>> {
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
    fn try_reswords(&mut self) -> Option<Token<'a>> {
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
    fn try_arg(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_arg")
    }
    fn parse_relop(&mut self) {
        todo!("parser.parse_relop")
    }
    fn parse_rel_expr(&mut self) {
        todo!("parser.parse_rel_expr")
    }
    fn parse_arg_value(&mut self) {
        todo!("parser.parse_arg_value")
    }
    fn try_aref_args(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_aref_args")
    }
    fn parse_arg_rhs(&mut self) {
        todo!("parser.parse_arg_rhs")
    }
    fn parse_paren_args(&mut self) {
        todo!("parser.parse_paren_args")
    }
    fn parse_opt_paren_args(&mut self) {
        todo!("parser.parse_opt_paren_args")
    }
    fn parse_opt_call_args(&mut self) {
        todo!("parser.parse_opt_call_args")
    }
    fn parse_call_args(&mut self) -> Vec<Node<'a>> {
        todo!("parser.parse_call_args")
    }
    fn parse_command_args(&mut self) {
        todo!("parser.parse_command_args")
    }
    fn parse_block_arg(&mut self) {
        todo!("parser.parse_block_arg")
    }
    fn parse_opt_block_arg(&mut self) {
        todo!("parser.parse_opt_block_arg")
    }
    fn parse_args(&mut self) {
        todo!("parser.parse_args")
    }
    fn try_mrhs_arg(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_mrhs_arg")
    }
    fn try_mrhs(&mut self) -> Option<Vec<Node<'a>>> {
        todo!("parser.try_mrhs")
    }
    fn parse_k_begin(&mut self) {
        todo!("parser.parse_k_begin")
    }
    fn parse_k_if(&mut self) {
        todo!("parser.parse_k_if")
    }
    fn parse_k_unless(&mut self) {
        todo!("parser.parse_k_unless")
    }
    fn parse_k_while(&mut self) {
        todo!("parser.parse_k_while")
    }
    fn parse_k_until(&mut self) {
        todo!("parser.parse_k_until")
    }
    fn parse_k_case(&mut self) {
        todo!("parser.parse_k_case")
    }
    fn parse_k_for(&mut self) {
        todo!("parser.parse_k_for")
    }
    fn parse_k_class(&mut self) {
        todo!("parser.parse_k_class")
    }
    fn parse_k_module(&mut self) {
        todo!("parser.parse_k_module")
    }
    fn parse_k_def(&mut self) {
        todo!("parser.parse_k_def")
    }
    fn parse_k_do(&mut self) {
        todo!("parser.parse_k_do")
    }
    fn parse_k_do_block(&mut self) {
        todo!("parser.parse_k_do_block")
    }
    fn parse_k_rescue(&mut self) {
        todo!("parser.parse_k_rescue")
    }
    fn parse_k_ensure(&mut self) {
        todo!("parser.parse_k_ensure")
    }
    fn parse_k_when(&mut self) {
        todo!("parser.parse_k_when")
    }
    fn parse_k_else(&mut self) {
        todo!("parser.parse_k_else")
    }
    fn parse_k_elsif(&mut self) {
        todo!("parser.parse_k_elsif")
    }
    fn parse_k_end(&mut self) {
        todo!("parser.parse_k_end")
    }
    fn parse_k_return(&mut self) {
        todo!("parser.parse_k_return")
    }
    fn parse_then(&mut self) {
        todo!("parser.parse_then")
    }
    fn parse_do(&mut self) {
        todo!("parser.parse_do")
    }
    fn parse_if_tail(&mut self) {
        todo!("parser.parse_if_tail")
    }
    fn parse_opt_else(&mut self) {
        todo!("parser.parse_opt_else")
    }
    fn parse_for_var(&mut self) {
        todo!("parser.parse_for_var")
    }
    fn parse_f_marg(&mut self) {
        todo!("parser.parse_f_marg")
    }
    fn parse_f_marg_list(&mut self) {
        todo!("parser.parse_f_marg_list")
    }
    fn parse_f_margs(&mut self) {
        todo!("parser.parse_f_margs")
    }
    fn parse_f_rest_marg(&mut self) {
        todo!("parser.parse_f_rest_marg")
    }
    fn parse_f_any_kwrest(&mut self) {
        todo!("parser.parse_f_any_kwrest")
    }
    fn parse_f_eq(&mut self) {
        todo!("parser.parse_f_eq")
    }
    fn parse_block_args_tail(&mut self) {
        todo!("parser.parse_block_args_tail")
    }
    fn parse_opt_block_args_tail(&mut self) {
        todo!("parser.parse_opt_block_args_tail")
    }
    fn parse_excessed_comma(&mut self) {
        todo!("parser.parse_excessed_comma")
    }
    fn parse_block_param(&mut self) {
        todo!("parser.parse_block_param")
    }
    fn parse_opt_block_param(&mut self) {
        todo!("parser.parse_opt_block_param")
    }
    fn parse_block_param_def(&mut self) {
        todo!("parser.parse_block_param_def")
    }
    fn parse_opt_bv_decl(&mut self) {
        todo!("parser.parse_opt_bv_decl")
    }
    fn parse_bv_decls(&mut self) {
        todo!("parser.parse_bv_decls")
    }
    fn parse_bvar(&mut self) {
        todo!("parser.parse_bvar")
    }
    fn try_lambda(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_lambda")
    }
    fn parse_f_larglist(&mut self) {
        todo!("parser.parse_f_larglist")
    }
    fn parse_lambda_body(&mut self) {
        todo!("parser.parse_lambda_body")
    }
    fn parse_do_block(&mut self) {
        todo!("parser.parse_do_block")
    }
    fn parse_block_call(&mut self) {
        todo!("parser.parse_block_call")
    }
    fn try_method_call(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_method_call")
    }

    // TODO: return ArgsType instead of ()
    fn try_brace_block(&mut self) -> Option<(Token<'a>, (), Option<Box<Node<'a>>>, Token<'a>)> {
        todo!("parser.try_brace_block")
    }
    fn parse_brace_body(&mut self) {
        todo!("parser.parse_brace_body")
    }
    fn parse_do_body(&mut self) {
        todo!("parser.parse_do_body")
    }
    fn parse_case_args(&mut self) {
        todo!("parser.parse_case_args")
    }
    fn parse_case_body(&mut self) {
        todo!("parser.parse_case_body")
    }
    fn parse_cases(&mut self) {
        todo!("parser.parse_cases")
    }
    fn parse_p_case_body(&mut self) {
        todo!("parser.parse_p_case_body")
    }
    fn parse_p_cases(&mut self) {
        todo!("parser.parse_p_cases")
    }
    fn parse_p_top_expr(&mut self) {
        todo!("parser.parse_p_top_expr")
    }
    fn parse_p_top_expr_body(&mut self) {
        todo!("parser.parse_p_top_expr_body")
    }
    fn parse_p_expr(&mut self) {
        todo!("parser.parse_p_expr")
    }
    fn parse_p_as(&mut self) {
        todo!("parser.parse_p_as")
    }
    fn parse_p_alt(&mut self) {
        todo!("parser.parse_p_alt")
    }
    fn parse_p_lparen(&mut self) {
        todo!("parser.parse_p_lparen")
    }
    fn parse_p_lbracket(&mut self) {
        todo!("parser.parse_p_lbracket")
    }
    fn parse_p_expr_basic(&mut self) {
        todo!("parser.parse_p_expr_basic")
    }
    fn parse_p_args(&mut self) {
        todo!("parser.parse_p_args")
    }
    fn parse_p_args_head(&mut self) {
        todo!("parser.parse_p_args_head")
    }
    fn parse_p_args_tail(&mut self) {
        todo!("parser.parse_p_args_tail")
    }
    fn parse_p_find(&mut self) {
        todo!("parser.parse_p_find")
    }
    fn parse_p_rest(&mut self) {
        todo!("parser.parse_p_rest")
    }
    fn parse_p_args_post(&mut self) {
        todo!("parser.parse_p_args_post")
    }
    fn parse_p_arg(&mut self) {
        todo!("parser.parse_p_arg")
    }
    fn parse_p_kwargs(&mut self) {
        todo!("parser.parse_p_kwargs")
    }
    fn parse_p_kwarg(&mut self) {
        todo!("parser.parse_p_kwarg")
    }
    fn parse_p_kw(&mut self) {
        todo!("parser.parse_p_kw")
    }
    fn parse_p_kw_label(&mut self) {
        todo!("parser.parse_p_kw_label")
    }
    fn parse_p_kwrest(&mut self) {
        todo!("parser.parse_p_kwrest")
    }
    fn parse_p_kwnorest(&mut self) {
        todo!("parser.parse_p_kwnorest")
    }
    fn parse_p_any_kwrest(&mut self) {
        todo!("parser.parse_p_any_kwrest")
    }
    fn parse_p_value(&mut self) {
        todo!("parser.parse_p_value")
    }
    fn parse_p_primitive(&mut self) {
        todo!("parser.parse_p_primitive")
    }
    fn parse_p_variable(&mut self) {
        todo!("parser.parse_p_variable")
    }
    fn parse_p_var_ref(&mut self) {
        todo!("parser.parse_p_var_ref")
    }
    fn parse_p_expr_ref(&mut self) {
        todo!("parser.parse_p_expr_ref")
    }
    fn parse_p_const(&mut self) {
        todo!("parser.parse_p_const")
    }
    fn parse_opt_ensure(&mut self) -> Option<(Token<'a>, Box<Node<'a>>)> {
        todo!("parser.parse_opt_ensure")
    }
    fn try_literal(&mut self) -> Option<Box<Node<'a>>> {
        None.or_else(|| self.try_numeric())
            .or_else(|| self.try_symbol())
    }
    fn try_words(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_words")
    }
    fn parse_word_list(&mut self) {
        todo!("parser.parse_word_list")
    }
    fn parse_word(&mut self) {
        todo!("parser.parse_word")
    }
    fn try_symbols(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_symbols")
    }
    fn parse_symbol_list(&mut self) {
        todo!("parser.parse_symbol_list")
    }
    fn try_qwords(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_qwords")
    }
    fn try_qsymbols(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_qsymbols")
    }
    fn parse_qword_list(&mut self) {
        todo!("parser.parse_qword_list")
    }
    fn parse_qsym_list(&mut self) {
        todo!("parser.parse_qsym_list")
    }
    fn parse_regexp_contents(&mut self) {
        todo!("parser.parse_regexp_contents")
    }
    fn parse_nonlocal_var(&mut self) {
        todo!("parser.parse_nonlocal_var")
    }
    fn try_user_variable(&mut self) -> Option<Box<Node<'a>>> {
        None.or_else(|| self.try_lvar())
            .or_else(|| self.try_ivar())
            .or_else(|| self.try_gvar())
            .or_else(|| self.try_t_const())
            .or_else(|| self.try_cvar())
    }
    fn try_var_ref(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_var_ref")
    }
    fn try_var_lhs(&mut self) -> Option<Box<Node<'a>>> {
        None.or_else(|| self.try_user_variable())
            .or_else(|| self.try_keyword_variable())
            .map(|node| Builder::<C>::assignable(node))
    }
    fn parse_superclass(&mut self) {
        todo!("parser.parse_superclass")
    }
    fn parse_f_opt_paren_args(&mut self) {
        todo!("parser.parse_f_opt_paren_args")
    }
    fn parse_f_paren_args(&mut self) {
        todo!("parser.parse_f_paren_args")
    }
    fn parse_f_arglist(&mut self) {
        todo!("parser.parse_f_arglist")
    }
    fn parse_args_tail(&mut self) {
        todo!("parser.parse_args_tail")
    }
    fn parse_opt_args_tail(&mut self) {
        todo!("parser.parse_opt_args_tail")
    }
    fn parse_f_args(&mut self) {
        todo!("parser.parse_f_args")
    }
    fn parse_args_forward(&mut self) {
        todo!("parser.parse_args_forward")
    }
    fn parse_f_bad_arg(&mut self) {
        todo!("parser.parse_f_bad_arg")
    }
    fn parse_f_norm_arg(&mut self) {
        todo!("parser.parse_f_norm_arg")
    }
    fn parse_f_arg_asgn(&mut self) {
        todo!("parser.parse_f_arg_asgn")
    }
    fn parse_f_arg_item(&mut self) {
        todo!("parser.parse_f_arg_item")
    }
    fn parse_f_arg(&mut self) {
        todo!("parser.parse_f_arg")
    }
    fn parse_f_label(&mut self) {
        todo!("parser.parse_f_label")
    }
    fn parse_f_kw(&mut self) {
        todo!("parser.parse_f_kw")
    }
    fn parse_f_block_kw(&mut self) {
        todo!("parser.parse_f_block_kw")
    }
    fn parse_f_block_kwarg(&mut self) {
        todo!("parser.parse_f_block_kwarg")
    }
    fn parse_f_kwarg(&mut self) {
        todo!("parser.parse_f_kwarg")
    }
    fn parse_kwrest_mark(&mut self) {
        todo!("parser.parse_kwrest_mark")
    }
    fn parse_f_no_kwarg(&mut self) {
        todo!("parser.parse_f_no_kwarg")
    }
    fn parse_f_kwrest(&mut self) {
        todo!("parser.parse_f_kwrest")
    }
    fn parse_f_opt(&mut self) {
        todo!("parser.parse_f_opt")
    }
    fn parse_f_block_opt(&mut self) {
        todo!("parser.parse_f_block_opt")
    }
    fn parse_f_block_optarg(&mut self) {
        todo!("parser.parse_f_block_optarg")
    }
    fn parse_f_optarg(&mut self) {
        todo!("parser.parse_f_optarg")
    }
    fn parse_restarg_mark(&mut self) {
        todo!("parser.parse_restarg_mark")
    }
    fn parse_f_rest_arg(&mut self) {
        todo!("parser.parse_f_rest_arg")
    }
    fn parse_blkarg_mark(&mut self) {
        todo!("parser.parse_blkarg_mark")
    }
    fn parse_f_block_arg(&mut self) {
        todo!("parser.parse_f_block_arg")
    }
    fn parse_opt_f_block_arg(&mut self) {
        todo!("parser.parse_opt_f_block_arg")
    }
    fn parse_singleton(&mut self) {
        todo!("parser.parse_singleton")
    }
    fn parse_assoc_list(&mut self) -> Vec<Box<Node<'a>>> {
        todo!("parser.parse_assoc_list")
    }
    fn parse_assocs(&mut self) {
        todo!("parser.parse_assocs")
    }
    fn parse_assoc(&mut self) {
        todo!("parser.parse_assoc")
    }
    fn parse_operation(&mut self) {
        todo!("parser.parse_operation")
    }
    fn parse_operation2(&mut self) {
        todo!("parser.parse_operation2")
    }
    fn parse_operation3(&mut self) {
        todo!("parser.parse_operation3")
    }
    fn parse_dot_or_colon(&mut self) {
        todo!("parser.parse_dot_or_colon")
    }
    fn try_call_op(&mut self) -> Option<Token<'a>> {
        None.or_else(|| self.try_token(TokenValue::tDOT))
            .or_else(|| self.try_token(TokenValue::tANDDOT))
    }
    fn parse_call_op2(&mut self) {
        todo!("parser.parse_call_op2")
    }
    fn parse_opt_terms(&mut self) {
        while matches!(
            self.current_token().value(),
            TokenValue::tSEMI | TokenValue::tNL
        ) {
            self.skip_token()
        }
    }
    fn try_opt_nl(&mut self) -> Option<Token<'a>> {
        todo!("parser.try_opt_nl")
    }
    fn try_rparen(&mut self) -> Option<Token<'a>> {
        todo!("parser.try_rparen")
    }
    fn try_rbracket(&mut self) -> Option<Token<'a>> {
        todo!("parser.try_rbracket")
    }
    fn try_rbrace(&mut self) -> Option<Token<'a>> {
        todo!("parser.try_rbrace")
    }
    fn try_trailer(&mut self) -> Option<Token<'a>> {
        todo!("parser.try_trailer")
    }
    fn try_term(&mut self) -> Option<Token<'a>> {
        todo!("parser.try_term")
    }
    fn parse_terms(&mut self) {
        todo!("parser.parse_terms")
    }
    fn parse_none(&mut self) {
        todo!("parser.parse_none")
    }

    fn try_colon2_const(&mut self) -> Option<(Token<'a>, Token<'a>)> {
        let colon2_t = self.try_token(TokenValue::tCOLON2)?;
        let const_t = self.expect_token(TokenValue::tCONSTANT);
        Some((colon2_t, const_t))
    }
}
