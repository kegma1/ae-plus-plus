use crate::{ops, Runtime};
use snailquote::unescape;
use std::io::{stdin, stdout, Write};

pub fn execute(ctx: &mut Runtime, prg: Vec<ops::Instruction>) -> Result<u8, (&'static str, ops::Pos)> {
    let mut i = 0;
    while i < prg.len() {
        let (token, pos) = &prg[i];
        match token {
            ops::Operator::Print => {
                if ctx.stack.len() < 1 {
                    return Err((
                        "'toki' requires 1 argument on the top of the stack",
                        pos.clone(),
                    ));
                }

                let print_value = ctx.stack.pop().unwrap();

                if let ops::Type::Str(x) = print_value {
                    print!("{}", ctx.str_heap[x])
                } else {
                    println!("{}", print_value)
                }
            }
            ops::Operator::Input => {
                let print_value = ctx.stack.pop();

                if let Some(ops::Type::Str(x)) = print_value {
                    print!("{}", ctx.str_heap[x])
                }

                let mut s = String::new();

                let _ = stdout().flush();
                stdin()
                    .read_line(&mut s)
                    .expect("Did not enter a correct string");
                if let Some('\n') = s.chars().next_back() {
                    s.pop();
                }
                if let Some('\r') = s.chars().next_back() {
                    s.pop();
                }

                let unescaped_x = unescape(&s).unwrap();
                ctx.str_heap.push(unescaped_x);
                let i = ctx.str_heap.len() - 1;
                ctx.stack.push(ops::Type::Str(i));
            }
            ops::Operator::Add => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'+' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not an Int")
                        };
                        ctx.stack.push(ops::Type::Int(x + y));
                    }
                    ops::Type::Float(x) => {
                        let y = if let ops::Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(ops::Type::Float(x + y));
                    }
                    ops::Type::Str(x) => {
                        let y = if let ops::Type::Str(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };

                        let x_val = ctx.str_heap[x].clone();
                        let y_val = ctx.str_heap[y].clone();
                        ctx.str_heap.push(x_val + &y_val);
                        ctx.stack.push(ops::Type::Str(ctx.str_heap.len() - 1));
                    }
                    _ => panic!("{:?} does not support add ops::Operator", a),
                }
            }
            ops::Operator::Sub => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'-' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not an Int")
                        };
                        ctx.stack.push(ops::Type::Int(x - y));
                    }
                    ops::Type::Float(x) => {
                        let y = if let ops::Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(ops::Type::Float(x - y));
                    }
                    _ => panic!("{:?} does not support sub ops::Operator", a),
                }
            }
            ops::Operator::Mult => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'*' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not an Int")
                        };
                        ctx.stack.push(ops::Type::Int(x * y));
                    }
                    ops::Type::Float(x) => {
                        let y = if let ops::Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(ops::Type::Float(x * y));
                    }
                    _ => panic!("{:?} does not support mult ops::Operator", a),
                }
            }
            ops::Operator::Div => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'/' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match a {
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not an Int")
                        };
                        ctx.stack.push(ops::Type::Int(x % y));
                        ctx.stack.push(ops::Type::Int(x / y));
                    }
                    ops::Type::Float(x) => {
                        let y = if let ops::Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(ops::Type::Float(x % y));
                        ctx.stack.push(ops::Type::Float(x / y));
                    }
                    _ => panic!("{:?} does not support div ops::Operator", a),
                }
            }
            ops::Operator::Not => {
                if ctx.stack.len() < 1 {
                    return Err((
                        "'ike' requires 1 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }
                let b = ctx.stack.pop().unwrap();
                match b {
                    ops::Type::Bool(x) => {
                        ctx.stack.push(ops::Type::Bool(!x));
                    }
                    ops::Type::Int(x) => {
                        ctx.stack.push(ops::Type::Int(!x));
                    }
                    _ => panic!("{:?} does not support not ops::Operator", b),
                }
            }
            ops::Operator::And => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'en' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    ops::Type::Bool(x) => {
                        let y = if let ops::Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(ops::Type::Bool(x && y));
                    }
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Int(x & y));
                    }
                    _ => panic!("{:?} does not support and ops::Operator", a),
                }
            }
            ops::Operator::Or => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'anu' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    ops::Type::Bool(x) => {
                        let y = if let ops::Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(ops::Type::Bool(x || y));
                    }
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Int(x | y));
                    }
                    _ => panic!("{:?} does not support or ops::Operator", a),
                }
            }
            ops::Operator::Eq => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'=' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    ops::Type::Bool(x) => {
                        let y = if let ops::Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(ops::Type::Bool(x == y));
                    }
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Bool(x == y));
                    }
                    ops::Type::Float(x) => {
                        let y = if let ops::Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Bool(x == y));
                    }
                    ops::Type::Str(str_x) => {
                        let x = &ctx.str_heap[str_x];
                        let y = if let ops::Type::Str(y) = b {
                            &ctx.str_heap[y]
                        } else {
                            panic!("not a Str")
                        };
                        ctx.stack.push(ops::Type::Bool(x == y));
                    } //_ => panic!("")
                }
            }
            ops::Operator::Lt => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'>' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    ops::Type::Bool(x) => {
                        let y = if let ops::Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(ops::Type::Bool(x < y));
                    }
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Bool(x < y));
                    }
                    ops::Type::Float(x) => {
                        let y = if let ops::Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Bool(x < y));
                    }
                    _ => panic!(""),
                }
            }
            ops::Operator::Le => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'>=' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    ops::Type::Bool(x) => {
                        let y = if let ops::Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(ops::Type::Bool(x <= y));
                    }
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Bool(x <= y));
                    }
                    ops::Type::Float(x) => {
                        let y = if let ops::Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Bool(x <= y));
                    }
                    _ => panic!(""),
                }
            }
            ops::Operator::Gt => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'<' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    ops::Type::Bool(x) => {
                        let y = if let ops::Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(ops::Type::Bool(x > y));
                    }
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Bool(x > y));
                    }
                    ops::Type::Float(x) => {
                        let y = if let ops::Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Bool(x > y));
                    }
                    _ => panic!(""),
                }
            }
            ops::Operator::Ge => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'<=' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                match a {
                    ops::Type::Bool(x) => {
                        let y = if let ops::Type::Bool(y) = b {
                            y
                        } else {
                            panic!("not a Bool")
                        };
                        ctx.stack.push(ops::Type::Bool(x >= y));
                    }
                    ops::Type::Int(x) => {
                        let y = if let ops::Type::Int(y) = b {
                            y
                        } else {
                            panic!("not a Int")
                        };
                        ctx.stack.push(ops::Type::Bool(x >= y));
                    }
                    ops::Type::Float(x) => {
                        let y = if let ops::Type::Float(y) = b {
                            y
                        } else {
                            panic!("not a Float")
                        };
                        ctx.stack.push(ops::Type::Bool(x >= y));
                    }
                    _ => panic!(""),
                }
            }
            ops::Operator::Dup => {
                if ctx.stack.len() < 1 {
                    return Err((
                        "'sin' requires 1 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                ctx.stack.push(b);
                ctx.stack.push(b);
            }
            ops::Operator::Drop => {
                if ctx.stack.len() < 1 {
                    return Err((
                        "'pakala' requires 1 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let _ = ctx.stack.pop().unwrap();
            }
            ops::Operator::Swap => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'ensu' requires 2 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                ctx.stack.push(b);
                ctx.stack.push(a);
            }
            ops::Operator::If(x) => {
                if ctx.stack.len() < 1 {
                    return Err((
                        "'la' requires 1 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }
                let b = ctx.stack.pop().unwrap();
                match b {
                    ops::Type::Bool(val) => {
                        if val {
                            i += 1;
                            continue;
                        } else {
                            i = x.expect("no matching pini") - 1;
                        }
                    }
                    _ => panic!("{:?} does not support if ops::Operator", b),
                }
            }
            ops::Operator::While => (),
            ops::Operator::Do(x) => {
                if ctx.stack.len() < 1 {
                    return Err((
                        "'pali' requires 1 arguments on the top of the stack",
                        pos.clone(),
                    ));
                }
                let b = ctx.stack.pop().unwrap();
                match b {
                    ops::Type::Bool(val) => {
                        if val {
                            i += 1;
                            continue;
                        } else if let Some(ptr) = x {
                            i = ptr + 1
                        }
                    }
                    _ => panic!("{:?} does not support do ops::Operator", b),
                }
            }
            ops::Operator::Else(x) => {
                if let Some(ptr) = x {
                    i = *ptr;
                }
            }
            ops::Operator::End(x) => {
                if let Some(ptr) = x {
                    i = *ptr;
                }
            }

            ops::Operator::Literal(literal) => ctx.stack.push(*literal),
            // ops::Operator::Null => {continue;}
        }
        //println!("{:?}", token);
        i += 1;
    }
    Ok(0)
}
