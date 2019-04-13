#[macro_use] mod types;
mod token;

use nom::types::CompleteStr;
use types::Span;

fn main() {
    println!("{:?}", token::name(Span::new(CompleteStr("aaa"))));
}
