use std::collections::HashMap;
use std::{env, fmt};
use termsize;


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
    pub def: HashMap<String, Option<ops::Value>>,
    pub return_stack: Vec<usize>,
    frame_stack: Vec<(Vec<ops::Value>, Option<ops::TypeLiteral>)>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            stack: vec![],
            mem: vec![],
            top: 0,
            def: HashMap::new(),
            return_stack: vec![],
            frame_stack: vec![],
        }
    }

    pub fn push(&mut self, x: ops::Value) {
        self.stack.push(x);
    }

    pub fn pop(&mut self) -> Option<ops::Value> {
        self.stack.pop()
    }

    pub fn peek(&mut self) -> Option<&ops::Value> {
        match self.stack.len() {
            0 => None,
            n => Some(&self.stack[n - 1]),
        }
    }

    pub fn swap(&mut self, new_stack: Vec<ops::Value>, ret_typ: Option<ops::TypeLiteral>) {
        self.frame_stack.push((self.stack.clone(), ret_typ));
        self.stack = new_stack
    }

    pub fn retur(&mut self) -> Option<ops::TypeLiteral> {
        let (old_stack, ret_type) = self.frame_stack.pop().unwrap();
        self.stack = old_stack;
        ret_type
    }

    pub fn write(&mut self, data: &Vec<ops::Value>) -> (ops::Ptr, usize) {
        let ptr = self.top;
        for val in data {
            self.mem.push(val.clone());
            self.top += 1;
        }
        (ptr, data.len())
    }

    pub fn over_write(&mut self, ptr: ops::Ptr, data: &ops::Value) {
        self.mem[ptr] = data.clone()
    }

    pub fn read(&self, ptr: ops::Ptr) -> Option<ops::Value> {
        self.mem.get(ptr).copied()
    }

    pub fn read_data(&self, ptr: ops::Ptr, len: usize) -> Option<&[ops::Value]> {
        self.mem.get(ptr..(ptr + len))
    }

    pub fn read_str(&self, str_ptr: &ops::Value) -> Option<String> {
        if let ops::Value::Str((ptr, len)) = str_ptr {
            Some(
                self.read_data(*ptr, *len)
                    .unwrap()
                    .iter()
                    .map(|x| {
                        if let ops::Value::Char(c) = x {
                            c.clone()
                        } else {
                            '\0'
                        }
                    })
                    .collect::<String>(),
            )
        } else {
            None
        }
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stabel: ")?;
        for v in &self.stack {
            write!(f, "{}, ", v.to_string(&self))?;
        }
        write!(f,"\n")?;

        write!(f, "Minne: ")?;
        let mut mem = String::from("");
        for v in &self.mem {
            mem.push_str(&format!("{}, ", v.to_string(self)))
        }
        let width = termsize::get().unwrap().cols.into();
        if (mem.len() + 7) <= width {
            write!(f, "{}\n", mem)?;
        } else {
            write!(f, "{}...\n", &mem[0..(width - 10)])?;
        }
        
        
        Ok(())
    }
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
                println!("{}", ctx)
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
