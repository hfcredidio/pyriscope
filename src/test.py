from lark import Lark

grammar = Lark(r"""
    grammar : rule+
    rule    : CNAME ":" productionrule
    productionrule : production ("|" production)*
    production : term*
    term : element repeats | "[" productionrule "]"
    element : STRING | CNAME | "(" productionrule ")"
    repeats :  "*" | "+"

    STRING : /'.*?(?<!\\)'/
    %import common.CNAME
    %import common.WS
    %ignore WS
""", start="grammar")

python_grammar = r"""
import_as_name: NAME ['as' NAME]
dotted_as_name: dotted_name ['as' NAME]
import_as_names: import_as_name (',' import_as_name)* [',']
dotted_as_names: dotted_as_name (',' dotted_as_name)*
dotted_name: NAME ('.' NAME)*
"""
parsed = grammar.parse(python_grammar)

print(parsed.pretty())
