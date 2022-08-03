macro_rules! gen_all_of {
    ($name:ident; generic = $($generic:tt),+; fields = $($field:ident),+; next = $next:ident) => {
        #[allow(unused_parens)]
        pub(crate) struct $name<$($generic),+>
        where
            $(Steps: From<$generic>),+
        {
            pub(crate) name: &'static str,
            pub(crate) inner: Result<($($generic),+), SeqError>,
        }

        #[allow(dead_code)]
        #[allow(unused_parens)]
        impl<$($generic),+> $name<$($generic),+>
        where
            $(Steps: From<$generic>),+
        {
            pub(crate) fn and<N, Func>(self, f: Func) -> $next<$($generic),+, N>
            where
                Func: FnOnce() -> Result<N, ParseError>,
                Steps: From<N>,
            {
                let Self { inner, name } = self;
                match inner {
                    Ok(($($field),+)) => {
                        match f() {
                            Ok(d) => $next {
                                name,
                                inner: Ok(($($field),+, d)),
                            },
                            Err(mut error) => {
                                // this the 2nd+ element in a sequence,
                                // so all lookahead errors must become
                                // non-lookahead
                                error.make_required();

                                let mut steps: Steps = Steps::empty();
                                $(steps += Steps::from($field);)+

                                $next {
                                    name,
                                    inner: Err(SeqError {
                                        steps,
                                        error,
                                    }),
                                }
                            }
                        }
                    }
                    Err(err) => $next {
                        name,
                        inner: Err(err),
                    },
                }
            }

            pub(crate) fn stop(self) -> Result<($($generic),+), ParseError> {
                let Self { inner, name } = self;
                match inner {
                    Ok(($($field),+)) => Ok(($($field),+)),
                    Err(SeqError { steps, error }) => Err(ParseError::SeqError {
                        name,
                        steps,
                        error: Box::new(error),
                    }),
                }
            }
        }

    };
}
pub(crate) use gen_all_of;
