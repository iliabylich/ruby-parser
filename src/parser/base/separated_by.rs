use crate::{
    parser::base::{Maybe1, Rule, Unbox},
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

    fn parse(parser: &mut Parser) -> Self::Output {
        let mut items = vec![];
        let mut seps = vec![];

        match Maybe1::<Item>::parse(parser) {
            Some(item) => items.push(item.unbox()),
            None => return (items, seps),
        }

        loop {
            match Maybe1::<Sep>::parse(parser) {
                Some(sep) => seps.push(sep.unbox()),
                None => break,
            }

            match Maybe1::<Item>::parse(parser) {
                Some(item) => items.push(item.unbox()),
                None => break,
            }
        }

        (items, seps)
    }
}
