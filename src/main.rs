use std::env;

mod cross_ref;
mod execute;
mod lex;
mod ops;
mod parse;
const MEM_SIZE: usize = 360_000;
#[derive(Debug)]
pub struct Runtime {
    stack: Vec<ops::Value>,
    mem: [u8; MEM_SIZE],
    len: usize,
    pub str_heap: Vec<String>,
}

impl Runtime{
    pub fn new() -> Self {
        Runtime {
            stack: vec![],
            mem: [0u8; MEM_SIZE],
            len: 0,
            str_heap: vec![],
        }
    }

    pub fn push(&mut self, x: ops::Value) {
        self.stack.push(x);
    }

    pub fn pop(&mut self) -> Option<ops::Value> {
        self.stack.pop()
    }

    pub fn read(&mut self, addr: ops::Ptr, len: usize) -> Vec<u8> {
        let mut output = vec![];
        for i in addr..(addr + (len - 1)) {
            output.push(self.mem[i])
        }
        output
    }

    pub fn write(&mut self, val: Vec<u8>) -> (ops::Ptr, usize) {
        let addr = self.len;
        for x in &val {
            self.mem[self.len] = *x;
            self.len += 1
        }
        (addr, val.len())
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
                println!("\nstack: {:?}\nStrings: {:?}", ctx.stack, ctx.str_heap)
            }
        },
        None => {
            let res = run(path);
            if let Err((e, pos)) = res {
                println!("{}:{}:{}  ERROR: {}\n", pos.2, pos.0, pos.1, e)
            }
        }
    }    
}

fn debug_run(path:&String, ctx: &mut Runtime) -> Result<u8, (&'static str, ops::Pos)> {
    let lexed = lex::lex(path)?;
    let mut parsed = parse::parse(lexed, ctx)?;
    let cross_refed = cross_ref::cross_reference(&mut parsed)?;
    // for (i, inst) in cross_refed.iter().enumerate() {
    //     println!("{}: {}", i, inst)
    // }
    execute::execute(ctx, &cross_refed)
}

fn run(path:&String) -> Result<u8, (&'static str, ops::Pos)> {
    let mut ctx = Runtime::new();

    let lexed = lex::lex(path)?;
    let mut parsed = parse::parse(lexed, &mut ctx)?;
    let cross_refed = cross_ref::cross_reference(&mut parsed)?;
    execute::execute(&mut ctx, &cross_refed)
}