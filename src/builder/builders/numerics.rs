use crate::{
    buffer::Buffer,
    builder::{builders::helpers::string_value, Builder, Constructor},
    nodes::{Complex, Float, Int, Rational},
    token::Token,
    Node,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn integer(integer_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = integer_t.loc;
        Box::new(Node::Int(Int {
            value: string_value(expression_l, buffer),
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn float(float_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = float_t.loc;
        Box::new(Node::Float(Float {
            value: string_value(expression_l, buffer),
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn rational(rational_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = rational_t.loc;
        Box::new(Node::Rational(Rational {
            value: string_value(expression_l, buffer),
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn complex(complex_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = complex_t.loc;
        Box::new(Node::Complex(Complex {
            value: string_value(expression_l, buffer),
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn unary_num(unary_t: Token, mut numeric: Box<Node>, buffer: &Buffer) -> Box<Node> {
        let new_operator_l = unary_t.loc;

        match &mut *numeric {
            Node::Int(Int {
                value,
                expression_l,
                operator_l,
            })
            | Node::Float(Float {
                value,
                expression_l,
                operator_l,
            })
            | Node::Rational(Rational {
                value,
                operator_l,
                expression_l,
            })
            | Node::Complex(Complex {
                value,
                operator_l,
                expression_l,
            }) => {
                *operator_l = Some(new_operator_l);
                *expression_l = new_operator_l.join(expression_l);
                *value = string_value(*expression_l, buffer);
            }

            _ => {}
        }

        numeric
    }
}
