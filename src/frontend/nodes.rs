#[derive(Debug)]
pub enum Node {
    ExprNode(Expr),
    FuncDefNode(String, Vec<(String, Types)>, Types, Vec<Node>),
    VarDefNode(String, Types, Expr),
    IfNode(Expr, Vec<Node>),
    WhileNode(Expr, Vec<Node>),
    ReturnNode(Expr),
    VarAssignNode(String, Expr),
    ExternNode(String, Vec<(String, Types)>, Types), // ForLoopNode( TODO: Decide syntax
                                                     //     Box<Node>, Expr, Box<Node>,
                                                     //     Vec<Node>
                                                     // )
}

#[derive(Debug, Clone)]
pub enum Expr {
    BiOpNode(Box<Expr>, Operation, Box<Expr>),
    ComparisonNode(Box<Expr>, Comparison, Box<Expr>),
    Num(i64),
    Ident(String),
    FuncCall(String, Vec<Expr>),
    String(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, Copy)]
pub enum Comparison {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Types {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    STRING,
    VOID,
}
