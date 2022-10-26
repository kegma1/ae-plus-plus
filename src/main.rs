use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use snailquote::unescape;
use std::io::{stdin,stdout,Write};

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
type Lexeme = (Instruction, Pos);
type Ptr = usize;

macro_rules! printerr {
    ($msg:expr,$tok:expr) => {
        println!("ERROR: {} {}:{}:{}", $msg, $tok.2, $tok.0, $tok.1);
        std::process::exit(0);
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Literal(Type),

    Add,
    Sub,
    Mult,
    Div,

    Print,
    Input,

    Not,
    And,
    Or,

    Eq,
    Lt,
    Le,
    Gt,
    Ge,

    If(Option<Ptr>),
    Else(Option<Ptr>),
    End(Option<Ptr>),
    Do(Option<Ptr>),
    While,

    Dup,
    Drop,
    Swap,
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
//    println!("{:?}", cross_reference(&mut parsed).unwrap());
    let _ = execute(&mut ctx, cross_reference(&mut parsed).unwrap());
}

enum Mode {
    Normal,
    String
}

fn lex(path: &String) -> Result<Vec<(String, Pos)>, &'static str> {
    let mut prg: Vec<(String, Pos)> = vec![];

    if let Ok(lines) = read_lines(path) {
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let mut col = 1;
                let mut word = String::from("");
                let mut mode = Mode::Normal;
                for char in ip.chars() {
                    match (char, &mode) {
                        ('#', Mode::Normal) => break,
                        ('"', Mode::Normal) => { word.push(char); mode = Mode::String },
                        ('"', Mode::String) => { word.push(char); mode = Mode::Normal },
                        (' ', Mode::Normal) => {
                            if !word.is_empty() {
                                prg.push((word.clone(), (i + 1 , col, path.clone())));
                                col += word.len() + 1;
                                word.clear();
                            }
                        },
                        (_, _)=> word.push(char)
                    }
                }
                prg.push((word.clone(), (i + 1 , col, path.clone())));
            }
        }
    }

    Ok(prg)
}

fn cross_reference(prg: &mut Vec<Lexeme>) -> Result<Vec<Lexeme>, &'static str> {
    let mut stack: Vec<usize> = vec![];
    for i in 0..prg.len() {
        let token = prg[i].0.clone();
        match token {
            Instruction::If(_) => stack.push(i),
            Instruction::While => stack.push(i),
            Instruction::Else(_) => {
                let if_i = stack.pop().unwrap();
                if let Instruction::If(_) = prg[if_i].0 {
                    prg[if_i].0 = Instruction::If(Some(i + 1));
                    stack.push(i)
                } else { printerr!("'ante-la' can only close 'la' blocks", prg[if_i].1); }
            }
            Instruction::End(_) => {
                let block_i = stack.pop().unwrap();

                if prg[block_i].0 == Instruction::If(None) {
                    prg[block_i].0 = Instruction::If(Some(i));
                }
                else if prg[block_i].0 == Instruction::Else(None) {
                    prg[block_i].0 = Instruction::Else(Some(i));
                }
                else if let Instruction::Do(Some(while_i)) = prg[block_i].0 {
                    prg[i].0 = Instruction::End(Some(while_i));
                    prg[block_i].0 = Instruction::Do(Some(i));
                }
            }
            Instruction::Do(_) => {
                let while_i = stack.pop().unwrap();
                prg[i].0 = Instruction::Do(Some(while_i));
                stack.push(i)
            }
            _ => (),
        }
    }

    Ok(prg.clone())
}

// https://github.com/ttm/tokipona/blob/master/data/toki-pona_english.txt

fn parse(prg: Result<Vec<(String, Pos)>, &'static str>, ctx: &mut Runtime) -> Result<Vec<Lexeme>, &'static str> {
    
    // println!("{:?}",prg);
    if let Err(e) = prg {
        return Err(e);
    }

    let mut parsed_prg: Vec<Lexeme> = vec![];

    let unwraped_prg = prg.unwrap();
    let mut i = 0;
    let prg_len = unwraped_prg.len();
    while i < prg_len {
        let (token, pos) = unwraped_prg[i].clone();
        // println!("{}", i);
        parsed_prg.push((
                match token.as_str() {
                    "toki" => Instruction::Print,
                    "o!" => Instruction::Input,
                    "+" => Instruction::Add,
                    "-" => Instruction::Sub,
                    "*" => Instruction::Mult,
                    "/" => Instruction::Div,
                    "ike" => Instruction::Not,
                    "en" => Instruction::And,
                    "anu" => Instruction::Or,
                    "lon" => Instruction::Literal(Type::Bool(true)),
                    "la" => Instruction::If(None),
                    "ante-la" => Instruction::Else(None),
                    "pini" => Instruction::End(None),
                    "pali" => Instruction::Do(None),
                    "tenpo" => Instruction::While,
                    "sin" => Instruction::Dup,
                    "pakala" => Instruction::Drop,
                    "esun" => Instruction::Swap,
                    "=" => Instruction::Eq,
                    ">" => Instruction::Lt,
                    ">=" => Instruction::Le,
                    "<" => Instruction::Gt,
                    "<=" => Instruction::Ge,
                    x if x.parse::<i32>().is_ok() => {
                        Instruction::Literal(Type::Int(x.parse::<i32>().unwrap()))
                    }
                    x if x.parse::<f32>().is_ok() => {
                        Instruction::Literal(Type::Float(x.parse::<f32>().unwrap()))
                    }
                    x if x.chars().nth(0) == Some('"') => {
                        let unescaped_x = unescape(x).unwrap();
                        ctx.str_heap.push(unescaped_x);
                        let i = ctx.str_heap.len() - 1;
                        Instruction::Literal(Type::Str(i))
                    },
                    _ => {i += 1; continue}
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
            Instruction::Print => {
                if ctx.stack.len() < 1 {
                    printerr!("'toki' requires 1 argument on the top of the stack", pos);
                }

                let print_value = ctx.stack.pop().unwrap();

                if let Type::Str(x) = print_value {
                    print!("{}", ctx.str_heap[x])
                } else { println!("{}", print_value) }
            }
            Instruction::Input => {
                let print_value = ctx.stack.pop();

                if let Some(Type::Str(x)) = print_value {
                    print!("{}", ctx.str_heap[x])
                }

                let mut s = String::new();

                let _=stdout().flush();
                stdin().read_line(&mut s).expect("Did not enter a correct string");
                if let Some('\n')=s.chars().next_back() {
                    s.pop();
                }
                if let Some('\r')=s.chars().next_back() {
                    s.pop();
                }

                let unescaped_x = unescape(&s).unwrap();
                ctx.str_heap.push(unescaped_x);
                let i = ctx.str_heap.len() - 1;
                ctx.stack.push(Type::Str(i));
            }
            Instruction::Add => {
                if ctx.stack.len() < 2 {
                    printerr!("'+' requires 2 arguments on the top of the stack", pos);
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
                    Type::Str(x) => {
                        let y = if let Type::Str(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };

                        let x_val = ctx.str_heap[x].clone();
                        let y_val = ctx.str_heap[y].clone();
                        ctx.str_heap.push(x_val + &y_val);
                        ctx.stack.push(Type::Str(ctx.str_heap.len() - 1));
                    },
                    _ => panic!("{:?} does not support add operator", a),
                }
            }
            Instruction::Sub => {
                if ctx.stack.len() < 2 {
                    printerr!("'-' requires 2 arguments on the top of the stack", pos);
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
            Instruction::Mult => {
                if ctx.stack.len() < 2 {
                    printerr!("'*' requires 2 arguments on the top of the stack", pos);
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
            Instruction::Div => {
                if ctx.stack.len() < 2 {
                    printerr!("'/' requires 2 arguments on the top of the stack", pos);
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
            Instruction::Not => {
                if ctx.stack.len() < 1 {
                    printerr!("'ike' requires 1 arguments on the top of the stack", pos);
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
            Instruction::And => {
                if ctx.stack.len() < 2 {
                    printerr!("'en' requires 2 arguments on the top of the stack", pos);
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
            Instruction::Or => {
                if ctx.stack.len() < 2 {
                    printerr!("'anu' requires 2 arguments on the top of the stack", pos);
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
            Instruction::Eq => {
                if ctx.stack.len() < 2 {
                    printerr!("'=' requires 2 arguments on the top of the stack", pos);
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
                    Type::Str(str_x) => {
                        let x = &ctx.str_heap[str_x];
                        let y = if let Type::Str(y) = b {
                            &ctx.str_heap[y]
                        } else {
                            panic!("not a Str")
                        };
                        ctx.stack.push(Type::Bool(x == y));
                    }
                    //_ => panic!("")
                }
            }
            Instruction::Lt => {
                if ctx.stack.len() < 2 {
                    printerr!("'>' requires 2 arguments on the top of the stack", pos);
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
            Instruction::Le => {
                if ctx.stack.len() < 2 {
                    printerr!("'>=' requires 2 arguments on the top of the stack", pos);
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
                        ctx.stack.push(Type::Bool(x <= y));
                    }
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x <= y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x <= y));
                    }
                    _ => panic!("")
                }
            }
            Instruction::Gt => {
                if ctx.stack.len() < 2 {
                    printerr!("'<' requires 2 arguments on the top of the stack", pos);
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
            Instruction::Ge => {
                if ctx.stack.len() < 2 {
                    printerr!("'<=' requires 2 arguments on the top of the stack", pos);
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
                        ctx.stack.push(Type::Bool(x >= y));
                    }
                    Type::Int(x) => {
                        let y = if let Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(Type::Bool(x >= y));
                    }
                    Type::Float(x) => {
                        let y = if let Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(Type::Bool(x >= y));
                    }
                    _ => panic!("")
                }
            }
            Instruction::Dup => {
                if ctx.stack.len() < 1 {
                    printerr!("'sin' requires 1 arguments on the top of the stack", pos);
                }

                let b = ctx.stack.pop().unwrap();
                ctx.stack.push(b);
                ctx.stack.push(b);
            }
            Instruction::Drop => {
                if ctx.stack.len() < 1 {
                    printerr!("'pakala' requires 1 arguments on the top of the stack", pos);
                }

                let _ = ctx.stack.pop().unwrap();
            }
            Instruction::Swap => {
                if ctx.stack.len() < 2 {
                    printerr!("'ensu' requires 2 arguments on the top of the stack", pos);
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                ctx.stack.push(b);
                ctx.stack.push(a);
            }
            Instruction::If(x) => {
                if ctx.stack.len() < 1 {
                    printerr!("'la' requires 1 arguments on the top of the stack", pos);
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
            Instruction::While => (),
            Instruction::Do(x) => {
                if ctx.stack.len() < 1 {
                    printerr!("'pali' requires 1 arguments on the top of the stack", pos);
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
            Instruction::Else(x) => {
                if let Some(ptr) = x {
                    i = *ptr;
                }
            },
            Instruction::End(x) => {
                if let Some(ptr) = x {
                    i = *ptr;
                }
            }

            Instruction::Literal(literal) => ctx.stack.push(*literal),
            // Instruction::Null => {continue;}
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
