use std::{fs, io::Write, path::Path};

pub struct HeaderEmitter {
    pub funcs: Vec<FuncID>,
}

pub struct FuncID {
    pub name: String,
    pub typ: String,
    pub args: Vec<(String, String)>,
}

impl HeaderEmitter {
    pub fn new() -> HeaderEmitter {
        HeaderEmitter { funcs: Vec::new() }
    }
    pub fn emit_c(self, out_dir: &Path) {
        let mut header_file = fs::File::create((*out_dir).join("out.h")).unwrap();
        writeln!(header_file, "#ifndef IRN_HEADER").unwrap();
        writeln!(header_file, "#define IRN_HEADER").unwrap();
        writeln!(header_file, "#include <stdint.h>\n#include <stdio.h>").unwrap();
        for sig in self.funcs {
            writeln!(
                header_file,
                "{} {}({});",
                sig.typ,
                sig.name,
                format_args(sig.args)
            )
            .unwrap();
        }
        writeln!(header_file, "#endif").unwrap();
    }
    pub fn add_func(&mut self, ret: String, name: String, args: Vec<(String, String)>) {
        let id = FuncID {
            name,
            typ: ret,
            args,
        };
        self.funcs.push(id);
    }
}

pub fn format_args(args: Vec<(String, String)>) -> String {
    let mut ret = String::new();
    let mut args_mut = args;
    args_mut.reverse();
    let first = args_mut.pop();
    if let Some(t) = first {
        ret = format!("{} {}", t.0, t.1)
    }
    for arg in args_mut {
        ret = format!("{}, {} {}", ret, arg.0, arg.1);
    }
    ret
}
