pub enum Node {
    ExprNode(Expr),
    FuncDefNode(
        String,
        Vec<(Types, String)>,
        Types
    )
}

#[derive(Debug)]
pub enum Expr {
    BiOpNode(Box<Expr>, Operation, Box<Expr>),
    ComparisonNode(Box<Expr>, Comparison, Box<Expr>),
    Num(i64),
    Ident(String),
    FuncCall(String, Vec<Box<Expr>>)
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

#[derive(Debug)]
pub enum Comparison {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte
}

#[derive(Debug)]
pub enum Types {
    Infer,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    STRING,
}