use crate::{
    parser::base::{Captured, ParseError, ParseResult, Rule},
    Parser,
};

pub(crate) struct SeparatedBy<Item, Sep>
where
    Item: Rule,
    Sep: Rule,
    Captured: From<Item::Output>,
    Captured: From<Sep::Output>,
{
    _item: std::marker::PhantomData<Item>,
    _sep: std::marker::PhantomData<Sep>,
}

impl<Item, Sep> Rule for SeparatedBy<Item, Sep>
where
    Item: Rule,
    Sep: Rule,
    Captured: From<Item::Output>,
    Captured: From<Sep::Output>,
{
    type Output = (Vec<Item::Output>, Vec<Sep::Output>);

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let mut items = vec![];
        let mut seps = vec![];

        enum ReadResult<T> {
            Ok(T),
            Err(ParseError),
            None,
        }

        let read_item = |parser: &mut Parser| {
            if !Item::starts_now(parser) {
                return ReadResult::None;
            }

            match Item::parse(parser) {
                Ok(item) => ReadResult::Ok(item),
                Err(err) => ReadResult::Err(err),
            }
        };

        let read_sep = |parser: &mut Parser| {
            if !Sep::starts_now(parser) {
                return ReadResult::None;
            }

            match Sep::parse(parser) {
                Ok(sep) => ReadResult::Ok(sep),
                Err(err) => ReadResult::Err(err),
            }
        };

        let append_all_captures =
            |error: &mut ParseError, items: &mut Vec<Item::Output>, seps: &mut Vec<Sep::Output>| {
                error.captured = Captured::from(std::mem::take(items))
                    + Captured::from(std::mem::take(seps))
                    + std::mem::take(&mut error.captured);
            };

        match read_item(parser) {
            ReadResult::Ok(item) => items.push(item),
            ReadResult::Err(mut err) => {
                append_all_captures(&mut err, &mut items, &mut seps);
                return Err(err);
            }
            ReadResult::None => return Ok((items, seps)),
        }

        match read_sep(parser) {
            ReadResult::Ok(sep) => seps.push(sep),
            ReadResult::Err(mut err) => {
                append_all_captures(&mut err, &mut items, &mut seps);
                return Err(err);
            }
            ReadResult::None => return Ok((items, seps)),
        }

        loop {
            match read_item(parser) {
                ReadResult::Ok(item) => items.push(item),
                ReadResult::Err(mut err) => {
                    append_all_captures(&mut err, &mut items, &mut seps);
                    return Err(err);
                }
                ReadResult::None => break,
            }

            match read_sep(parser) {
                ReadResult::Ok(sep) => seps.push(sep),
                ReadResult::Err(mut err) => {
                    append_all_captures(&mut err, &mut items, &mut seps);
                    return Err(err);
                }
                ReadResult::None => break,
            }
        }

        Ok((items, seps))
    }
}
