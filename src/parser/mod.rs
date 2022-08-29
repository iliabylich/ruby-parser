use crate::buffer::Buffer;
use crate::builder::Builder;
use crate::lexer::Lexer;
use crate::nodes::Node;
use crate::parser::macros::{all_of, maybe, one_of};
use crate::state::OwnedState;
use crate::token::{Token, TokenKind};
use crate::transactions::{ParseError, ParseResult};

use self::macros::separated_by;

mod checkpoint;
mod macros;

mod alias;
mod arg;
mod args;
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
mod method_definition;
mod mlhs;
mod module;
mod numeric;
mod opt_else;
mod opt_ensure;
mod opt_rescue;
mod pattern_matching;
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

pub struct Parser {
    state: OwnedState,

    lexer: Lexer,
    debug: bool,
}

static mut COUNTER: usize = 0;

impl Parser {
    pub fn new(input: &[u8]) -> Self {
        let mut state = OwnedState::new(input);
        let state_ref = state.new_ref();

        Self {
            state,
            lexer: Lexer::new(state_ref),
            debug: false,
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
                lookahead: false,
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
        self.parse_program().unwrap()
    }

    fn parse_program(&mut self) -> ParseResult<Option<Box<Node>>> {
        self.try_top_compstmt()
    }

    pub(crate) fn buffer(&self) -> &Buffer {
        self.lexer.buffer().for_lookahead()
    }
}

impl Parser {
    fn parse_command_rhs(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.parse_command_rhs")
    }

    fn parse_def_name(&mut self) -> ParseResult<Token> {
        self.parse_fname()
    }

    fn parse_expr_value(&mut self) -> ParseResult<Box<Node>> {
        let expr = self.parse_expr()?;
        // self.value_expr(&expr);
        Ok(expr)
    }

    fn parse_expr_value_do(&mut self) -> ParseResult<(Box<Node>, Token)> {
        all_of!("expr_value do", self.parse_expr(), self.parse_do(),)
    }

    fn parse_command_call(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "command call",
            checkpoint = self.new_checkpoint(),
            self.parse_command(),
            self.parse_block_command(),
        )
    }

    fn parse_block_command(&mut self) -> ParseResult<Box<Node>> {
        let (block_call, maybe_args) = all_of!(
            "block command",
            self.parse_block_call(),
            one_of!(
                "block call arguments",
                checkpoint = self.new_checkpoint(),
                {
                    all_of!(
                        "required block call arguments",
                        self.parse_call_op2(),
                        self.parse_operation2(),
                        self.parse_command_args(),
                    )
                    .map(|values| Some(values))
                },
                Ok(None),
            ),
        )?;

        panic!("{:?} {:?}", block_call, maybe_args)
    }

    fn parse_cmd_brace_block(&mut self) {
        todo!("parser.parse_cmd_brace_block")
    }

    fn parse_fcall(&mut self) -> ParseResult<Token> {
        self.parse_operation()
    }

    fn parse_cname(&mut self) -> ParseResult<Token> {
        one_of!(
            "cname",
            checkpoint = self.new_checkpoint(),
            {
                let token = self.try_token(TokenKind::tIDENTIFIER)?;
                // TODO: report class or module name must be constant
                Ok(token)
            },
            self.try_token(TokenKind::tCONSTANT),
        )
    }
    fn parse_fname(&mut self) -> ParseResult<Token> {
        one_of!(
            "fname",
            self.try_token(TokenKind::tIDENTIFIER),
            self.try_token(TokenKind::tCONSTANT),
            self.try_token(TokenKind::tFID),
            self.parse_op(),
            self.parse_reswords(),
        )
    }
    fn parse_fitem(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "fitem",
            checkpoint = self.new_checkpoint(),
            {
                let token = self.parse_fname()?;
                Ok(Builder::symbol_internal(token, self.buffer()))
            },
            self.parse_symbol(),
        )
    }

    fn parse_op(&mut self) -> ParseResult<Token> {
        one_of!(
            "operation",
            self.try_token(TokenKind::tPIPE),
            self.try_token(TokenKind::tCARET),
            self.try_token(TokenKind::tAMPER),
            self.try_token(TokenKind::tCMP),
            self.try_token(TokenKind::tEQ),
            self.try_token(TokenKind::tEQQ),
            self.try_token(TokenKind::tMATCH),
            self.try_token(TokenKind::tNMATCH),
            self.try_token(TokenKind::tGT),
            self.try_token(TokenKind::tGEQ),
            self.try_token(TokenKind::tLT),
            self.try_token(TokenKind::tLEQ),
            self.try_token(TokenKind::tNEQ),
            self.try_token(TokenKind::tLSHFT),
            self.try_token(TokenKind::tRSHFT),
            self.try_token(TokenKind::tPLUS),
            self.try_token(TokenKind::tMINUS),
            self.try_token(TokenKind::tSTAR),
            self.try_token(TokenKind::tSTAR),
            self.try_token(TokenKind::tDIVIDE),
            self.try_token(TokenKind::tPERCENT),
            self.try_token(TokenKind::tDSTAR),
            self.try_token(TokenKind::tBANG),
            self.try_token(TokenKind::tTILDE),
            self.try_token(TokenKind::tUPLUS),
            self.try_token(TokenKind::tUMINUS),
            self.try_token(TokenKind::tAREF),
            self.try_token(TokenKind::tASET),
            self.try_token(TokenKind::tBACK_REF),
        )
    }
    fn parse_reswords(&mut self) -> ParseResult<Token> {
        one_of!(
            "reserved word",
            self.try_token(TokenKind::k__LINE__),
            self.try_token(TokenKind::k__FILE__),
            self.try_token(TokenKind::k__ENCODING__),
            self.try_token(TokenKind::klBEGIN),
            self.try_token(TokenKind::klEND),
            self.try_token(TokenKind::kALIAS),
            self.try_token(TokenKind::kAND),
            self.try_token(TokenKind::kBEGIN),
            self.try_token(TokenKind::kBREAK),
            self.try_token(TokenKind::kCASE),
            self.try_token(TokenKind::kCLASS),
            self.try_token(TokenKind::kDEF),
            self.try_token(TokenKind::kDEFINED),
            self.try_token(TokenKind::kDO),
            self.try_token(TokenKind::kELSE),
            self.try_token(TokenKind::kELSIF),
            self.try_token(TokenKind::kEND),
            self.try_token(TokenKind::kENSURE),
            self.try_token(TokenKind::kFALSE),
            self.try_token(TokenKind::kFOR),
            self.try_token(TokenKind::kIN),
            self.try_token(TokenKind::kMODULE),
            self.try_token(TokenKind::kNEXT),
            self.try_token(TokenKind::kNIL),
            self.try_token(TokenKind::kNOT),
            self.try_token(TokenKind::kOR),
            self.try_token(TokenKind::kREDO),
            self.try_token(TokenKind::kRESCUE),
            self.try_token(TokenKind::kRETRY),
            self.try_token(TokenKind::kRETURN),
            self.try_token(TokenKind::kSELF),
            self.try_token(TokenKind::kSUPER),
            self.try_token(TokenKind::kTHEN),
            self.try_token(TokenKind::kTRUE),
            self.try_token(TokenKind::kUNDEF),
            self.try_token(TokenKind::kWHEN),
            self.try_token(TokenKind::kYIELD),
            self.try_token(TokenKind::kIF),
            self.try_token(TokenKind::kUNLESS),
            self.try_token(TokenKind::kWHILE),
            self.try_token(TokenKind::kUNTIL),
        )
    }
    fn parse_relop(&mut self) {
        todo!("parser.parse_relop")
    }
    fn parse_rel_expr(&mut self) {
        todo!("parser.parse_rel_expr")
    }
    fn parse_block_arg(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "block_arg",
            checkpoint = self.new_checkpoint(),
            {
                let (amper_t, arg_value) = all_of!(
                    "tAMPER",
                    self.try_token(TokenKind::tAMPER),
                    self.parse_arg_value(),
                )?;
                todo!("{:?} {:?}", amper_t, arg_value)
            },
            {
                let amper_t = self.try_token(TokenKind::tAMPER)?;
                todo!("{:?}", amper_t)
            },
        )
    }
    fn parse_opt_block_arg(&mut self) -> ParseResult<Vec<Node>> {
        todo!("parser.parse_opt_block_arg")
    }
    fn parse_args(&mut self) -> ParseResult<Vec<Node>> {
        fn item(parser: &mut Parser) -> ParseResult<Box<Node>> {
            one_of!(
                "args item",
                {
                    let (star_t, arg_value) = all_of!(
                        "tSTAR arg_value",
                        parser.try_token(TokenKind::tSTAR),
                        parser.parse_arg_value(),
                    )?;
                    todo!("{:?} {:?}", star_t, arg_value)
                },
                parser.parse_arg_value(),
            )
        }

        let (args, _commas) = separated_by!(
            "args",
            checkpoint = self.new_checkpoint(),
            item = item(self),
            sep = self.try_token(TokenKind::tCOMMA)
        )?;
        Ok(args)
    }
    fn parse_mrhs_arg(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "mrhs",
            checkpoint = self.new_checkpoint(),
            {
                let args = self.parse_mrhs()?;
                Ok(Builder::array(None, args, None))
            },
            self.parse_arg_value(),
        )
    }
    fn parse_mrhs(&mut self) -> ParseResult<Vec<Node>> {
        self.parse_args()
    }
    fn parse_k_do(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kDO)
    }
    fn parse_k_do_block(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kDO)
    }
    fn parse_k_rescue(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kRESCUE)
    }
    fn parse_k_ensure(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kENSURE)
    }
    fn parse_k_else(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kELSE)
    }
    fn parse_k_elsif(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kELSIF)
    }
    fn parse_k_end(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kEND)
    }
    fn parse_k_return(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kRETURN)
    }
    fn parse_do(&mut self) -> ParseResult<Token> {
        one_of!("do", self.parse_term(), self.try_token(TokenKind::kDO),)
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
    fn try_opt_block_param(&mut self) -> ParseResult<Option<Box<Node>>> {
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
    fn parse_lambda(&mut self) -> ParseResult<Box<Node>> {
        let (lambda_t, arglist, body) = all_of!(
            "lambda",
            self.try_token(TokenKind::tLAMBDA),
            self.parse_f_larglist(),
            self.parse_lambda_body(),
        )?;

        todo!("builder.lambda {:?} {:?} {:?}", lambda_t, arglist, body)
    }
    fn parse_f_larglist(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.parse_f_larglist")
    }
    fn parse_lambda_body(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.parse_lambda_body")
    }
    fn parse_do_block(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.parse_do_block")
    }
    fn parse_block_call(&mut self) -> ParseResult<Box<Node>> {
        let (head, tail) = all_of!(
            "block_call",
            self.parse_block_call_head(),
            one_of!(
                "block call tail",
                checkpoint = self.new_checkpoint(),
                self.parse_block_call_tail().map(|value| Some(value)),
                Ok(None),
            ),
        )?;

        todo!("{:?} {:?}", head, tail)
    }
    fn parse_block_call_head(&mut self) -> ParseResult<Box<Node>> {
        let (command, do_block) = all_of!(
            "command do_block",
            self.parse_command(),
            self.parse_do_block(),
        )?;

        todo!("{:?} {:?}", command, do_block)
    }
    fn parse_block_call_tail(&mut self) -> ParseResult<Box<Node>> {
        // | call_op2 operation2 opt_paren_args
        // | call_op2 operation2 opt_paren_args brace_block
        // | call_op2 operation2 command_args do_block
        todo!("parse_block_call_tail")
    }
    fn parse_method_call(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "method call",
            checkpoint = self.new_checkpoint(),
            {
                let (fcall, paren_args) =
                    all_of!("fcall (args)", self.parse_fcall(), self.parse_paren_args(),)?;
                todo!("{:?} {:?}", fcall, paren_args)
            },
            {
                let (primary_value, lbrack_t, opt_call_args, rbrack_t) = all_of!(
                    "primary [opt call args]",
                    self.parse_primary_value(),
                    self.expect_token(TokenKind::tLBRACK),
                    self.parse_opt_call_args(),
                    self.parse_rbracket(),
                )?;
                todo!(
                    "{:?} {:?} {:?} {:?}",
                    primary_value,
                    lbrack_t,
                    opt_call_args,
                    rbrack_t
                )
            },
            {
                let (primary_value, call_t, paren_args) = all_of!(
                    "primary call_op2 paren_args",
                    self.parse_primary_value(),
                    self.parse_call_op2(),
                    self.parse_paren_args(),
                )?;
                todo!("{:?} {:?} {:?}", primary_value, call_t, paren_args)
            },
            {
                let (primary_value, call_t, op_t, opt_paren_args) = all_of!(
                    "primary call_op2 operation2 opt_paren_args",
                    self.parse_primary_value(),
                    self.parse_call_op2(),
                    self.parse_operation2(),
                    self.parse_opt_paren_args(),
                )?;
                todo!(
                    "{:?} {:?} {:?} {:?}",
                    primary_value,
                    call_t,
                    op_t,
                    opt_paren_args
                )
            },
            {
                let (super_t, paren_args) = all_of!(
                    "super(args)",
                    self.try_token(TokenKind::kSUPER),
                    self.parse_paren_args(),
                )?;
                todo!("{:?} {:?}", super_t, paren_args)
            },
            {
                let super_t = self.try_token(TokenKind::kSUPER)?;
                todo!("{:?}", super_t)
            },
        )
    }

    // TODO: return ArgsType instead of ()
    fn parse_brace_block(&mut self) -> ParseResult<(Token, Option<Box<Node>>, Token)> {
        todo!("parser.parse_brace_block")
    }
    fn parse_do_body(&mut self) {
        todo!("parser.parse_do_body")
    }

    fn parse_literal(&mut self) -> ParseResult<Box<Node>> {
        one_of!("literal", self.parse_numeric(), self.parse_symbol(),)
    }
    fn parse_nonlocal_var(&mut self) {
        todo!("parser.parse_nonlocal_var")
    }
    fn parse_user_variable(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "user variable",
            self.parse_lvar(),
            self.parse_ivar(),
            self.parse_gvar(),
            self.parse_t_const(),
            self.parse_cvar(),
        )
    }
    fn parse_var_ref(&mut self) -> ParseResult<Box<Node>> {
        one_of!(
            "variable reference",
            self.parse_user_variable(),
            self.parse_keyword_variable(),
        )
    }
    fn parse_var_lhs(&mut self) -> ParseResult<Box<Node>> {
        let lhs = one_of!(
            "variable as LHS in assignment",
            self.parse_user_variable(),
            self.parse_keyword_variable(),
        )?;
        Ok(Builder::assignable(lhs))
    }
    fn try_f_opt_paren_args(&mut self) -> ParseResult<Option<Box<Node>>> {
        todo!("parser.try_f_opt_paren_args")
    }
    fn parse_args_tail(&mut self) {
        todo!("parser.parse_args_tail")
    }
    fn parse_opt_args_tail(&mut self) {
        todo!("parser.parse_opt_args_tail")
    }
    fn parse_args_forward(&mut self) -> ParseResult<Token> {
        self.expect_token(TokenKind::tBDOT3)
    }
    fn parse_operation(&mut self) -> ParseResult<Token> {
        one_of!(
            "operation",
            self.try_token(TokenKind::tIDENTIFIER),
            self.try_token(TokenKind::tCONSTANT),
            self.try_token(TokenKind::tFID),
        )
    }
    fn parse_operation2(&mut self) -> ParseResult<Token> {
        one_of!("operation 2", self.parse_operation(), self.parse_op(),)
    }
    fn parse_operation3(&mut self) -> ParseResult<Token> {
        one_of!(
            "operation 3",
            self.try_token(TokenKind::tIDENTIFIER),
            self.try_token(TokenKind::tFID),
            self.parse_op(),
        )
    }
    fn parse_dot_or_colon(&mut self) -> ParseResult<Token> {
        one_of!(
            "dot or colon",
            self.try_token(TokenKind::tDOT),
            self.try_token(TokenKind::tCOLON2),
        )
    }
    fn parse_call_op(&mut self) -> ParseResult<Token> {
        one_of!(
            "call operation",
            self.try_token(TokenKind::tDOT),
            self.try_token(TokenKind::tANDDOT),
        )
    }
    fn parse_call_op2(&mut self) -> ParseResult<Token> {
        one_of!(
            "call operation 2",
            self.parse_call_op(),
            self.try_token(TokenKind::tCOLON2),
        )
    }
    fn parse_opt_terms(&mut self) -> ParseResult<Vec<Token>> {
        maybe!(self.parse_terms()).map(|maybe_terms| maybe_terms.unwrap_or_else(|| vec![]))
    }

    fn try_opt_nl(&mut self) -> ParseResult<Option<Token>> {
        maybe!(self.try_token(TokenKind::tNL))
    }

    fn parse_rparen(&mut self) -> ParseResult<Token> {
        let (_, rparen_t) = all_of!(
            "rparen",
            self.try_opt_nl(),
            self.expect_token(TokenKind::tRPAREN),
        )?;

        Ok(rparen_t)
    }
    fn parse_rbracket(&mut self) -> ParseResult<Token> {
        let (_, rbrack_t) = all_of!(
            "rbrack",
            self.try_opt_nl(),
            self.expect_token(TokenKind::tRBRACK),
        )?;

        Ok(rbrack_t)
    }
    fn parse_rbrace(&mut self) -> ParseResult<Token> {
        let (_, rbrace_t) = all_of!(
            "rbrace",
            self.try_opt_nl(),
            self.expect_token(TokenKind::tRCURLY),
        )?;

        Ok(rbrace_t)
    }
    fn try_trailer(&mut self) -> ParseResult<Option<Token>> {
        maybe!(one_of!(
            "trailer",
            self.try_token(TokenKind::tNL),
            self.try_token(TokenKind::tCOMMA),
        ))
    }
    fn parse_term(&mut self) -> ParseResult<Token> {
        one_of!(
            "term",
            self.try_token(TokenKind::tSEMI),
            self.try_token(TokenKind::tNL),
        )
    }
    fn parse_terms(&mut self) -> ParseResult<Vec<Token>> {
        let (terms, _semis) = separated_by!(
            "terms",
            checkpoint = self.new_checkpoint(),
            item = self.parse_term().map(Box::new),
            sep = self.try_token(TokenKind::tSEMI)
        )?;
        Ok(terms)
    }

    fn parse_colon2_const(&mut self) -> ParseResult<(Token, Token)> {
        all_of!(
            "::CONST",
            self.try_token(TokenKind::tCOLON2),
            self.try_token(TokenKind::tCONSTANT),
        )
    }
}
