#[macro_use]
extern crate lalrpop_util;

mod frontend;

fn main() {
    frontend::parser::parse("1+1*2");
}
