use std::{fs::File, path::Path};

use super::headeremitter::{format_args, HeaderEmitter};
use crate::frontend::nodes::{Comparison, Expr, Node, Operation, Types};
use std::io::Write;

pub fn compile(ast: Vec<Node>, out_dir: &Path) {
    let mut c_file = File::create(out_dir.join("out.c")).unwrap();
    let mut header_emit = HeaderEmitter::new();
    writeln!(c_file, "// Generated by the iron compiler").unwrap();
    writeln!(
        c_file,
        "#include \"out.h\"\n#include <stdint.h>\n#include <stdio.h>"
    )
    .unwrap();
    writeln!(c_file, "{}", compile_ast(ast, &mut header_emit)).unwrap();
    header_emit.emit_c(out_dir);
}

impl Types {
    pub fn to_c_type(self) -> String {
        String::from(match self {
            Types::I8 => "int8_t",
            Types::I16 => "int16_t",
            Types::I32 => "int32_t",
            Types::VOID => "void",
            _ => todo!(),
        })
    }
}

fn compile_ast(ast: Vec<Node>, emitter: &mut HeaderEmitter) -> String {
    let mut ret = String::new();
    for node in ast {
        match node {
            Node::FuncDefNode(name, args, typ, body) => {
                emitter.add_func(typ.to_c_type(), name.clone(), to_c_args(args.clone()));
                ret = format!(
                    "{}\n{} {}({}) {{{}\n}}",
                    ret,
                    typ.to_c_type(),
                    name,
                    format_args(to_c_args(args)),
                    compile_ast(body, emitter)
                );
            }
            Node::ExternNode(name, args, typ) => {
                ret = format!(
                    "{}\nextern {} {}({});",
                    ret,
                    typ.to_c_type(),
                    name,
                    format_args(to_c_args(args))
                );
            }
            Node::VarDefNode(name, typ, expr) => {
                ret = format!(
                    "{}\n{} {} = {};",
                    ret,
                    typ.to_c_type(),
                    name,
                    compile_expr(expr)
                );
            }
            Node::ReturnNode(expr) => {
                ret = format!("{}\nreturn {};", ret, compile_expr(expr));
            }
            Node::ExprNode(expr) => {
                ret = format!("{}\n{};", ret, compile_expr(expr));
            }
            Node::VarAssignNode(var, expr) => {
                ret = format!("{}\n{} = {};", ret, var, compile_expr(expr));
            }
            Node::IfNode(cond, body) => {
                ret = format!(
                    "{}\nif ({}) {{{}\n}}",
                    ret,
                    compile_expr(cond),
                    compile_ast(body, emitter)
                );
            }
            Node::WhileNode(cond, body) => {
                ret = format!(
                    "{}\nwhile ({}) {{{}\n}}",
                    ret,
                    compile_expr(cond),
                    compile_ast(body, emitter)
                );
            }
        }
    }
    ret
}

fn to_c_args(args: Vec<(String, Types)>) -> Vec<(String, String)> {
    let mut ret = vec![];
    for (name, typ) in args {
        ret.push((typ.to_c_type(), name));
    }
    ret
}

pub fn compile_expr(expr: Expr) -> String {
    match expr {
        Expr::Ident(i) => i,
        Expr::Num(n) => n.to_string(),
        Expr::BiOpNode(lhs, op, rhs) => {
            let lhs_val = compile_expr(*lhs);
            let rhs_val = compile_expr(*rhs);
            match op {
                Operation::Add => format!("{}+{}", lhs_val, rhs_val),
                Operation::Sub => format!("{}-{}", lhs_val, rhs_val),
                Operation::Div => format!("{}/{}", lhs_val, rhs_val),
                Operation::Mul => format!("{}*{}", lhs_val, rhs_val),
                Operation::Mod => format!("{}%{}", lhs_val, rhs_val),
            }
        }
        Expr::FuncCall(name, args) => {
            let mut args_str: String = String::new();
            let mut args_val = args.clone();
            args_val.reverse();
            let top = args_val.pop();
            match top {
                Some(t) => {
                    args_str = compile_expr(t);
                }
                None => (),
            }
            for arg in args_val {
                args_str = format!("{}, {}", args_str, compile_expr(arg));
            }
            format!("{}({})", name, args_str)
        }
        Expr::String(s) => {
            format!("\"{}\"", s)
        }
        Expr::ComparisonNode(a, cond, b) => {
            let lhs_val = compile_expr(*a);
            let rhs_val = compile_expr(*b);
            match cond {
                Comparison::Eq => {
                    format!("{} == {}", lhs_val, rhs_val)
                }
                Comparison::Neq => {
                    format!("{} != {}", lhs_val, rhs_val)
                }
                Comparison::Gt => {
                    format!("{} > {}", lhs_val, rhs_val)
                }
                Comparison::Lt => {
                    format!("{} < {}", lhs_val, rhs_val)
                }
                Comparison::Gte => {
                    format!("{} >= {}", lhs_val, rhs_val)
                }
                Comparison::Lte => {
                    format!("{} <= {}", lhs_val, rhs_val)
                }
            }
        }
    }
}
