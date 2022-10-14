use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

#[derive(Debug, PartialEq)]
enum Type {
    Int(i32),
    Float(f32),
    Bool(bool),
    // String
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Int(x) => write!(f, "{}", x),
            Type::Float(x) => write!(f, "{}", x),
            Type::Bool(x) => write!(f, "{}", x)
        } 
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
                // println!("{:?}", parse(ip).unwrap())
                execute(&mut ctx, parse(ip).unwrap());
            }
        }
    }
}

fn parse(line: String) -> Result<Vec<Instructions>, &'static str> {
    let mut parsed_line: Vec<Instructions> = vec![];
    
    for token in line.split(" ") {
        parsed_line.push(match token {
            x if x.contains("#") => break,
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
            x if x.parse::<i32>().is_ok() => Instructions::Literal(Type::Int(x.parse::<i32>().unwrap())),
            x if x.parse::<f32>().is_ok() => Instructions::Literal(Type::Float(x.parse::<f32>().unwrap())),
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
                println!("{}", print_value)
            },
            Instructions::Add => {
                assert!(ctx.stack.len() >= 2);

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b { y } else { panic!("not an Int")};
                        ctx.stack.push(Type::Int(x + y));

                    },
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b { y } else { panic!("not a Float")};
                        ctx.stack.push(Type::Float(x + y));

                    }
                    _ => panic!("{:?} does not support add operator", a)
                }
            },
            Instructions::Sub => {
                assert!(ctx.stack.len() >= 2);

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b { y } else { panic!("not an Int")};
                        ctx.stack.push(Type::Int(x - y));

                    },
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b { y } else { panic!("not a Float")};
                        ctx.stack.push(Type::Float(x - y));

                    }
                    _ => panic!("{:?} does not support sub operator", a)
                }
            },
            Instructions::Mult => {
                assert!(ctx.stack.len() >= 2);

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b { y } else { panic!("not an Int")};
                        ctx.stack.push(Type::Int(x * y));

                    },
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b { y } else { panic!("not a Float")};
                        ctx.stack.push(Type::Float(x * y));

                    }
                    _ => panic!("{:?} does not support mult operator", a)
                }
            },
            Instructions::Div => {
                assert!(ctx.stack.len() >= 2);

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b { y } else { panic!("not an Int")};
                        ctx.stack.push(Type::Int(x / y));

                    },
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b { y } else { panic!("not a Float")};
                        ctx.stack.push(Type::Float(x / y));

                    }
                    _ => panic!("{:?} does not support div operator", a)
                }
            },
            Instructions::Mod => {
                assert!(ctx.stack.len() >= 2);

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b { y } else { panic!("not an Int")};
                        ctx.stack.push(Type::Int(x % y));

                    },
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b { y } else { panic!("not a Float")};
                        ctx.stack.push(Type::Float(x % y));

                    }
                    _ => panic!("{:?} does not support mod operator", a)
                }
            }
            Instructions::Not => {
                assert!(ctx.stack.len() >= 1);
                let b = ctx.stack.pop().unwrap();
                match b {
                    Type::Bool(x) => {
                        ctx.stack.push(Type::Bool(!x));
                    },
                    _ => panic!("{:?} does not support not operator", b)
                }
            },
            Instructions::And => {
                assert!(ctx.stack.len() >= 2);

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    Type::Bool(x) => {
                        let y = if let Type::Bool(y) = b { y } else { panic!("not a Bool")};
                        ctx.stack.push(Type::Bool(x && y));
                    },
                    _ => panic!("{:?} does not support and operator", a)
                }
            },
            Instructions::Or => {
                assert!(ctx.stack.len() >= 2);

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    Type::Bool(x) => {
                        let y = if let Type::Bool(y) = b { y } else { panic!("not a Bool")};
                        ctx.stack.push(Type::Bool(x || y));
                    },
                    _ => panic!("{:?} does not support or operator", a)
                }
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
