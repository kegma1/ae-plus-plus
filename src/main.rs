use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Number(i32),
    Bool(bool),
    // String
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Number(x) => write!(f, "{}", x),
            Type::Bool(x) => write!(f, "{}", x)
        } 
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instructions {
    Literal(Type),
    Add,
    Sub,
    Mult,
    Div,
    Mod,

    Print,

    Not,
    And,
    Or,

    Null
}

#[derive(Debug, PartialEq, Eq)]
struct Runtime {
    stack: Vec<Type>
}

impl Runtime {
    fn new() -> Self {
        Runtime { stack: vec![] }
    }
}


fn main() {
    let args:Vec<String> = env::args().collect();
    let path = &args[1];

    let mut ctx = Runtime::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                execute(&mut ctx, parse(ip).unwrap());
            }
        }
    }
}

fn parse(line: String) -> Result<Vec<Instructions>, &'static str> {
    let mut parsed_line: Vec<Instructions> = vec![];
    
    for token in line.split(" ") {
        parsed_line.push(match token {
            "toki" => Instructions::Print,
            "+" => Instructions::Add,
            "-" => Instructions::Sub,
            "*" => Instructions::Mult,
            "/" => Instructions::Div,
            "%" => Instructions::Mod,
            "ike" => Instructions::Not,
            "en" => Instructions::And,
            "anu" => Instructions::Or,
            "lon" => Instructions::Literal(Type::Bool(true)),
            x if x.parse::<i32>().is_ok() => Instructions::Literal(Type::Number(x.parse::<i32>().unwrap())),
            _ => Instructions::Null
        })
    }
    Ok(parsed_line)
}

fn execute(ctx: &mut Runtime, line: Vec<Instructions>) {

    for token in line {
        match token {
            Instructions::Print => {
                assert!(ctx.stack.len() >= 1);

                let print_value = ctx.stack.pop().unwrap();
                print!("{}", print_value)
            },
            Instructions::Add => {
                assert!(ctx.stack.len() >= 2);
                let b = if let Type::Number(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                let a = if let Type::Number(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                ctx.stack.push(Type::Number(a + b));
            },
            Instructions::Sub => {
                assert!(ctx.stack.len() >= 2);
                let b = if let Type::Number(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                let a = if let Type::Number(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                ctx.stack.push(Type::Number(a - b));
            },
            Instructions::Mult => {
                assert!(ctx.stack.len() >= 2);
                let b = if let Type::Number(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                let a = if let Type::Number(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                ctx.stack.push(Type::Number(a * b));
            },
            Instructions::Div => {
                assert!(ctx.stack.len() >= 2);
                let b = if let Type::Number(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                let a = if let Type::Number(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                ctx.stack.push(Type::Number(a / b));
            },
            Instructions::Mod => {
                assert!(ctx.stack.len() >= 2);
                let b = if let Type::Number(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                let a = if let Type::Number(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                ctx.stack.push(Type::Number(a % b));
            }
            Instructions::Not => {
                assert!(ctx.stack.len() >= 1);
                let b = if let Type::Bool(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                ctx.stack.push(Type::Bool(!b));
            },
            Instructions::And => {
                assert!(ctx.stack.len() >= 2);
                let b = if let Type::Bool(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                let a = if let Type::Bool(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                ctx.stack.push(Type::Bool(a && b));
            },
            Instructions::Or => {
                assert!(ctx.stack.len() >= 2);
                let b = if let Type::Bool(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                let a = if let Type::Bool(x) = ctx.stack.pop().unwrap() { x } else { panic!("emmm idk") };
                ctx.stack.push(Type::Bool(a || b));
            },
            Instructions::Literal(literal) => {
                ctx.stack.push(literal)
            },
            Instructions::Null => {continue;}
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
