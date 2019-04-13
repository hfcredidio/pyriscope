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
