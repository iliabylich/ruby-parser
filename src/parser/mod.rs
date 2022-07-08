use crate::builder::{Builder, Constructor, RustConstructor};
use crate::lexer::{buffer::Buffer, Lexer};
use crate::nodes::Node;
use crate::token::{Token, TokenKind};

mod checkpoint;

mod alias;
mod array;
mod case;
mod class;
mod command;
mod defined;
mod expr;
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
mod qsymbols;
mod qwords;
mod regexp;
mod stmt;
mod string;
mod symbol;
mod symbols;
mod undef;
mod variables;
mod while_until;
mod words;
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

    pub(crate) fn current_token(&mut self) -> &Token {
        self.lexer.current_token()
    }
    pub(crate) fn skip_token(&mut self) {
        self.lexer.skip_token()
    }
    pub(crate) fn take_token(&mut self) -> Token {
        self.lexer.take_token()
    }

    pub(crate) fn expect_token(&mut self, expected: TokenKind) -> Token {
        if self.current_token().kind() == &expected {
            self.take_token()
        } else {
            panic!(
                "expected token {:?}, got {:?}",
                expected,
                self.current_token()
            )
        }
    }

    pub(crate) fn try_token(&mut self, expected: TokenKind) -> Option<Token> {
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

    fn try_def_name(&mut self) -> Option<Token> {
        self.try_fname()
    }

    fn parse_defn_head(&mut self) -> (Token, Token) {
        todo!("parser.parse_defn_head")
    }

    fn parse_defs_head(&mut self) -> (Token, Node<'a>, Token, Token) {
        todo!("parser.parse_defs_head")
    }

    fn try_expr_value(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_expr_value")
    }

    fn try_expr_value_do(&mut self) -> Option<(Node<'a>, Token)> {
        todo!("parser.try_expr_value_do")
    }

    fn try_command_call(&mut self) -> Option<Box<Node<'a>>> {
        None.or_else(|| self.try_command())
            .or_else(|| self.try_block_command())
    }

    fn try_block_command(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_block_command")
    }

    fn parse_cmd_brace_block(&mut self) {
        todo!("parser.parse_cmd_brace_block")
    }

    fn try_fcall(&mut self) -> Option<Token> {
        self.try_operation()
    }

    fn parse_cname(&mut self) {
        todo!("parser.parse_cname")
    }
    fn parse_cpath(&mut self) {
        todo!("parser.parse_cpath")
    }
    fn try_fname(&mut self) -> Option<Token> {
        self.try_token(TokenKind::tIDENTIFIER)
            .or_else(|| self.try_token(TokenKind::tCONSTANT))
            .or_else(|| self.try_token(TokenKind::tFID))
            .or_else(|| self.try_op())
            .or_else(|| self.try_reswords())
    }
    fn try_fitem(&mut self) -> Option<Box<Node<'a>>> {
        self.try_fname()
            .map(|token| Builder::<C>::symbol_internal(token, self.buffer()))
            .or_else(|| self.try_symbol())
    }

    fn try_op(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token(TokenKind::tPIPE))
            .or_else(|| self.try_token(TokenKind::tCARET))
            .or_else(|| self.try_token(TokenKind::tAMPER))
            .or_else(|| self.try_token(TokenKind::tCMP))
            .or_else(|| self.try_token(TokenKind::tEQ))
            .or_else(|| self.try_token(TokenKind::tEQQ))
            .or_else(|| self.try_token(TokenKind::tMATCH))
            .or_else(|| self.try_token(TokenKind::tNMATCH))
            .or_else(|| self.try_token(TokenKind::tGT))
            .or_else(|| self.try_token(TokenKind::tGEQ))
            .or_else(|| self.try_token(TokenKind::tLT))
            .or_else(|| self.try_token(TokenKind::tLEQ))
            .or_else(|| self.try_token(TokenKind::tNEQ))
            .or_else(|| self.try_token(TokenKind::tLSHFT))
            .or_else(|| self.try_token(TokenKind::tRSHFT))
            .or_else(|| self.try_token(TokenKind::tPLUS))
            .or_else(|| self.try_token(TokenKind::tMINUS))
            .or_else(|| self.try_token(TokenKind::tSTAR))
            .or_else(|| self.try_token(TokenKind::tSTAR))
            .or_else(|| self.try_token(TokenKind::tDIVIDE))
            .or_else(|| self.try_token(TokenKind::tPERCENT))
            .or_else(|| self.try_token(TokenKind::tPOW))
            .or_else(|| self.try_token(TokenKind::tDSTAR))
            .or_else(|| self.try_token(TokenKind::tBANG))
            .or_else(|| self.try_token(TokenKind::tTILDE))
            .or_else(|| self.try_token(TokenKind::tUPLUS))
            .or_else(|| self.try_token(TokenKind::tUMINUS))
            .or_else(|| self.try_token(TokenKind::tAREF))
            .or_else(|| self.try_token(TokenKind::tASET))
            .or_else(|| self.try_token(TokenKind::tBACK_REF))
    }
    fn try_reswords(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token(TokenKind::k__LINE__))
            .or_else(|| self.try_token(TokenKind::k__FILE__))
            .or_else(|| self.try_token(TokenKind::k__ENCODING__))
            .or_else(|| self.try_token(TokenKind::klBEGIN))
            .or_else(|| self.try_token(TokenKind::klEND))
            .or_else(|| self.try_token(TokenKind::kALIAS))
            .or_else(|| self.try_token(TokenKind::kAND))
            .or_else(|| self.try_token(TokenKind::kBEGIN))
            .or_else(|| self.try_token(TokenKind::kBREAK))
            .or_else(|| self.try_token(TokenKind::kCASE))
            .or_else(|| self.try_token(TokenKind::kCLASS))
            .or_else(|| self.try_token(TokenKind::kDEF))
            .or_else(|| self.try_token(TokenKind::kDEFINED))
            .or_else(|| self.try_token(TokenKind::kDO))
            .or_else(|| self.try_token(TokenKind::kELSE))
            .or_else(|| self.try_token(TokenKind::kELSIF))
            .or_else(|| self.try_token(TokenKind::kEND))
            .or_else(|| self.try_token(TokenKind::kENSURE))
            .or_else(|| self.try_token(TokenKind::kFALSE))
            .or_else(|| self.try_token(TokenKind::kFOR))
            .or_else(|| self.try_token(TokenKind::kIN))
            .or_else(|| self.try_token(TokenKind::kMODULE))
            .or_else(|| self.try_token(TokenKind::kNEXT))
            .or_else(|| self.try_token(TokenKind::kNIL))
            .or_else(|| self.try_token(TokenKind::kNOT))
            .or_else(|| self.try_token(TokenKind::kOR))
            .or_else(|| self.try_token(TokenKind::kREDO))
            .or_else(|| self.try_token(TokenKind::kRESCUE))
            .or_else(|| self.try_token(TokenKind::kRETRY))
            .or_else(|| self.try_token(TokenKind::kRETURN))
            .or_else(|| self.try_token(TokenKind::kSELF))
            .or_else(|| self.try_token(TokenKind::kSUPER))
            .or_else(|| self.try_token(TokenKind::kTHEN))
            .or_else(|| self.try_token(TokenKind::kTRUE))
            .or_else(|| self.try_token(TokenKind::kUNDEF))
            .or_else(|| self.try_token(TokenKind::kWHEN))
            .or_else(|| self.try_token(TokenKind::kYIELD))
            .or_else(|| self.try_token(TokenKind::kIF))
            .or_else(|| self.try_token(TokenKind::kUNLESS))
            .or_else(|| self.try_token(TokenKind::kWHILE))
            .or_else(|| self.try_token(TokenKind::kUNTIL))
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
    fn try_k_begin(&mut self) -> Option<Token> {
        todo!("parser.try_k_begin")
    }
    fn try_k_if(&mut self) -> Option<Token> {
        todo!("parser.try_k_if")
    }
    fn try_k_unless(&mut self) -> Option<Token> {
        todo!("parser.try_k_unless")
    }
    fn try_k_while(&mut self) -> Option<Token> {
        todo!("parser.try_k_while")
    }
    fn try_k_until(&mut self) -> Option<Token> {
        todo!("parser.try_k_until")
    }
    fn try_k_case(&mut self) -> Option<Token> {
        todo!("parser.try_k_case")
    }
    fn try_k_for(&mut self) -> Option<Token> {
        todo!("parser.try_k_for")
    }
    fn try_k_class(&mut self) -> Option<Token> {
        todo!("parser.try_k_class")
    }
    fn try_k_module(&mut self) -> Option<Token> {
        todo!("parser.try_k_module")
    }
    fn try_k_def(&mut self) -> Option<Token> {
        todo!("parser.try_k_def")
    }
    fn try_k_do(&mut self) -> Option<Token> {
        todo!("parser.try_k_do")
    }
    fn try_k_do_block(&mut self) -> Option<Token> {
        todo!("parser.try_k_do_block")
    }
    fn try_k_rescue(&mut self) -> Option<Token> {
        todo!("parser.try_k_rescue")
    }
    fn try_k_ensure(&mut self) -> Option<Token> {
        todo!("parser.try_k_ensure")
    }
    fn try_k_when(&mut self) -> Option<Token> {
        todo!("parser.try_k_when")
    }
    fn try_k_else(&mut self) -> Option<Token> {
        todo!("parser.try_k_else")
    }
    fn try_k_elsif(&mut self) -> Option<Token> {
        todo!("parser.try_k_elsif")
    }
    fn try_k_end(&mut self) -> Option<Token> {
        todo!("parser.try_k_end")
    }
    fn try_k_return(&mut self) -> Option<Token> {
        todo!("parser.try_k_return")
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
    fn try_block_call(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_block_call")
    }
    fn try_method_call(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_method_call")
    }

    // TODO: return ArgsType instead of ()
    fn try_brace_block(&mut self) -> Option<(Token, (), Option<Box<Node<'a>>>, Token)> {
        todo!("parser.try_brace_block")
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
    fn try_p_top_expr_body(&mut self) -> Option<Box<Node<'a>>> {
        todo!("parser.try_p_top_expr_body")
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
    fn parse_opt_ensure(&mut self) -> Option<(Token, Box<Node<'a>>)> {
        todo!("parser.parse_opt_ensure")
    }
    fn try_literal(&mut self) -> Option<Box<Node<'a>>> {
        None.or_else(|| self.try_numeric())
            .or_else(|| self.try_symbol())
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
        None.or_else(|| self.try_user_variable())
            .or_else(|| self.try_keyword_variable())
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
    fn try_operation(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token(TokenKind::tIDENTIFIER))
            .or_else(|| self.try_token(TokenKind::tCONSTANT))
            .or_else(|| self.try_token(TokenKind::tFID))
    }
    fn try_operation2(&mut self) -> Option<Token> {
        None.or_else(|| self.try_operation())
            .or_else(|| self.try_op())
    }
    fn try_operation3(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token(TokenKind::tIDENTIFIER))
            .or_else(|| self.try_token(TokenKind::tFID))
            .or_else(|| self.try_op())
    }
    fn try_dot_or_colon(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token(TokenKind::tDOT))
            .or_else(|| self.try_token(TokenKind::tCOLON2))
    }
    fn try_call_op(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token(TokenKind::tDOT))
            .or_else(|| self.try_token(TokenKind::tANDDOT))
    }
    fn try_call_op2(&mut self) -> Option<Token> {
        None.or_else(|| self.try_call_op())
            .or_else(|| self.try_token(TokenKind::tCOLON2))
    }
    fn parse_opt_terms(&mut self) {
        self.parse_terms();
    }
    fn try_opt_nl(&mut self) -> Option<Token> {
        self.try_token(TokenKind::tNL)
    }
    fn try_rparen(&mut self) -> Option<Token> {
        let checkpoint = self.new_checkpoint();
        self.try_opt_nl();
        if let Some(rparen_t) = self.try_token(TokenKind::tRPAREN) {
            Some(rparen_t)
        } else {
            self.restore_checkpoint(checkpoint);
            None
        }
    }
    fn try_rbracket(&mut self) -> Option<Token> {
        let checkpoint = self.new_checkpoint();
        self.try_opt_nl();
        if let Some(rbrack_t) = self.try_token(TokenKind::tRBRACK) {
            Some(rbrack_t)
        } else {
            self.restore_checkpoint(checkpoint);
            None
        }
    }
    fn try_rbrace(&mut self) -> Option<Token> {
        let checkpoint = self.new_checkpoint();
        self.try_opt_nl();
        if let Some(rbrace_t) = self.try_token(TokenKind::tRCURLY) {
            Some(rbrace_t)
        } else {
            self.restore_checkpoint(checkpoint);
            None
        }
    }
    fn try_trailer(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token(TokenKind::tNL))
            .or_else(|| self.try_token(TokenKind::tCOMMA))
    }
    fn try_term(&mut self) -> Option<Token> {
        None.or_else(|| self.try_token(TokenKind::tSEMI))
            .or_else(|| self.try_token(TokenKind::tNL))
    }
    fn parse_terms(&mut self) -> Vec<Token> {
        let mut terms = vec![];
        if let Some(term) = self.try_term() {
            terms.push(term)
        } else {
            return vec![];
        }
        loop {
            if self.current_token().is(TokenKind::tSEMI) {
                self.take_token();
            } else {
                break;
            }

            if let Some(term) = self.try_term() {
                terms.push(term)
            } else {
                break;
            }
        }
        terms
    }

    fn try_colon2_const(&mut self) -> Option<(Token, Token)> {
        let colon2_t = self.try_token(TokenKind::tCOLON2)?;
        let const_t = self.expect_token(TokenKind::tCONSTANT);
        Some((colon2_t, const_t))
    }
}
