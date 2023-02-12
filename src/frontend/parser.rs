lalrpop_mod!(pub rocket);

pub fn parse(src: &str) {
    let expr = rocket::FullExprParser::new()
        .parse(src)
        .unwrap();
    println!("{:#?}", expr);
}