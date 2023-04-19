/*use std::{collections::HashMap, process::exit};

use crate::frontend::nodes::{Node, Expr, Types};

pub fn check_types(ast: Vec<Node>) {
    let mut vars: HashMap<String, Types> = HashMap::new();
    let mut functions: HashMap<String, (Types, Vec<Types>)> = HashMap::new();
    for statement in ast {
        match statement {
            Node::VarDefNode(name, typ, expr) => {
                check_expr(expr, Some(typ), &mut vars.clone(), &mut functions);
                let n = name.as_str();
                vars.insert(n.to_string(), typ);
            }
            _ => todo!("add typechecking for node {:?}", statement)
        }
    }
}

fn check_func(body: Vec<Node>, ret_type: Types) {
    for node in body {

    }
}

fn check_expr(expr: Expr, eval_to: Option<Types>, vars: &mut HashMap<String, Types>, funcs: &mut HashMap<String, (Types, Vec<Types>)>) {
    let has_typ;
    match eval_to {
        None => has_typ = false,
        Some(_) => has_typ = true,
    }
    match expr {
        Expr::BiOpNode(lhs, op, rhs) => {
            check_expr(*lhs, eval_to, vars, funcs);
        },
        Expr::Num(_) => {
            if has_typ {
                if eval_to.unwrap().is_integer_type() != true {
                    println!("Invalid type.");
                    exit(-1);
                }
            }
        }
        Expr::String(_) => {
            if has_typ {
                if eval_to.unwrap() != Types::STRING {
                    println!("Invalid type.");
                    exit(-1);
                }
            }
        }
        Expr::FuncCall(name, args) => {
            for expr in args {

            }
        }
        _ => todo!()

    }
}*/
