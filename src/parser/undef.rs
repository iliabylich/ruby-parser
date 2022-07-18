use crate::{
    builder::{Builder, Constructor},
    parser::{ParseError, Parser},
    token::TokenKind,
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_undef(&mut self) -> Result<Box<Node>, ParseError> {
        let undef_t = self.try_token(TokenKind::kUNDEF)?;
        let names = self.parse_names()?;
        Ok(Builder::<C>::undef(undef_t, names))
    }

    fn parse_names(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut names = vec![];

        let fitem = self.try_fitem()?;
        names.push(*fitem);

        loop {
            if self.current_token().is(TokenKind::tCOMMA) {
                // consume
                self.skip_token();
            } else {
                break;
            }
            let fitem = self.try_fitem()?;
            names.push(*fitem);
        }

        Ok(names)
    }
}

#[test]
fn test_undef() {
    use crate::{
        loc::loc,
        nodes::{Sym, Undef},
        string_content::StringContent,
        Node, RustParser,
    };
    let mut parser = RustParser::new(b"undef a, :b, c");
    assert_eq!(
        parser.try_undef(),
        Ok(Box::new(Node::Undef(Undef {
            names: vec![
                Node::Sym(Sym {
                    name: StringContent::from("a"),
                    begin_l: None,
                    end_l: None,
                    expression_l: loc!(6, 7)
                }),
                Node::Sym(Sym {
                    name: StringContent::from("b"),
                    begin_l: Some(loc!(9, 10)),
                    end_l: None,
                    expression_l: loc!(9, 11)
                }),
                Node::Sym(Sym {
                    name: StringContent::from("c"),
                    begin_l: None,
                    end_l: None,
                    expression_l: loc!(13, 14)
                })
            ],
            keyword_l: loc!(0, 5),
            expression_l: loc!(0, 14)
        })))
    );
}
