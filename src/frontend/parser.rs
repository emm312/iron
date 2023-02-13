lalrpop_mod!(pub iron);

pub fn parse(src: &str) {
    let expr = iron::FuncArgsParser::new()
        .parse(src)
        .unwrap();
    println!("{:#?}", expr);
}