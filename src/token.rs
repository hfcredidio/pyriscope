use crate::types::{Span, Node, Token};
use nom::*;
use nom_locate::*;

named!(pub name<Span, Node>, do_parse!(
    value: is_a!("a") >>
    position: position!() >>
    (Node::Terminal {
        tok: Token::Name,
        value: value.to_string(),
        line: position.line,
    })
));

named!(pub number<Span, Node>, do_parse!(
    value: is_a!("0123456789") >>
    position: position!() >>
    (Node::Terminal {
        tok: Token::Number,
        value: value.to_string(),
        line: position.line,
    })
));

named!(pub string<Span, Node>, do_parse!(
    value: recognize!(delimited!(tag!("'"), is_a!("a"), tag!("'"))) >>
    position: position!() >>
    (Node::Terminal {
        tok: Token::String,
        value: value.to_string(),
        line: position.line,
    })
));

named!(pub newline<Span, Node>, do_parse!(
    value: is_a!("\n") >>
    position: position!() >>
    (Node::Terminal {
        tok: Token::Newline,
        value: value.to_string(),
        line: position.line,
    })
));
