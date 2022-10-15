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
    
    If,
    End,
    
    Null
}

type Pos = (u32, u32);
type Lexeme = (Instructions, Pos);

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

    let lexed = lex(path);
    let parsed = parse(lexed);
    println!("{:?}", parsed);
    let _ = execute(&mut ctx, parsed.unwrap());

    
}

fn lex(path: &String) -> Result<Vec<(String, Pos)>, &'static str> {
    let mut prg: Vec<(String, Pos)> = vec![];


    if let Ok(lines) = read_lines(path) {
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let _line_len = ip.len();
                let words = ip.split(" ");
                for (j, word) in words.enumerate() {
                    let _word_len = word.len();
                    
                    prg.push((String::from(word), (i as u32, j as u32)))
                }
            }
        }
    }

    Ok(prg)
}

fn parse(prg: Result<Vec<(String, Pos)>, &'static str>) -> Result<Vec<Lexeme>, &'static str> {

    if let Err(e) = prg {
        return Err(e)
    }

    let mut parsed_prg: Vec<Lexeme> = vec![];
    
    for (token, pos) in prg.unwrap() {
        parsed_prg.push((match token.as_str() {
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
            "la" => Instructions::If,
            "pini" => Instructions::End,
            x if x.parse::<i32>().is_ok() => Instructions::Literal(Type::Int(x.parse::<i32>().unwrap())),
            x if x.parse::<f32>().is_ok() => Instructions::Literal(Type::Float(x.parse::<f32>().unwrap())),
            _ => Instructions::Null
        }, pos))
    }
    Ok(parsed_prg)
}

fn execute(ctx: &mut Runtime, prg: Vec<Lexeme>) {


    for (token, _pos) in prg {
        match token {
            Instructions::Print => {
                assert!(ctx.stack.len() >= 1, "Not enough arguments");

                let print_value = ctx.stack.pop().unwrap();
                println!("{}", print_value)
            },
            Instructions::Add => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

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
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

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
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

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
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

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
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

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
                assert!(ctx.stack.len() >= 1, "Not enough arguments");
                let b = ctx.stack.pop().unwrap();
                match b {
                    Type::Bool(x) => {
                        ctx.stack.push(Type::Bool(!x));
                    },
                    Type::Int(x) => {
                        ctx.stack.push(Type::Int(!x));
                    },
                    _ => panic!("{:?} does not support not operator", b)
                }
            },
            Instructions::And => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    Type::Bool(x) => {
                        let y = if let Type::Bool(y) = b { y } else { panic!("not a Bool")};
                        ctx.stack.push(Type::Bool(x && y));
                    },
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b { y } else { panic!("not a Int")};
                        ctx.stack.push(Type::Int(x & y));
                    }
                    _ => panic!("{:?} does not support and operator", a)
                }
            },
            Instructions::Or => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    Type::Bool(x) => {
                        let y = if let Type::Bool(y) = b { y } else { panic!("not a Bool")};
                        ctx.stack.push(Type::Bool(x || y));
                    },
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b { y } else { panic!("not a Int")};
                        ctx.stack.push(Type::Int(x | y));
                    }
                    _ => panic!("{:?} does not support or operator", a)
                }
            },
            Instructions::If => {
                assert!(ctx.stack.len() >= 1, "Not enough arguments");
                let b = ctx.stack.pop().unwrap();
                match b {
                    Type::Bool(_x) => {
                        continue;
                    },
                    _ => panic!("{:?} does not support or operator", b)
                }
            },
            Instructions::End => {continue;},

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
