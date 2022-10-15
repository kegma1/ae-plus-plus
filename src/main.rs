use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
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
            Type::Bool(x) => write!(f, "{}", x),
        }
    }
}

type Pos = (u32, u32);
type Lexeme = (Instructions, Pos);
type Ptr = usize;

#[derive(Debug, PartialEq, Clone, Copy)]
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

    Eq,
    Lt,
    Gt,

    If(Option<Ptr>),
    End(Option<Ptr>),
    Do(Option<Ptr>),
    While,

    Dup,
    // Null
}

#[derive(Debug, PartialEq)]
struct Runtime {
    stack: Vec<Type>,
}

impl Runtime {
    fn new() -> Self {
        Runtime { stack: vec![] }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut ctx = Runtime::new();

    let lexed = lex(path);
    let mut parsed = parse(lexed).unwrap();
    //println!("{:?}", cross_refeance(&mut parsed).unwrap());
    let _ = execute(&mut ctx, cross_refeance(&mut parsed).unwrap());
}

fn lex(path: &String) -> Result<Vec<(String, Pos)>, &'static str> {
    let mut prg: Vec<(String, Pos)> = vec![];

    if let Ok(lines) = read_lines(path) {
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let words = ip.split(" ");
                for (j, word) in words.enumerate() {
                    if word.contains("#") {
                        break;
                    }

                    prg.push((String::from(word), (i as u32, j as u32)))
                }
            }
        }
    }
    Ok(prg)
}

fn cross_refeance(prg: &mut Vec<Lexeme>) -> Result<Vec<Lexeme>, &'static str> {
    let mut stack: Vec<usize> = vec![];
    for i in 0..prg.len() {
        let token = prg[i].0.clone();
        match token {
            Instructions::If(_) => stack.push(i),
            Instructions::While => stack.push(i),
            Instructions::End(_) => {
                let block_i = stack.pop().unwrap();
                if prg[block_i].0 == Instructions::If(None) {
                    prg[block_i].0 = Instructions::If(Some(i));
                } else if let Instructions::Do(Some(while_i)) = prg[block_i].0 {
                    prg[i].0 = Instructions::End(Some(while_i));
                    prg[block_i].0 = Instructions::Do(Some(i));
                }
            }
            Instructions::Do(_) => {
                let while_i = stack.pop().unwrap();
                prg[i].0 = Instructions::Do(Some(while_i));
                stack.push(i)
            }
            _ => (),
        }
    }

    Ok(prg.clone())
}

fn parse(prg: Result<Vec<(String, Pos)>, &'static str>) -> Result<Vec<Lexeme>, &'static str> {
    if let Err(e) = prg {
        return Err(e);
    }

    let mut parsed_prg: Vec<Lexeme> = vec![];

    for (token, pos) in prg.unwrap() {
        parsed_prg.push((
            match token.as_str() {
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
                "la" => Instructions::If(None),
                "pini" => Instructions::End(None),
                "pali" => Instructions::Do(None),
                "tenpo" => Instructions::While,
                "sin" => Instructions::Dup,
                "sama" => Instructions::Eq,
                "lili" => Instructions::Lt,
                "suli" => Instructions::Gt,
                x if x.parse::<i32>().is_ok() => {
                    Instructions::Literal(Type::Int(x.parse::<i32>().unwrap()))
                }
                x if x.parse::<f32>().is_ok() => {
                    Instructions::Literal(Type::Float(x.parse::<f32>().unwrap()))
                }
                _ => continue,
            },
            pos,
        ))
    }

    Ok(parsed_prg)
}

fn execute(ctx: &mut Runtime, prg: Vec<Lexeme>) {
    let mut i = 0;
    while i < prg.len() {
        let (token, _pos) = prg[i];
        match token {
            Instructions::Print => {
                assert!(ctx.stack.len() >= 1, "Not enough arguments");

                let print_value = ctx.stack.pop().unwrap();
                println!("{}", print_value)
            }
            Instructions::Add => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not an Int")
                        };
                        ctx.stack.push(Type::Int(x + y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(Type::Float(x + y));
                    }
                    _ => panic!("{:?} does not support add operator", a),
                }
            }
            Instructions::Sub => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not an Int")
                        };
                        ctx.stack.push(Type::Int(x - y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(Type::Float(x - y));
                    }
                    _ => panic!("{:?} does not support sub operator", a),
                }
            }
            Instructions::Mult => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not an Int")
                        };
                        ctx.stack.push(Type::Int(x * y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(Type::Float(x * y));
                    }
                    _ => panic!("{:?} does not support mult operator", a),
                }
            }
            Instructions::Div => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not an Int")
                        };
                        ctx.stack.push(Type::Int(x / y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(Type::Float(x / y));
                    }
                    _ => panic!("{:?} does not support div operator", a),
                }
            }
            Instructions::Mod => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not an Int")
                        };
                        ctx.stack.push(Type::Int(x % y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(Type::Float(x % y));
                    }
                    _ => panic!("{:?} does not support mod operator", a),
                }
            }
            Instructions::Not => {
                assert!(ctx.stack.len() >= 1, "Not enough arguments");
                let b = ctx.stack.pop().unwrap();
                match b {
                    Type::Bool(x) => {
                        ctx.stack.push(Type::Bool(!x));
                    }
                    Type::Int(x) => {
                        ctx.stack.push(Type::Int(!x));
                    }
                    _ => panic!("{:?} does not support not operator", b),
                }
            }
            Instructions::And => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    Type::Bool(x) => {
                        let y = if let Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(Type::Bool(x && y));
                    }
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Int(x & y));
                    }
                    _ => panic!("{:?} does not support and operator", a),
                }
            }
            Instructions::Or => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    Type::Bool(x) => {
                        let y = if let Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(Type::Bool(x || y));
                    }
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Int(x | y));
                    }
                    _ => panic!("{:?} does not support or operator", a),
                }
            },
            Instructions::Eq => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    Type::Bool(x) => {
                        let y = if let Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(Type::Bool(x == y));
                    },
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x == y));
                    },
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x == y));
                    },
                }
            },
            Instructions::Lt => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    Type::Bool(x) => {
                        let y = if let Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(Type::Bool(x < y));
                    },
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x < y));
                    },
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x < y));
                    },
                }
            },
            Instructions::Gt => {
                assert!(ctx.stack.len() >= 2, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    Type::Bool(x) => {
                        let y = if let Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(Type::Bool(x > y));
                    },
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x > y));
                    },
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x > y));
                    },
                }
            }
            Instructions::Dup => {
                assert!(ctx.stack.len() >= 1, "Not enough arguments");

                let b = ctx.stack.pop().unwrap();
                ctx.stack.push(b);
                ctx.stack.push(b);
            }
            Instructions::If(x) => {
                assert!(ctx.stack.len() >= 1, "Not enough arguments");
                let b = ctx.stack.pop().unwrap();
                match b {
                    Type::Bool(val) => {
                        if val {
                            i += 1;
                            continue;
                        } else {
                            i = x.expect("no matching pini") - 1;
                        }
                    }
                    _ => panic!("{:?} does not support if operator", b),
                }
            }
            Instructions::While => (),
            Instructions::Do(x) => {
                assert!(ctx.stack.len() >= 1, "Not enough arguments");
                let b = ctx.stack.pop().unwrap();
                match b {
                    Type::Bool(val) => {
                        if val {
                            i += 1;
                            continue;
                        } else {
                            if let Some(ptr) = x {
                                i = ptr + 1
                            }
                        }
                    }
                    _ => panic!("{:?} does not support do operator", b),
                }
            }
            Instructions::End(x) => {
                if let Some(ptr) = x {
                    i = ptr;
                }
            }

            Instructions::Literal(literal) => ctx.stack.push(literal),
            // Instructions::Null => {continue;}
        }
        //println!("{:?}", token);
        i += 1;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
