use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Type {
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(usize) // index in str_heap
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Int(x) => write!(f, "{}", x),
            Type::Float(x) => write!(f, "{}", x),
            Type::Bool(x) => write!(f, "{}", x),
            Type::Str(x) => write!(f, "{}", x),
        }
    }
}


type Pos = (usize, usize, String);
type Lexeme = (Instructions, Pos);
type Ptr = usize;

macro_rules! printerr {
    ($msg:expr,$tok:expr) => {
        println!("ERROR: {} {}:{}:{}", $msg, $tok.2, $tok.0, $tok.1);
        std::process::exit(0);
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instructions {
    Literal(Type),

    Add,
    Sub,
    Mult,
    Div,

    Print,

    Not,
    And,
    Or,

    Eq,
    Lt,
    Gt,

    If(Option<Ptr>),
    Else(Option<Ptr>),
    End(Option<Ptr>),
    Do(Option<Ptr>),
    While,

    Dup,
    Drop,
    // Null
}

#[derive(Debug, PartialEq)]
struct Runtime {
    stack: Vec<Type>,
    str_heap: Vec<String>
}

impl Runtime {
    fn new() -> Self {
        Runtime {
            stack: vec![],
            str_heap: vec![]
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut ctx = Runtime::new();

    let lexed = lex(path);
    let mut parsed = parse(lexed, &mut ctx).unwrap();
    println!("{:?}", cross_refeance(&mut parsed).unwrap());
    let _ = execute(&mut ctx, cross_refeance(&mut parsed).unwrap());
}

fn lex(path: &String) -> Result<Vec<(String, Pos)>, &'static str> {
    let mut prg: Vec<(String, Pos)> = vec![];

    if let Ok(lines) = read_lines(path) {
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let words = ip.split(' ');
                let mut col = 1;
                for word in words {
                    if word.contains('#') {
                        break;
                    }

                    prg.push((String::from(word), (i + 1 , col, path.clone())));
                    col += word.len() + 1
                }
            }
        }
    }

    Ok(prg.iter()
    .filter(|x| x.0 != "" )
    .map(|x| x.clone())
    .collect::<Vec<(String, Pos)>>())
}

fn cross_refeance(prg: &mut Vec<Lexeme>) -> Result<Vec<Lexeme>, &'static str> {
    let mut stack: Vec<usize> = vec![];
    for i in 0..prg.len() {
        let token = prg[i].0.clone();
        match token {
            Instructions::If(_) => stack.push(i),
            Instructions::While => stack.push(i),
            Instructions::Else(_) => {
                let if_i = stack.pop().unwrap();
                if let Instructions::If(_) = prg[if_i].0 {
                    prg[if_i].0 = Instructions::If(Some(i));
                    stack.push(i)
                } else { printerr!("'ante-la' can only close 'la' blocks", prg[if_i].1); }
            }
            Instructions::End(_) => {
                let block_i = stack.pop().unwrap();

                if prg[block_i].0 == Instructions::If(None) {
                    prg[block_i].0 = Instructions::If(Some(i));
                }
                else if prg[block_i].0 == Instructions::Else(None) {
                    prg[block_i].0 = Instructions::Else(Some(i));
                }
                else if let Instructions::Do(Some(while_i)) = prg[block_i].0 {
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

// https://github.com/ttm/tokipona/blob/master/data/toki-pona_english.txt

fn parse(prg: Result<Vec<(String, Pos)>, &'static str>, ctx: &mut Runtime) -> Result<Vec<Lexeme>, &'static str> {
    if let Err(e) = prg {
        return Err(e);
    }

    let mut parsed_prg: Vec<Lexeme> = vec![];

    let unwraped_prg = prg.unwrap();
    let mut i = 0;
    let prg_len = unwraped_prg.len();
    while i < prg_len {
        let (token, pos) = unwraped_prg[i].clone();
//        println!("{}", token);
        parsed_prg.push((
                match token.as_str() {
                    "toki" => Instructions::Print,
                    "+" => Instructions::Add,
                    "-" => Instructions::Sub,
                    "*" => Instructions::Mult,
                    "/" => Instructions::Div,
                    "ike" => Instructions::Not,
                    "en" => Instructions::And,
                    "anu" => Instructions::Or,
                    "lon" => Instructions::Literal(Type::Bool(true)),
                    "la" => Instructions::If(None),
                    "ante-la" => Instructions::Else(None),
                    "pini" => Instructions::End(None),
                    "pali" => Instructions::Do(None),
                    "tenpo" => Instructions::While,
                    "sin" => Instructions::Dup,
                    "pakala" => Instructions::Drop,
                    "=" => Instructions::Eq,
                    ">" => Instructions::Lt,
                    "<" => Instructions::Gt,
                    x if x.parse::<i32>().is_ok() => {
                        Instructions::Literal(Type::Int(x.parse::<i32>().unwrap()))
                    }
                    x if x.parse::<f32>().is_ok() => {
                        Instructions::Literal(Type::Float(x.parse::<f32>().unwrap()))
                    }
                    x if x.chars().nth(0).unwrap() == '"' => {
                        let mut str = String::from("");
                        let mut j = i;
                        loop {
                            str.push_str(unwraped_prg[j].0.as_str());
                            str += " ";
                            if !(unwraped_prg[j].0.chars().nth_back(0) != Some('"')) { break;  }
                            j += 1;
                        }
                        i = j;
                        ctx.str_heap.push(String::from(str));
                        let i = ctx.str_heap.len() - 1;
                        Instructions::Literal(Type::Str(i))
                    },
                    _ => continue,
                },
            pos,));

        i += 1
    }

    Ok(parsed_prg)
}

fn execute(ctx: &mut Runtime, prg: Vec<Lexeme>) {
    let mut i = 0;
    while i < prg.len() {
        let (token, pos) = &prg[i];
        match token {
            Instructions::Print => {
                if ctx.stack.len() < 1 {
                    printerr!("'toki' requiers 1 argument on the top of the stack", pos);
                }

                let print_value = ctx.stack.pop().unwrap();

                if let Type::Str(x) = print_value {
                    println!("{}", ctx.str_heap[x])
                } else { println!("{}", print_value) }
            }
            Instructions::Add => {
                if ctx.stack.len() < 2 {
                    printerr!("'+' requiers 2 arguments on the top of the stack", pos);
                }

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
                if ctx.stack.len() < 2 {
                    printerr!("'-' requiers 2 arguments on the top of the stack", pos);
                }

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
                if ctx.stack.len() < 2 {
                    printerr!("'*' requiers 2 arguments on the top of the stack", pos);
                }

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
                if ctx.stack.len() < 2 {
                    printerr!("'/' requiers 2 arguments on the top of the stack", pos);
                }

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
                        ctx.stack.push(Type::Int(x / y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(Type::Float(x % y));
                        ctx.stack.push(Type::Float(x / y));
                    }
                    _ => panic!("{:?} does not support div operator", a),
                }
            }
            Instructions::Not => {
                if ctx.stack.len() < 1 {
                    printerr!("'ike' requiers 1 arguments on the top of the stack", pos);
                }
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
                if ctx.stack.len() < 2 {
                    printerr!("'en' requiers 2 arguments on the top of the stack", pos);
                }

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
                if ctx.stack.len() < 2 {
                    printerr!("'anu' requiers 2 arguments on the top of the stack", pos);
                }

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
            }
            Instructions::Eq => {
                if ctx.stack.len() < 2 {
                    printerr!("'=' requiers 2 arguments on the top of the stack", pos);
                }

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
                    }
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x == y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x == y));
                    }
                    _ => panic!("")
                }
            }
            Instructions::Lt => {
                if ctx.stack.len() < 2 {
                    printerr!("'>' requiers 2 arguments on the top of the stack", pos);
                }

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
                    }
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x < y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x < y));
                    }
                    _ => panic!("")
                }
            }
            Instructions::Gt => {
                if ctx.stack.len() < 2 {
                    printerr!("'<' requiers 2 arguments on the top of the stack", pos);
                }

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
                    }
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x > y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x > y));
                    }
                    _ => panic!("")
                }
            }
            Instructions::Dup => {
                if ctx.stack.len() < 1 {
                    printerr!("'sin' requiers 1 arguments on the top of the stack", pos);
                }

                let b = ctx.stack.pop().unwrap();
                ctx.stack.push(b);
                ctx.stack.push(b);
            }
            Instructions::Drop => {
                if ctx.stack.len() < 1 {
                    printerr!("'pakala' requiers 1 arguments on the top of the stack", pos);
                }

                let _ = ctx.stack.pop().unwrap();
            }
            Instructions::If(x) => {
                if ctx.stack.len() < 1 {
                    printerr!("'la' requiers 1 arguments on the top of the stack", pos);
                }
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
                if ctx.stack.len() < 1 {
                    printerr!("'pali' requiers 1 arguments on the top of the stack", pos);
                }
                let b = ctx.stack.pop().unwrap();
                match b {
                    Type::Bool(val) => {
                        if val {
                            i += 1;
                            continue;
                        } else if let Some(ptr) = x {
                            i = ptr + 1
                        }
                    }
                    _ => panic!("{:?} does not support do operator", b),
                }
            }
            Instructions::Else(x) => {
                if let Some(ptr) = x {
                    i = *ptr;
                }
            },
            Instructions::End(x) => {
                if let Some(ptr) = x {
                    i = *ptr;
                }
            }

            Instructions::Literal(literal) => ctx.stack.push(*literal),
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
