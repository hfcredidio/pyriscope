use nom::types::CompleteStr;
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<CompleteStr<'a>>;

#[derive(Debug)]
pub enum Token {
    Endmarker,
    Name,
    Number,
    String,
    Newline,
    Indent,
    Dedent,
    Lpar,
    Rpar,
    Lsqb,
    Rsqb,
    Colon,
    Comma,
    Semi,
    Plus,
    Minus,
    Star,
    Slash,
    Vbar,
    Amper,
    Less,
    Greater,
    Equal,
    Dot,
    Percent,
    Lbrace,
    Rbrace,
    Eqequal,
    Notequal,
    Lessequal,
    Greaterequal,
    Tilde,
    Circumflex,
    Leftshift,
    Rightshift,
    Doublestar,
    Plusequal,
    Minequal,
    Starequal,
    Slashequal,
    Percentequal,
    Amperequal,
    Vbarequal,
    Circumflexequal,
    Leftshiftequal,
    Rightshiftequal,
    Doublestarequal,
    Doubleslash,
    Doubleslashequal,
    At,
    Atequal,
    Rarrow,
    Ellipsis,
    Colonequal,
    Op,
    Await,
    Async,
    TypeIgnore,
    TypeComment,
    Errortoken,
    Comment,
    Nl,
    Encoding,
}

#[derive(Debug)]
pub enum Symbol {
    SingleInput,
    FileInput,
    EvalInput,
    Decorator,
    Decorators,
    Decorated,
    AsyncFuncdef,
    Funcdef,
    Parameters,
    Typedargslist,
    Tfpdef,
    Varargslist,
    Vfpdef,
    Stmt,
    SimpleStmt,
    SmallStmt,
    ExprStmt,
    Annassign,
    TestlistStarExpr,
    Augassign,
    DelStmt,
    PassStmt,
    FlowStmt,
    BreakStmt,
    ContinueStmt,
    ReturnStmt,
    YieldStmt,
    RaiseStmt,
    ImportStmt,
    ImportName,
    ImportFrom,
    ImportAsName,
    DottedAsName,
    ImportAsNames,
    DottedAsNames,
    DottedName,
    GlobalStmt,
    NonlocalStmt,
    AssertStmt,
    CompoundStmt,
    AsyncStmt,
    IfStmt,
    WhileStmt,
    ForStmt,
    TryStmt,
    WithStmt,
    WithItem,
    ExceptClause,
    Suite,
    NamedexprTest,
    Test,
    TestNocond,
    Lambdef,
    LambdefNocond,
    OrTest,
    AndTest,
    NotTest,
    Comparison,
    CompOp,
    StarExpr,
    Expr,
    XorExpr,
    AndExpr,
    ShiftExpr,
    ArithExpr,
    Term,
    Factor,
    Power,
    AtomExpr,
    Atom,
    TestlistComp,
    Trailer,
    Subscriptlist,
    Subscript,
    Sliceop,
    Exprlist,
    Testlist,
    Dictorsetmaker,
    Classdef,
    Arglist,
    Argument,
    CompIter,
    SyncCompFor,
    CompFor,
    CompIf,
    EncodingDecl,
    YieldExpr,
    YieldArg,
    FuncBodySuite,
    FuncTypeInput,
    FuncType,
    Typelist,
}

#[derive(Debug)]
pub enum Node {
    Terminal {
        tok: Token,
        value: String,
        line: u32,
    },
    NonTerminal {
        sym: Symbol,
        children: Vec<Node>,
    }
}

pub trait MoveToVec<T> {
    fn move_to_vec(self, v: &mut Vec<T>);
}

impl MoveToVec<Node> for Node {
    fn move_to_vec(self, v: &mut Vec<Node>) {
        v.push(self);
    }
}

impl<U, T: MoveToVec<U>> MoveToVec<U> for Option<T> {
    fn move_to_vec(self, v: &mut Vec<U>) {
        if let Some(value) = self {
            value.move_to_vec(v);
        }
    }
}

impl<U, T: MoveToVec<U>> MoveToVec<U> for Vec<T> {
    fn move_to_vec(mut self, v: &mut Vec<U>) {
        for i in self.drain(..) {
            i.move_to_vec(v);
        }
    }
}

impl<U, T: MoveToVec<U>> MoveToVec<U> for (T,) {
    fn move_to_vec(self, v: &mut Vec<U>) {
        self.0.move_to_vec(v);
    }
}

impl<U, T: MoveToVec<U>, V: MoveToVec<U>> MoveToVec<U> for (T,V) {
    fn move_to_vec(self, v: &mut Vec<U>) {
        self.0.move_to_vec(v);
        self.1.move_to_vec(v);
    }
}

impl<U, T: MoveToVec<U>, V: MoveToVec<U>, W: MoveToVec<U>> MoveToVec<U> for (T,V,W) {
    fn move_to_vec(self, v: &mut Vec<U>) {
        self.0.move_to_vec(v);
        self.1.move_to_vec(v);
        self.2.move_to_vec(v);
    }
}

macro_rules! gvec {
    ($($x:expr),*) => {{
        let mut v = vec![];
        $(($x).move_to_vec(&mut v);)*
        v
    }};
}
