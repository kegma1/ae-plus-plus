use crate::{ops, Runtime};
use snailquote::unescape;
use std::io::{stdin, stdout, Write};



pub fn execute(ctx: &mut Runtime, prg: & Vec<ops::Instruction>) -> Result<u8, (&'static str, ops::Pos)> {
    let mut i = 0;
    while i < prg.len() {
        let token = &prg[i];

        match token.op {
            ops::Operator::Literal => ctx.stack.push(token.val.unwrap()),
            ops::Operator::Add => {
                if ctx.stack.len() < 2 {
                    return Err(("'+' operator requiers atleast 2 arguments of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.stack.push(ops::Value::Int(x + y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => ctx.stack.push(ops::Value::Float(x + y)),
                    (ops::Value::Str(x), ops::Value::Str(y)) => {
                        let mut new_str: String = ctx.str_heap[x].clone();
                        new_str.push_str(ctx.str_heap[y].as_str());

                        ctx.str_heap.push(new_str);
                        ctx.stack.push(ops::Value::Str(ctx.str_heap.len() - 1 as usize))
                        
                    },
                    (_, _) => {
                        let err_s: String = format!("'{} + {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::Sub => {
                if ctx.stack.len() < 2 {
                    return Err(("'-' operator requiers atleast 2 arguments of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.stack.push(ops::Value::Int(x - y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => ctx.stack.push(ops::Value::Float(x - y)),
                    (_, _) => {
                        let err_s: String = format!("'{} - {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::Mult => {
                if ctx.stack.len() < 2 {
                    return Err(("'*' operator requiers atleast 2 arguments of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.stack.push(ops::Value::Int(x * y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => ctx.stack.push(ops::Value::Float(x * y)),
                    (_, _) => {
                        let err_s: String = format!("'{} * {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::Div => {
                if ctx.stack.len() < 2 {
                    return Err(("'/' operator requiers atleast 2 arguments of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => {ctx.stack.push(ops::Value::Int(x % y)); ctx.stack.push(ops::Value::Int(x / y))},
                    (ops::Value::Float(x), ops::Value::Float(y)) => {ctx.stack.push(ops::Value::Float(x % y)); ctx.stack.push(ops::Value::Float(x / y))},
                    (_, _) => {
                        let err_s: String = format!("'{} / {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::Print => {
                if ctx.stack.len() < 1 {
                    return Err(("'toki' operator requiers atleast 1 argument", token.pos.clone()));
                }

                let print_val = ctx.stack.pop().unwrap();
                let _ = stdout().flush();
                match print_val  {
                    ops::Value::Int(x) => print!("{}\n", x),
                    ops::Value::Float(x) => print!("{}\n", x),
                    ops::Value::Bool(x) => print!("{}\n", x),
                    ops::Value::Str(x) => print!("{}\n", ctx.str_heap[x]),
                    ops::Value::TypeLiteral(_) => todo!("print for TypeLiter is not implemented"),
                }
            },
            ops::Operator::Input => {
                let print_value = ctx.stack.pop();

                if let Some(ops::Value::Str(x)) = print_value {
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
                ctx.stack.push(ops::Value::Str(i));
            },
            ops::Operator::Not => {
                if ctx.stack.len() < 1 {
                    return Err(("'ike' operator requiers atleast 1 argument", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();

                match b {
                    ops::Value::Bool(x) => ctx.stack.push(ops::Value::Bool(!x)),
                    _ => {
                        let err_s: String = format!("'ike {}' is not supported", b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::And => {
                if ctx.stack.len() < 2 {
                    return Err(("'en' operator requiers atleast 2 argument of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.stack.push(ops::Value::Bool(x && y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.stack.push(ops::Value::Int(x & y)),
                    (_, _) => {
                        let err_s: String = format!("'{} en {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::Or => {
                if ctx.stack.len() < 2 {
                    return Err(("'anu' operator requiers atleast 2 argument of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.stack.push(ops::Value::Bool(x || y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.stack.push(ops::Value::Int(x | y)),
                    (_, _) => {
                        let err_s: String = format!("'{} anu {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::Eq => {
                if ctx.stack.len() < 2 {
                    return Err(("'=' operator requiers atleast 2 argument of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.stack.push(ops::Value::Bool(x == y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.stack.push(ops::Value::Bool(x == y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => ctx.stack.push(ops::Value::Bool(x == y)),
                    (ops::Value::Str(x), ops::Value::Str(y)) => ctx.stack.push(ops::Value::Bool(ctx.str_heap[x] == ctx.str_heap[y])),
                    (_, _) => {
                        let err_s: String = format!("'{} = {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::Lt => {
                if ctx.stack.len() < 2 {
                    return Err(("'<' operator requiers atleast 2 argument of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.stack.push(ops::Value::Bool(x < y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.stack.push(ops::Value::Bool(x < y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => ctx.stack.push(ops::Value::Bool(x < y)),
                    (_, _) => {
                        let err_s: String = format!("'{} < {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::Le => {
                if ctx.stack.len() < 2 {
                    return Err(("'<=' operator requiers atleast 2 argument of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.stack.push(ops::Value::Bool(x <= y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.stack.push(ops::Value::Bool(x <= y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => ctx.stack.push(ops::Value::Bool(x <= y)),
                    (_, _) => {
                        let err_s: String = format!("'{} <= {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::Gt => {
                if ctx.stack.len() < 2 {
                    return Err(("'>' operator requiers atleast 2 argument of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.stack.push(ops::Value::Bool(x > y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.stack.push(ops::Value::Bool(x > y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => ctx.stack.push(ops::Value::Bool(x > y)),
                    (_, _) => {
                        let err_s: String = format!("'{} > {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::Ge => {
                if ctx.stack.len() < 2 {
                    return Err(("'>=' operator requiers atleast 2 argument of the same type", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.stack.push(ops::Value::Bool(x >= y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.stack.push(ops::Value::Bool(x >= y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => ctx.stack.push(ops::Value::Bool(x >= y)),
                    (_, _) => {
                        let err_s: String = format!("'{} >= {}' is not supported", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
            ops::Operator::If => {
                if ctx.stack.len() < 1 {
                    return Err(("'la' operator requiers atleast 1 argument", token.pos.clone()));
                }

                let con = ctx.stack.pop().unwrap();

                if let ops::Value::Bool(x) = con {
                    if x {
                        i += 1;
                        continue;
                    } else {
                        i = token.arg.unwrap();
                    }
                } else {
                    let err_s: String = format!("'ike {}' is not supported. 'ike' only takes Bool", con).to_owned();
                    return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                }
            },
            ops::Operator::Else | ops::Operator::End => {
                if let Some(ptr) = token.arg {
                    i = ptr;
                }
            },
            ops::Operator::Do => {
                if ctx.stack.len() < 1 {
                    return Err(("'pali' operator requiers atleast 1 argument", token.pos.clone()));
                }

                let con = ctx.stack.pop().unwrap();

                if let ops::Value::Bool(x) = con {
                    if x {
                        i += 1;
                        continue;
                    } else {
                        i = token.arg.unwrap();
                    }
                } else {
                    let err_s: String = format!("'pali {}' is not supported. 'pali' only takes Bool", con).to_owned();
                    return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                }
            },
            ops::Operator::While => (),
            ops::Operator::Dup => {
                if ctx.stack.len() < 1 {
                    return Err(("'sin' operator requiers atleast 1 argument", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();

                ctx.stack.push(b);
                ctx.stack.push(b);
            },
            ops::Operator::Drop => {
                if ctx.stack.len() < 1 {
                    return Err(("'sin' operator requiers atleast 1 argument", token.pos.clone()));
                }

                let _ = ctx.stack.pop().unwrap();
            },
            ops::Operator::Swap => {
                if ctx.stack.len() < 2 {
                    return Err(("'pakala' operator requiers atleast 2 arguments", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                ctx.stack.push(b);
                ctx.stack.push(a);
            },
            ops::Operator::Over => {
                if ctx.stack.len() < 2 {
                    return Err(("'sewi' operator requiers atleast 2 arguments", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();

                ctx.stack.push(a);
                ctx.stack.push(b);
                ctx.stack.push(a);
            },
            ops::Operator::Rot => {
                if ctx.stack.len() < 3 {
                    return Err(("'sike' operator requiers atleast 2 arguments", token.pos.clone()));
                }

                let b = ctx.stack.pop().unwrap();
                let a = ctx.stack.pop().unwrap();
                let c = ctx.stack.pop().unwrap();

                ctx.stack.push(b);
                ctx.stack.push(c);
                ctx.stack.push(a);
            },
            ops::Operator::Cast => {
                if ctx.stack.len() < 2 {
                    return Err(("'...' operator requiers atleast 2 arguments", token.pos.clone()));
                }

                let typ = ctx.stack.pop().unwrap();
                let b = ctx.stack.pop().unwrap();

                match (typ, b) {
                    (ops::Value::TypeLiteral(ops::TypeLiteral::Int), _) => {
                        match b {
                            ops::Value::Float(x) => ctx.stack.push(ops::Value::Int(x as i32)),
                            ops::Value::Bool(x) => if x { ctx.stack.push(ops::Value::Int(1i32))} else {ctx.stack.push(ops::Value::Int(0i32))},
                            ops::Value::Str(x) => {
                                if let Ok(new_x) = ctx.str_heap[x].parse::<i32>() {
                                    ctx.stack.push(ops::Value::Int(new_x));
                                } else {
                                    return Err(("Failed to cast to Int", token.pos.clone()));
                                }
                            }
                            _ => {
                                let err_s: String = format!("Can't cast {} to {}", b, typ).to_owned();
                                return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                            }
                        }
                    },
                    (ops::Value::TypeLiteral(ops::TypeLiteral::Float), _) => {
                        match b {
                            ops::Value::Int(x) => ctx.stack.push(ops::Value::Float(x as f32)),
                            ops::Value::Bool(x) => if x { ctx.stack.push(ops::Value::Float(1.))} else {ctx.stack.push(ops::Value::Float(0.))},
                            ops::Value::Str(x) => {
                                if let Ok(new_x) = ctx.str_heap[x].parse::<f32>() {
                                    ctx.stack.push(ops::Value::Float(new_x));
                                } else {
                                    return Err(("Failed to cast to Float", token.pos.clone()));
                                }
                            }
                            _ => {
                                let err_s: String = format!("Can't cast {} to {}", b, typ).to_owned();
                                return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                            }
                        }
                    },
                    (ops::Value::TypeLiteral(ops::TypeLiteral::Str), _) => {
                        match b {
                            ops::Value::Int(x) => {
                                let new_x = x.to_string();
                                ctx.str_heap.push(new_x);
                                ctx.stack.push(ops::Value::Str(ctx.str_heap.len() - 1))
                            },
                            ops::Value::Float(x) => {
                                let new_x = x.to_string();
                                ctx.str_heap.push(new_x);
                                ctx.stack.push(ops::Value::Str(ctx.str_heap.len() - 1))
                            },
                            ops::Value::Bool(x) => {
                                let new_x = x.to_string();
                                ctx.str_heap.push(new_x);
                                ctx.stack.push(ops::Value::Str(ctx.str_heap.len() - 1))
                            },
                            _ => {
                                let err_s: String = format!("Can't cast {} to {}", b, typ).to_owned();
                                return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                            }
                        }
                    },
                    (_, _) => {
                        let err_s: String = format!("Can't cast {} to {}. Second argument must be a TypeLitr", b, typ).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            },
        }
        // println!("{:?}", token.op);
        i += 1;
    }

    Ok(0)
}

