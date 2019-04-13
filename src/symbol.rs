use crate::types::{Span, Node, Symbol, MoveToVec};
use crate::token::*;
use nom::*;

named!(pub dotted_name<Span, Node>, ws!(do_parse!(
    first: name >>
    rest: many0!(tuple!(dot, name)) >>
    (Node::NonTerminal {
        sym: Symbol::DottedName,
        children: gvec![first, rest],
    })
)));

named!(pub dotted_as_name<Span, Node>, ws!(do_parse!(
    dot_name: dotted_name >>
    alias: opt!(tuple!(as_, name)) >>
    (Node::NonTerminal {
        sym: Symbol::DottedAsName,
        children: gvec![dot_name, alias],
    })
)));

named!(pub dotted_as_names<Span, Node>, ws!(do_parse!(
    first: dotted_as_name >>
    rest: many0!(tuple!(comma, dotted_as_name)) >>
    (Node::NonTerminal {
        sym: Symbol::DottedAsNames,
        children: gvec![first, rest],
    })
)));

named!(pub import_name<Span, Node>, ws!(do_parse!(
    imp: import >>
    module_name: dotted_as_names >>
    (Node::NonTerminal {
        sym: Symbol::ImportName,
        children: gvec![imp, module_name],
    })
)));

named!(pub import_as_name<Span, Node>, ws!(do_parse!(
    module_name: name >>
    alias: opt!(tuple!(as_, name)) >>
    (Node::NonTerminal {
        sym: Symbol::ImportAsName,
        children: gvec![module_name, alias],
    })
)));

named!(pub import_as_names<Span, Node>, ws!(do_parse!(
    first: import_as_name >>
    rest: many0!(tuple!(comma, import_as_name)) >>
    trailing_comma: opt!(comma) >>
    (Node::NonTerminal {
        sym: Symbol::ImportAsNames,
        children: gvec![first, rest, trailing_comma],
    })
)));

named!(pub import_from<Span, Node>, ws!(do_parse!(
    from_: from >>
    module: alt!(
       map!(
           tuple!(many0!(alt!(ellipsis | dot)), dotted_name),
           |res| gvec![res.0, res.1]
       ) |
       many1!(alt!(ellipsis | dot))
    ) >>
    import_: import >>
    stuff: alt!(
        map!(star, |res| vec![res]) | 
        map!(tuple!(lpar, import_as_names, rpar), |res| vec![res.0, res.1, res.2]) |
        map!(import_as_names, |res| vec![res])
    ) >>
    (Node::NonTerminal {
        sym: Symbol::ImportFrom,
        children: gvec![from_, module, import_, stuff],
    })
)));

named!(pub import_stmt<Span, Node>, alt!(import_name | import_from));

named!(pub global_stmt<Span, Node>, ws!(do_parse!(
    global_: global >>
    first: name >>
    rest: many0!(tuple!(comma, name)) >>
    (Node::NonTerminal {
        sym: Symbol::GlobalStmt,
        children: gvec![global_, first, rest],
    })
)));

named!(pub nonlocal_stmt<Span, Node>, ws!(do_parse!(
    nonlocal_: nonlocal >>
    first: name >>
    rest: many0!(tuple!(comma, name)) >>
    (Node::NonTerminal {
        sym: Symbol::NonlocalStmt,
        children: gvec![nonlocal_, first, rest],
    })
)));
