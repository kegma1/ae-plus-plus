use std::env;

mod cross_ref;
mod execute;
mod lex;
mod ops;
mod parse;

#[derive(Debug)]
pub struct Runtime {
    pub stack: Vec<ops::Value>,
    pub str_heap: Vec<String>,
}

impl Runtime{
    pub fn new() -> Self {
        Runtime {
            stack: vec![],
            str_heap: vec![],
        }
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
                if let Err(e) = res {
                    println!("ERROR: {}, {}:{}:{}", e.0, e.1 .0, e.1 .1, e.1 .2);
                }
                println!("\nstack: {:?}\nStrings: {:?}", ctx.stack, ctx.str_heap)
            }
        },
        None => {
            let res = run(path);
            if let Err(e) = res {
                println!("ERROR: {}, {}:{}:{}", e.0, e.1 .0, e.1 .1, e.1 .2)
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