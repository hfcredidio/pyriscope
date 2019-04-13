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


macro_rules! make_tag_parser {
    ($name:ident, $tag:expr, $tok:path) => {
        named!(pub $name<Span, Node>, do_parse!(
            value: tag!($tag) >>
            position: position!() >>
            (Node::Terminal {
                tok: $tok,
                value: value.to_string(),
                line: position.line,
            })
        ));
    }
}

make_tag_parser!(lpar, "(", Token::Lpar);
make_tag_parser!(rpar, ")", Token::Rpar);
make_tag_parser!(lsqb, "[", Token::Lsqb);
make_tag_parser!(rsqb, "]", Token::Rsqb);
make_tag_parser!(colon, ":", Token::Colon);
make_tag_parser!(comma, ",", Token::Comma);
make_tag_parser!(semi, ";", Token::Semi);
make_tag_parser!(plus, "+", Token::Plus);
make_tag_parser!(minus, "-", Token::Minus);
make_tag_parser!(star, "*", Token::Star);
make_tag_parser!(slash, "/", Token::Slash);
make_tag_parser!(vbar, "|", Token::Vbar);
make_tag_parser!(amper, "&", Token::Amper);
make_tag_parser!(less, "<", Token::Less);
make_tag_parser!(greater, ">", Token::Greater);
make_tag_parser!(equal, "=", Token::Equal);
make_tag_parser!(dot, ".", Token::Dot);
make_tag_parser!(percent, "%", Token::Percent);
make_tag_parser!(lbrace, "{", Token::Lbrace);
make_tag_parser!(rbrace, "}", Token::Rbrace);
make_tag_parser!(eqequal, "==", Token::Eqequal);
make_tag_parser!(notequal, "!=", Token::Notequal);
make_tag_parser!(lessequal, "<=", Token::Lessequal);
make_tag_parser!(greaterequal, ">=", Token::Greaterequal);
make_tag_parser!(tilde, "~", Token::Tilde);
make_tag_parser!(circumflex, "^", Token::Circumflex);
make_tag_parser!(leftshift, "<<", Token::Leftshift);
make_tag_parser!(rightshift, ">>", Token::Rightshift);
make_tag_parser!(doublestar, "**", Token::Doublestar);
make_tag_parser!(plusequal, "+=", Token::Plusequal);
make_tag_parser!(minequal, "-=", Token::Minequal);
make_tag_parser!(starequal, "*=", Token::Starequal);
make_tag_parser!(slashequal, "/=", Token::Slashequal);
make_tag_parser!(percentequal, "%=", Token::Percentequal);
make_tag_parser!(amperequal, "&=", Token::Amperequal);
make_tag_parser!(vbarequal, "|=", Token::Vbarequal);
make_tag_parser!(circumflexequal, "^=", Token::Circumflexequal);
make_tag_parser!(leftshiftequal, "<<=", Token::Leftshiftequal);
make_tag_parser!(rightshiftequal, ">>=", Token::Rightshiftequal);
make_tag_parser!(doublestarequal, "**=", Token::Doublestarequal);
make_tag_parser!(doubleslash, "//", Token::Doubleslash);
make_tag_parser!(doubleslashequal, "//=", Token::Doubleslashequal);
make_tag_parser!(at, "@", Token::At);
make_tag_parser!(atequal, "@=", Token::Atequal);
make_tag_parser!(rarrow, "->", Token::Rarrow);
make_tag_parser!(ellipsis, "...", Token::Ellipsis);
make_tag_parser!(colonequal, ":=", Token::Colonequal);

make_tag_parser!(as_, "as", Token::Name);
make_tag_parser!(import, "import", Token::Name);
make_tag_parser!(from, "from", Token::Name);
make_tag_parser!(global, "global", Token::Name);
make_tag_parser!(nonlocal, "nonlocal", Token::Name);
