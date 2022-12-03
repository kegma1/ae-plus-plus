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
    pub current_scope: usize,
    top: usize,
    pub def: HashMap<String, (Option<ops::Value>, usize)>,
    pub return_stack: Vec<usize>,
    frame_stack: Vec<Vec<ops::Value>>,
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
            current_scope: 0,
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

    pub fn call(&mut self, func: &ops::FuncPtr, i: usize) -> Option<usize> {
        let new_stack: Vec<ops::Value> = {
            let start = self.stack.len() - func.params.len();
            let params: Vec<Option<ops::Value>> = self.stack.drain(start..)
                .enumerate()
                .map(|(j, par)| {
                    if !par.eq(&func.params[j]) {
                        return None;
                    } else {Some(par)}
                })
                .collect();
            let mut arg = vec![];
            for par in params {
                if let Some(x) = par {
                    arg.push(x)
                } else {
                    return None;
                }
            }
            arg
        };
        self.frame_stack.push(self.stack.clone());
        self.stack = new_stack;
        self.return_stack.push(i);
        self.current_scope += 1;
        Some(func.ptr)
    }

    pub fn retur(&mut self, func: &ops::FuncPtr) -> Option<usize> {
        let mut returned_items: Vec<ops::Value> = {
            let start = self.stack.len() - func.returns.len();
            let returns = self.stack.get(start..).unwrap();
            for (j, par) in returns.iter().enumerate() {
                if !par.eq(&func.returns[j]) {
                    return None;
                }
            }
            returns.into()
        };

        let old_stack = self.frame_stack.pop().unwrap();
        self.stack = old_stack;
        self.stack.append(&mut returned_items);
        self.return_stack.pop()
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
        self.mem.get(ptr).cloned()
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
        let width = termsize::get().unwrap().cols.into();

        write!(f, "Stabel: ")?;
        let mut stack = String::from("");
        for v in &self.stack {
            stack.push_str(&format!("{}, ", v.to_string(self)));
        }

        if (stack.len() + 8) <= width {
            write!(f, "{}\n", stack)?;
        } else {
            write!(
                f,
                "...{}\n",
                &stack[(stack.len() - (width - 11))..(stack.len() - 1)]
            )?;
        }

        write!(f, "Minne: ")?;
        let mut mem = String::from("");
        for v in &self.mem {
            mem.push_str(&format!("{}, ", v.to_string(self)))
        }
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

#[macro_export]
macro_rules! report_err {
    ($tok:expr, $err_msg:expr) => {
        return Err((
            $err_msg,
            $tok.clone(),
        ))
    };

    ($pos:expr, $($arg:tt)*) => {
        let err_s: String = format!($($arg)*).to_owned();
        return Err((Box::leak(err_s.into_boxed_str()), $pos.clone()));
    };
}
