use crate::{builder::Constructor, parser::Parser, token::Token, Node};

impl<'a, C> Parser<'a, C> where C: Constructor {}
