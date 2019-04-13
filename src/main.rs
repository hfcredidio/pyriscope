#[macro_use] mod types;
mod token;
mod symbol;

use nom::types::CompleteStr;
use types::Span;

fn main() {
    println!("{:?}", symbol::dotted_name(Span::new(CompleteStr("a.a.a"))));
}
