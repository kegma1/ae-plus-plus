use std::collections::HashMap;
use std::env;

mod cross_ref;
mod execute;
mod lex;
mod ops;
mod parse;

#[derive(Debug)]
pub struct Runtime {
    stack: Vec<ops::Value>,
    mem: Vec<ops::Value>,
    top: usize,
    pub str_heap: Vec<String>,
    pub def: HashMap<String, Option<ops::Value>>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            stack: vec![],
            str_heap: vec![],
            mem: vec![],
            top: 0,
            def: HashMap::new(),
        }
    }

    pub fn push(&mut self, x: ops::Value) {
        self.stack.push(x);
    }

    pub fn pop(&mut self) -> Option<ops::Value> {
        self.stack.pop()
    }

    pub fn write(&mut self, data: &Vec<ops::Value>) -> (ops::Ptr, usize) {
        let ptr = self.top;
        for val in data {
            self.mem.push(val.clone());
            self.top += 1;
        }
        (ptr, data.len())
    }

    pub fn read(&self, ptr: ops::Ptr) -> Option<ops::Value> {
        self.mem.get(ptr).copied()
    }

    pub fn read_data(&self, ptr: ops::Ptr, len: usize) -> Option<&[ops::Value]> {
        self.mem.get(ptr..(ptr + len))
    }

    pub fn read_str(&self, str_ptr: &ops::Value) -> Option<String> {
        if let ops::Value::Str((ptr, len)) = str_ptr {
            Some(self
        .read_data(*ptr, *len)
        .unwrap()
        .iter()
        .map(|x| {
            if let ops::Value::Char(c) = x {
                c.clone()
            } else {
                '\0'
            }
        }).collect::<String>())
    } else {
        None
    }}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag: Option<&String>;
    let path: &String;
    if args.len() <= 2 {
        path = &args[1];
        flag = None;
    } else {
        path = &args[2];
        flag = Some(&args[1])
    }

    match flag {
        Some(x) => {
            if x == "-d" {
                let mut ctx = Runtime::new();
                let res = debug_run(path, &mut ctx);
                if let Err((e, pos)) = res {
                    println!("{}:{}:{}  ERROR: {}\n", pos.2, pos.0, pos.1, e)
                }
                println!(
                    "\nStabel: {:?}\nStrenger: {:?}\nDefinisjoner: {:?}\nMinne: {:?}",
                    ctx.stack, ctx.str_heap, ctx.def, ctx.mem
                )
            }
        }
        None => {
            let res = run(path);
            if let Err((e, pos)) = res {
                println!("{}:{}:{}  ERROR: {}\n", pos.2, pos.0, pos.1, e)
            }
        }
    }
}

fn debug_run(path: &String, ctx: &mut Runtime) -> Result<u8, (&'static str, ops::Pos)> {
    let lexed = lex::lex(path)?;
    let mut parsed = parse::parse(lexed, ctx)?;
    let cross_refed = cross_ref::cross_reference(&mut parsed, &ctx)?;
    // for (i, inst) in cross_refed.iter().enumerate() {
    //     println!("{}: {}", i, inst)
    // }
    execute::execute(ctx, &cross_refed)
}

fn run(path: &String) -> Result<u8, (&'static str, ops::Pos)> {
    let mut ctx = Runtime::new();

    let lexed = lex::lex(path)?;
    let mut parsed = parse::parse(lexed, &mut ctx)?;
    let cross_refed = cross_ref::cross_reference(&mut parsed, &ctx)?;
    execute::execute(&mut ctx, &cross_refed)
}
