#[macro_use]
extern crate lalrpop_util;

mod frontend;

fn main() {
    frontend::parser::parse("var: i32");
}
