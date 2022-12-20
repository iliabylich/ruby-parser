use crate::{
    parser::base::{Maybe1, ParseResult, Rule, Unbox},
    Parser,
};

pub(crate) struct SeparatedBy<Item, Sep>
where
    Item: Rule,
    Item::Output: Unbox,
    Sep: Rule,
    Sep::Output: Unbox,
{
    _item: std::marker::PhantomData<Item>,
    _sep: std::marker::PhantomData<Sep>,
}

impl<Item, Sep> Rule for SeparatedBy<Item, Sep>
where
    Item: Rule,
    Item::Output: Unbox,
    Sep: Rule,
    Sep::Output: Unbox,
{
    type Output = (
        Vec<<Item::Output as Unbox>::Output>,
        Vec<<Sep::Output as Unbox>::Output>,
    );

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let mut items = vec![];
        let mut seps = vec![];

        match Maybe1::<Item>::parse(parser).unwrap() {
            Some(item) => items.push(item.unbox()),
            None => return Ok((items, seps)),
        }

        loop {
            match Maybe1::<Sep>::parse(parser).unwrap() {
                Some(sep) => seps.push(sep.unbox()),
                None => break,
            }

            match Maybe1::<Item>::parse(parser).unwrap() {
                Some(item) => items.push(item.unbox()),
                None => break,
            }
        }

        Ok((items, seps))
    }
}
