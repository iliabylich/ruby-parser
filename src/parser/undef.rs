use crate::{
    builder::{Builder, Constructor},
    parser::Parser,
    token::TokenValue,
    Node,
};

impl<'a, C> Parser<'a, C>
where
    C: Constructor,
{
    pub(crate) fn try_undef(&mut self) -> Option<Box<Node<'a>>> {
        let undef_t = self.try_token(TokenValue::kUNDEF)?;
        let names = self.parse_names();
        Some(Builder::<C>::undef(undef_t, names))
    }

    fn parse_names(&mut self) -> Vec<Node<'a>> {
        let mut names = vec![];
        if let Some(fitem) = self.try_fitem() {
            names.push(*fitem);
        }
        loop {
            if self.current_token().is(TokenValue::tCOMMA) {
                // consume
                self.skip_token();
            } else {
                break;
            }
            match self.try_fitem() {
                Some(fitem) => names.push(*fitem),
                None => panic!("expected fitem, got {:?}", self.current_token()),
            }
        }
        names
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
        Some(Box::new(Node::Undef(Undef {
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
