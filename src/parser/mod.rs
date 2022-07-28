use crate::buffer::Buffer;
use crate::builder::{Builder, Constructor, RustConstructor};
use crate::lexer::Lexer;
use crate::nodes::Node;
use crate::state::OwnedState;
use crate::token::{Token, TokenKind};
use crate::transactions::{ParseError, ParseResult};

mod checkpoint;

mod alias;
mod arg;
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

pub struct Parser<C: Constructor = RustConstructor> {
    state: OwnedState,

    lexer: Lexer,
    debug: bool,
    phantom: std::marker::PhantomData<C>,
}
pub type RustParser = Parser<RustConstructor>;

impl<C> Parser<C>
where
    C: Constructor,
{
    pub fn new(input: &[u8]) -> Self {
        let mut state = OwnedState::new(input);
        let state_ref = state.new_ref();

        Self {
            state,
            lexer: Lexer::new(state_ref),
            debug: false,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self.lexer = self.lexer.debug();
        self
    }

    pub(crate) fn current_token(&mut self) -> Token {
        self.lexer.current_token()
    }
    pub(crate) fn skip_token(&mut self) {
        self.lexer.skip_token()
    }

    pub(crate) fn expect_token(&mut self, expected: TokenKind) -> ParseResult<Token> {
        let token = self.current_token();
        self.skip_token();

        if token.is(expected) {
            Ok(token)
        } else {
            Err(ParseError::TokenError {
                lookahead: true,
                expected,
                got: token.kind,
                loc: token.loc,
            })
        }
    }

    pub(crate) fn try_token(&mut self, expected: TokenKind) -> ParseResult<Token> {
        if self.current_token().is(expected) {
            let token = self.current_token();
            self.skip_token();
            Ok(token)
        } else {
            Err(ParseError::TokenError {
                lookahead: true,
                expected,
                got: self.current_token().kind,
                loc: self.current_token().loc,
            })
        }
    }

    pub fn parse(&mut self) -> Option<Box<Node>> {
        self.try_program().unwrap()
    }

    fn try_program(&mut self) -> ParseResult<Option<Box<Node>>> {
        self.try_top_compstmt()
    }

    pub(crate) fn buffer(&self) -> &Buffer {
        self.lexer.buffer().for_lookahead()
    }
}

impl<C> Parser<C>
where
    C: Constructor,
{
    fn try_command_rhs(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_command_rhs")
    }

    fn try_def_name(&mut self) -> ParseResult<Token> {
        self.try_fname()
    }

    fn try_expr_value(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_expr_value")
    }

    fn try_expr_value_do(&mut self) -> ParseResult<(Box<Node>, Token)> {
        todo!("parser.try_expr_value_do")
    }

    fn try_command_call(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("command call")
            .or_else(|| self.try_command())
            .or_else(|| self.try_block_command())
            .stop()
    }

    fn try_block_command(&mut self) -> ParseResult<Box<Node>> {
        let (block_call, maybe_args) = self
            .all_of("block command")
            .and(|| self.try_block_call())
            .and(|| {
                self.one_of("block call arguments")
                    .or_else(|| {
                        let a = self
                            .all_of("required block call arguments")
                            .and(|| self.try_call_op2())
                            .and(|| self.try_operation2())
                            .and(|| self.try_command_args())
                            .stop()
                            .map(|values| Some(values))?;

                        Ok(a)
                    })
                    .or_else(|| Ok(None))
                    .stop()
            })
            .stop()?;

        panic!("{:?} {:?}", block_call, maybe_args)
    }

    fn try_cmd_brace_block(&mut self) {
        todo!("parser.try_cmd_brace_block")
    }

    fn try_fcall(&mut self) -> ParseResult<Token> {
        self.try_operation()
    }

    fn try_cname(&mut self) {
        todo!("parser.try_cname")
    }
    fn try_fname(&mut self) -> ParseResult<Token> {
        self.one_of("fname")
            .or_else(|| self.try_token(TokenKind::tIDENTIFIER))
            .or_else(|| self.try_token(TokenKind::tCONSTANT))
            .or_else(|| self.try_token(TokenKind::tFID))
            .or_else(|| self.try_op())
            .or_else(|| self.try_reswords())
            .stop()
    }
    fn try_fitem(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("fitem")
            .or_else(|| {
                self.try_fname()
                    .map(|token| Builder::<C>::symbol_internal(token, self.buffer()))
            })
            .or_else(|| self.try_symbol())
            .stop()
    }

    fn try_op(&mut self) -> ParseResult<Token> {
        self.one_of("operation")
            .or_else(|| self.try_token(TokenKind::tPIPE))
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
            .stop()
    }
    fn try_reswords(&mut self) -> ParseResult<Token> {
        self.one_of("reserved word")
            .or_else(|| self.try_token(TokenKind::k__LINE__))
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
            .stop()
    }
    fn try_relop(&mut self) {
        todo!("parser.try_relop")
    }
    fn try_rel_expr(&mut self) {
        todo!("parser.try_rel_expr")
    }
    fn try_paren_args(&mut self) -> ParseResult<Vec<Node>> {
        todo!("parser.try_paren_args")
    }
    fn try_opt_paren_args(&mut self) -> ParseResult<Vec<Node>> {
        todo!("parser.try_opt_paren_args")
    }
    fn try_block_arg(&mut self) {
        todo!("parser.try_block_arg")
    }
    fn try_opt_block_arg(&mut self) {
        todo!("parser.try_opt_block_arg")
    }
    fn try_args(&mut self) -> ParseResult<Vec<Node>> {
        let mut args = vec![];
        let mut commas = vec![];

        fn item<C: Constructor>(parser: &mut Parser<C>) -> ParseResult<Box<Node>> {
            if parser.current_token().is(TokenKind::tSTAR) {
                let star_t = parser.current_token();
                let arg_value = parser.try_arg_value().map_err(|mut err| {
                    err.make_required();
                    err
                })?;
                todo!("{:?} {:?}", star_t, arg_value)
            } else {
                parser.try_arg_value()
            }
        }

        let arg = item(self)?;
        args.push(*arg);
        loop {
            match self.expect_token(TokenKind::tCOMMA) {
                Ok(comma) => commas.push(comma),
                Err(_) => return Ok(args),
            }
            match item(self) {
                Ok(item) => args.push(*item),
                Err(error) => {
                    if error.is_lookahead() {
                        return Err(ParseError::seq_error::<Vec<Node>, _>(
                            "args",
                            (args, commas),
                            error,
                        ));
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(args)
    }
    fn try_mrhs_arg(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_mrhs_arg")
    }
    fn try_mrhs(&mut self) -> ParseResult<Vec<Node>> {
        todo!("parser.try_mrhs")
    }
    fn try_k_begin(&mut self) -> ParseResult<Token> {
        todo!("parser.try_k_begin")
    }
    fn try_k_do(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kDO)
    }
    fn try_k_do_block(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kDO)
    }
    fn try_k_rescue(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kRESCUE)
    }
    fn try_k_ensure(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kENSURE)
    }
    fn try_k_when(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kWHEN)
    }
    fn try_k_else(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kELSE)
    }
    fn try_k_elsif(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kELSIF)
    }
    fn try_k_end(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kEND)
    }
    fn try_k_return(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kRETURN)
    }
    fn try_do(&mut self) -> ParseResult<Token> {
        self.one_of("do")
            .or_else(|| self.try_term())
            .or_else(|| self.try_token(TokenKind::kDO))
            .stop()
    }
    fn try_f_marg(&mut self) {
        todo!("parser.try_f_marg")
    }
    fn try_f_marg_list(&mut self) {
        todo!("parser.try_f_marg_list")
    }
    fn try_f_margs(&mut self) {
        todo!("parser.try_f_margs")
    }
    fn try_f_rest_marg(&mut self) {
        todo!("parser.try_f_rest_marg")
    }
    fn try_f_any_kwrest(&mut self) {
        todo!("parser.try_f_any_kwrest")
    }
    fn try_f_eq(&mut self) {
        todo!("parser.try_f_eq")
    }
    fn try_block_args_tail(&mut self) {
        todo!("parser.try_block_args_tail")
    }
    fn try_opt_block_args_tail(&mut self) {
        todo!("parser.try_opt_block_args_tail")
    }
    fn try_excessed_comma(&mut self) {
        todo!("parser.try_excessed_comma")
    }
    fn try_block_param(&mut self) {
        todo!("parser.try_block_param")
    }
    fn try_opt_block_param(&mut self) {
        todo!("parser.try_opt_block_param")
    }
    fn try_block_param_def(&mut self) {
        todo!("parser.try_block_param_def")
    }
    fn try_opt_bv_decl(&mut self) {
        todo!("parser.try_opt_bv_decl")
    }
    fn try_bv_decls(&mut self) {
        todo!("parser.try_bv_decls")
    }
    fn try_bvar(&mut self) {
        todo!("parser.try_bvar")
    }
    fn try_lambda(&mut self) -> ParseResult<Box<Node>> {
        let (lambda_t, arglist, body) = self
            .all_of("lambda")
            .and(|| self.try_token(TokenKind::tLAMBDA))
            .and(|| self.try_f_larglist())
            .and(|| self.try_lambda_body())
            .stop()?;

        todo!("builder.lambda {:?} {:?} {:?}", lambda_t, arglist, body)
    }
    fn try_f_larglist(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_f_larglist")
    }
    fn try_lambda_body(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_lambda_body")
    }
    fn try_do_block(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_do_block")
    }
    fn try_block_call(&mut self) -> ParseResult<Box<Node>> {
        let (head, tail) = self
            .all_of("block_call")
            .and(|| self.try_block_call_head())
            .and(|| {
                self.one_of("block call tail")
                    .or_else(|| self.try_block_call_tail().map(|value| Some(value)))
                    .or_else(|| Ok(None))
                    .stop()
            })
            .stop()?;

        todo!("{:?} {:?}", head, tail)
    }
    fn try_block_call_head(&mut self) -> ParseResult<Box<Node>> {
        let (command, do_block) = self
            .all_of("command do_block")
            .and(|| self.try_command())
            .and(|| self.try_do_block())
            .stop()?;

        todo!("{:?} {:?}", command, do_block)
    }
    fn try_block_call_tail(&mut self) -> ParseResult<Box<Node>> {
        // | call_op2 operation2 opt_paren_args
        // | call_op2 operation2 opt_paren_args brace_block
        // | call_op2 operation2 command_args do_block
        todo!("try_block_call_tail")
    }
    fn try_method_call(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("method call")
            .or_else(|| {
                let (fcall, paren_args) = self
                    .all_of("fcall (args)")
                    .and(|| self.try_fcall())
                    .and(|| self.try_paren_args())
                    .stop()?;
                todo!("{:?} {:?}", fcall, paren_args)
            })
            .or_else(|| {
                let (primary_value, lbrack_t, opt_call_args, rbrack_t) = self
                    .all_of("primary [opt call args]")
                    .and(|| self.try_primary_value())
                    .and(|| self.expect_token(TokenKind::tLBRACK))
                    .and(|| self.try_opt_call_args())
                    .and(|| self.try_rbracket())
                    .stop()?;
                todo!(
                    "{:?} {:?} {:?} {:?}",
                    primary_value,
                    lbrack_t,
                    opt_call_args,
                    rbrack_t
                )
            })
            .or_else(|| {
                let (primary_value, call_t, paren_args) = self
                    .all_of("primary call_op2 paren_args")
                    .and(|| self.try_primary_value())
                    .and(|| self.try_call_op2())
                    .and(|| self.try_paren_args())
                    .stop()?;
                todo!("{:?} {:?} {:?}", primary_value, call_t, paren_args)
            })
            .or_else(|| {
                let (primary_value, call_t, op_t, opt_paren_args) = self
                    .all_of("primary call_op2 operation2 opt_paren_args")
                    .and(|| self.try_primary_value())
                    .and(|| self.try_call_op2())
                    .and(|| self.try_operation2())
                    .and(|| self.try_opt_paren_args())
                    .stop()?;
                todo!(
                    "{:?} {:?} {:?} {:?}",
                    primary_value,
                    call_t,
                    op_t,
                    opt_paren_args
                )
            })
            .or_else(|| {
                let (super_t, paren_args) = self
                    .all_of("super(args)")
                    .and(|| self.try_token(TokenKind::kSUPER))
                    .and(|| self.try_paren_args())
                    .stop()?;
                todo!("{:?} {:?}", super_t, paren_args)
            })
            .or_else(|| {
                let super_t = self.try_token(TokenKind::kSUPER)?;
                todo!("{:?}", super_t)
            })
            .stop()
    }

    // TODO: return ArgsType instead of ()
    fn try_brace_block(&mut self) -> ParseResult<(Token, Option<Box<Node>>, Token)> {
        todo!("parser.try_brace_block")
    }
    fn try_do_body(&mut self) {
        todo!("parser.try_do_body")
    }
    fn try_p_case_body(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_p_case_body")
    }
    fn try_p_cases(&mut self) {
        todo!("parser.try_p_cases")
    }
    fn try_p_top_expr(&mut self) {
        todo!("parser.try_p_top_expr")
    }
    fn try_p_top_expr_body(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_p_top_expr_body")
    }
    fn try_p_expr(&mut self) {
        todo!("parser.try_p_expr")
    }
    fn try_p_as(&mut self) {
        todo!("parser.try_p_as")
    }
    fn try_p_alt(&mut self) {
        todo!("parser.try_p_alt")
    }
    fn try_p_lparen(&mut self) {
        todo!("parser.try_p_lparen")
    }
    fn try_p_lbracket(&mut self) {
        todo!("parser.try_p_lbracket")
    }
    fn try_p_expr_basic(&mut self) {
        todo!("parser.try_p_expr_basic")
    }
    fn try_p_args(&mut self) {
        todo!("parser.try_p_args")
    }
    fn try_p_args_head(&mut self) {
        todo!("parser.try_p_args_head")
    }
    fn try_p_args_tail(&mut self) {
        todo!("parser.try_p_args_tail")
    }
    fn try_p_find(&mut self) {
        todo!("parser.try_p_find")
    }
    fn try_p_rest(&mut self) {
        todo!("parser.try_p_rest")
    }
    fn try_p_args_post(&mut self) {
        todo!("parser.try_p_args_post")
    }
    fn try_p_arg(&mut self) {
        todo!("parser.try_p_arg")
    }
    fn try_p_kwargs(&mut self) {
        todo!("parser.try_p_kwargs")
    }
    fn try_p_kwarg(&mut self) {
        todo!("parser.try_p_kwarg")
    }
    fn try_p_kw(&mut self) {
        todo!("parser.try_p_kw")
    }
    fn try_p_kw_label(&mut self) {
        todo!("parser.try_p_kw_label")
    }
    fn try_p_kwrest(&mut self) {
        todo!("parser.try_p_kwrest")
    }
    fn try_p_kwnorest(&mut self) {
        todo!("parser.try_p_kwnorest")
    }
    fn try_p_any_kwrest(&mut self) {
        todo!("parser.try_p_any_kwrest")
    }
    fn try_p_value(&mut self) {
        todo!("parser.try_p_value")
    }
    fn try_p_primitive(&mut self) {
        todo!("parser.try_p_primitive")
    }
    fn try_p_variable(&mut self) {
        todo!("parser.try_p_variable")
    }
    fn try_p_var_ref(&mut self) {
        todo!("parser.try_p_var_ref")
    }
    fn try_p_expr_ref(&mut self) {
        todo!("parser.try_p_expr_ref")
    }
    fn try_p_const(&mut self) {
        todo!("parser.try_p_const")
    }
    fn try_literal(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("literal")
            .or_else(|| self.try_numeric())
            .or_else(|| self.try_symbol())
            .stop()
    }
    fn try_nonlocal_var(&mut self) {
        todo!("parser.try_nonlocal_var")
    }
    fn try_user_variable(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("user variable")
            .or_else(|| self.try_lvar())
            .or_else(|| self.try_ivar())
            .or_else(|| self.try_gvar())
            .or_else(|| self.try_t_const())
            .or_else(|| self.try_cvar())
            .stop()
    }
    fn try_var_ref(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("variable reference")
            .or_else(|| self.try_user_variable())
            .or_else(|| self.try_keyword_variable())
            .stop()
    }
    fn try_var_lhs(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("variable as LHS in assignment")
            .or_else(|| self.try_user_variable())
            .or_else(|| self.try_keyword_variable())
            .stop()
            .map(|node| Builder::<C>::assignable(node))
    }
    fn try_f_opt_paren_args(&mut self) -> ParseResult<Vec<Node>> {
        todo!("parser.try_f_opt_paren_args")
    }
    fn try_f_paren_args(&mut self) {
        todo!("parser.try_f_paren_args")
    }
    fn try_args_tail(&mut self) {
        todo!("parser.try_args_tail")
    }
    fn try_opt_args_tail(&mut self) {
        todo!("parser.try_opt_args_tail")
    }
    fn try_f_args(&mut self) {
        todo!("parser.try_f_args")
    }
    fn try_args_forward(&mut self) {
        todo!("parser.try_args_forward")
    }
    fn try_f_bad_arg(&mut self) {
        todo!("parser.try_f_bad_arg")
    }
    fn try_f_norm_arg(&mut self) {
        todo!("parser.try_f_norm_arg")
    }
    fn try_f_arg_asgn(&mut self) {
        todo!("parser.try_f_arg_asgn")
    }
    fn try_f_arg_item(&mut self) {
        todo!("parser.try_f_arg_item")
    }
    fn try_f_arg(&mut self) {
        todo!("parser.try_f_arg")
    }
    fn try_f_label(&mut self) {
        todo!("parser.try_f_label")
    }
    fn try_f_kw(&mut self) {
        todo!("parser.try_f_kw")
    }
    fn try_f_block_kw(&mut self) {
        todo!("parser.try_f_block_kw")
    }
    fn try_f_block_kwarg(&mut self) {
        todo!("parser.try_f_block_kwarg")
    }
    fn try_f_kwarg(&mut self) {
        todo!("parser.try_f_kwarg")
    }
    fn try_kwrest_mark(&mut self) {
        todo!("parser.try_kwrest_mark")
    }
    fn try_f_no_kwarg(&mut self) {
        todo!("parser.try_f_no_kwarg")
    }
    fn try_f_kwrest(&mut self) {
        todo!("parser.try_f_kwrest")
    }
    fn try_f_opt(&mut self) {
        todo!("parser.try_f_opt")
    }
    fn try_f_block_opt(&mut self) {
        todo!("parser.try_f_block_opt")
    }
    fn try_f_block_optarg(&mut self) {
        todo!("parser.try_f_block_optarg")
    }
    fn try_f_optarg(&mut self) {
        todo!("parser.try_f_optarg")
    }
    fn try_restarg_mark(&mut self) {
        todo!("parser.try_restarg_mark")
    }
    fn try_f_rest_arg(&mut self) {
        todo!("parser.try_f_rest_arg")
    }
    fn try_blkarg_mark(&mut self) {
        todo!("parser.try_blkarg_mark")
    }
    fn try_f_block_arg(&mut self) {
        todo!("parser.try_f_block_arg")
    }
    fn try_opt_f_block_arg(&mut self) {
        todo!("parser.try_opt_f_block_arg")
    }
    fn try_assoc_list(&mut self) -> ParseResult<Vec<Node>> {
        todo!("parser.try_assoc_list")
    }
    fn try_assocs(&mut self) -> ParseResult<Vec<Node>> {
        todo!("parser.try_assocs")
    }
    fn try_assoc(&mut self) {
        todo!("parser.try_assoc")
    }
    fn try_operation(&mut self) -> ParseResult<Token> {
        self.one_of("operation")
            .or_else(|| self.try_token(TokenKind::tIDENTIFIER))
            .or_else(|| self.try_token(TokenKind::tCONSTANT))
            .or_else(|| self.try_token(TokenKind::tFID))
            .stop()
    }
    fn try_operation2(&mut self) -> ParseResult<Token> {
        self.one_of("operation 2")
            .or_else(|| self.try_operation())
            .or_else(|| self.try_op())
            .stop()
    }
    fn try_operation3(&mut self) -> ParseResult<Token> {
        self.one_of("operation 3")
            .or_else(|| self.try_token(TokenKind::tIDENTIFIER))
            .or_else(|| self.try_token(TokenKind::tFID))
            .or_else(|| self.try_op())
            .stop()
    }
    fn try_dot_or_colon(&mut self) -> ParseResult<Token> {
        self.one_of("dot or colon")
            .or_else(|| self.try_token(TokenKind::tDOT))
            .or_else(|| self.try_token(TokenKind::tCOLON2))
            .stop()
    }
    fn try_call_op(&mut self) -> ParseResult<Token> {
        self.one_of("call operation")
            .or_else(|| self.try_token(TokenKind::tDOT))
            .or_else(|| self.try_token(TokenKind::tANDDOT))
            .stop()
    }
    fn try_call_op2(&mut self) -> ParseResult<Token> {
        self.one_of("call operation 2")
            .or_else(|| self.try_call_op())
            .or_else(|| self.try_token(TokenKind::tCOLON2))
            .stop()
    }
    fn try_opt_terms(&mut self) -> ParseResult<Vec<Token>> {
        self.try_terms()
    }

    fn try_opt_nl(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::tNL)
    }

    fn try_rparen(&mut self) -> ParseResult<Token> {
        let (_, rparen_t) = self
            .all_of("rparen")
            .and(|| self.try_opt_nl())
            .and(|| self.expect_token(TokenKind::tRPAREN))
            .stop()?;

        Ok(rparen_t)
    }
    fn try_rbracket(&mut self) -> ParseResult<Token> {
        let (_, rbrack_t) = self
            .all_of("rbrack")
            .and(|| self.try_opt_nl())
            .and(|| self.expect_token(TokenKind::tRBRACK))
            .stop()?;

        Ok(rbrack_t)
    }
    fn try_rbrace(&mut self) -> ParseResult<Token> {
        let (_, rbrace_t) = self
            .all_of("rbrace")
            .and(|| self.try_opt_nl())
            .and(|| self.expect_token(TokenKind::tRCURLY))
            .stop()?;

        Ok(rbrace_t)
    }
    fn try_trailer(&mut self) -> ParseResult<Option<Token>> {
        self.one_of("trailer")
            .or_else(|| self.try_token(TokenKind::tNL).map(|token| Some(token)))
            .or_else(|| self.try_token(TokenKind::tCOMMA).map(|token| Some(token)))
            .or_else(|| Ok(None))
            .stop()
    }
    fn try_term(&mut self) -> ParseResult<Token> {
        self.one_of("term")
            .or_else(|| self.try_token(TokenKind::tSEMI))
            .or_else(|| self.try_token(TokenKind::tNL))
            .stop()
    }
    fn try_terms(&mut self) -> ParseResult<Vec<Token>> {
        let mut terms = vec![];
        if let Ok(term) = self.try_term() {
            terms.push(term)
        } else {
            return Ok(vec![]);
        }
        loop {
            if let Err(_) = self.try_token(TokenKind::tSEMI) {
                break;
            }

            if let Ok(term) = self.try_term() {
                terms.push(term)
            } else {
                break;
            }
        }
        Ok(terms)
    }

    fn try_colon2_const(&mut self) -> ParseResult<(Token, Token)> {
        self.all_of("::CONST")
            .and(|| self.try_token(TokenKind::tCOLON2))
            .and(|| self.try_token(TokenKind::tCONSTANT))
            .stop()
    }
}
