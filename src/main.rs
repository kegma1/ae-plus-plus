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
    let path = &args[1];

    // let mut ctx = Runtime::new();

    // let lexed = lex::lex(path).unwrap();
    // let mut parsed = parse::parse(lexed, &mut ctx).unwrap();
    // let cross_refed = cross_ref::cross_reference(&mut parsed).unwrap();

    // for (i,tok) in cross_refed.iter().enumerate() {
    //     println!("{} {}", i, tok)
    // }

    let res = run(path);
    if let Err(e) = res {
        println!("ERROR: {}, {}:{}:{}", e.0, e.1 .0, e.1 .1, e.1 .2)
    }
}

fn run(path:&String) -> Result<u8, (&'static str, ops::Pos)> {
    let mut ctx = Runtime::new();

    let lexed = lex::lex(path)?;
    let mut parsed = parse::parse(lexed, &mut ctx)?;
    let cross_refed = cross_ref::cross_reference(&mut parsed)?;
    execute::execute(&mut ctx, &cross_refed)
}