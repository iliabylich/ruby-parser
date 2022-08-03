use crate::{
    buffer::Buffer,
    builder::{helpers::string_value, Builder},
    nodes::{RegOpt, Regexp},
    string_content::StringContent,
    token::Token,
    Node,
};

impl Builder {
    pub(crate) fn regexp_options(regexp_end_t: &Token, buffer: &Buffer) -> Option<Box<Node>> {
        let expression_l = regexp_end_t.loc;

        if expression_l.size() == 1 {
            // no regexp options, only trailing "/"
            return None;
        }

        // exclude leading '/'
        let expression_l = expression_l.adjust_start(1);
        let options = string_value(expression_l, buffer);

        let mut options = options.as_str().chars().collect::<Vec<_>>();
        options.sort_unstable();
        options.dedup();
        let options = if options.is_empty() {
            None
        } else {
            Some(StringContent::from(options.into_iter().collect::<String>()))
        };

        Some(Box::new(Node::RegOpt(RegOpt {
            options,
            expression_l,
        })))
    }

    pub(crate) fn regexp_compose(
        begin_t: Token,
        parts: Vec<Node>,
        end_t: Token,
        options: Option<Box<Node>>,
    ) -> Box<Node> {
        let begin_l = begin_t.loc;
        let end_l = end_t.loc.resize(1);
        let expression_l = begin_l
            .join(&end_l)
            .maybe_join(&options.as_ref().map(|options| *options.expression()));

        match options.as_deref() {
            Some(Node::RegOpt(RegOpt {
                options,
                expression_l,
            })) => {
                // TODO: validate_static_regexp
            }
            None => {
                // TODO: validate_static_regexp
            }
            _ => unreachable!("bug: must be Option<RegOpt>"),
        }

        Box::new(Node::Regexp(Regexp {
            parts,
            options,
            begin_l,
            end_l,
            expression_l,
        }))
    }
}
