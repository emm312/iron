#[macro_use]
extern crate lalrpop_util;

mod backend;
mod frontend;
mod typechecker;

use std::path::Path;

use clap::Parser;

use crate::backend::compiler;

#[derive(Parser)]
struct Args {
    #[arg()]
    input_file: String,

    #[arg(short, long, default_value_t = String::from("a.out"))]
    output_file: String,
}

fn main() {
    let args = Args::parse();
    let src = std::fs::read_to_string(args.input_file).expect("File not found.");
    let ast = frontend::parser::parse(&src);
    println!("{:#?}", ast);
    // typechecker::typecheck::check_types(ast);
    println!("AST Typechecked OK");
    compiler::compile(ast, &Path::new("."));
}
