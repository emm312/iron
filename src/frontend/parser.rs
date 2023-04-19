use std::process::exit;

use super::nodes::Node;

lalrpop_mod!(pub iron);

pub fn parse(src: &str) -> Vec<Node> {
    let res = iron::IronParser::new().parse(src);

    match res {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            exit(-1)
        }
    }
}
