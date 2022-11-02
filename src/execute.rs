use crate::{ops, Runtime};
use snailquote::unescape;
use std::io::{stdin, stdout, Write};

pub fn execute(
    ctx: &mut Runtime,
    prg: &Vec<ops::Instruction>,
) -> Result<u8, (&'static str, ops::Pos)> {
    let mut i = 0;
    while i < prg.len() {
        let token = &prg[i];

        match token.op {
            ops::Operator::Literal => ctx.push(token.val.unwrap()),
            ops::Operator::Add => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'+' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Int(x + y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Float(x + y))
                    }
                    (ops::Value::Str(x), ops::Value::Str(y)) => {
                        let mut new_str: String = ctx.str_heap[x].clone();
                        new_str.push_str(ctx.str_heap[y].as_str());

                        ctx.str_heap.push(new_str);
                        ctx.push(ops::Value::Str(ctx.str_heap.len() - 1 as usize))
                    }
                    (_, _) => {
                        let err_s: String = format!("'{} + {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Sub => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'-' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Int(x - y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Float(x - y))
                    }
                    (_, _) => {
                        let err_s: String = format!("'{} - {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Mult => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'*' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Int(x * y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Float(x * y))
                    }
                    (_, _) => {
                        let err_s: String = format!("'{} * {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Div => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'/' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => {
                        ctx.push(ops::Value::Int(x % y));
                        ctx.push(ops::Value::Int(x / y))
                    }
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Float(x % y));
                        ctx.push(ops::Value::Float(x / y))
                    }
                    (_, _) => {
                        let err_s: String = format!("'{} / {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Print => {
                if ctx.stack.len() < 1 {
                    return Err(("'skriv-ut' operator krever minst 1 argument", token.pos.clone()));
                }

                let print_val = ctx.pop().unwrap();
                let _ = stdout().flush();
                match print_val {
                    ops::Value::Int(x) => print!("{}\n", x),
                    ops::Value::Float(x) => print!("{}\n", x),
                    ops::Value::Bool(x) => print!("{}\n", x),
                    ops::Value::Str(x) => print!("{}\n", ctx.str_heap[x]),
                    ops::Value::TypeLiteral(_) => todo!("print for TypeLiter is not implemented"),
                    ops::Value::Ptr(_) => todo!("print for Pointer is not implemented"),
                }
            }
            ops::Operator::Input => {
                let print_value = ctx.pop();

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
                ctx.push(ops::Value::Str(i));
            }
            ops::Operator::Not => {
                if ctx.stack.len() < 1 {
                    return Err(("'ikke' operator krever minst 1 argument", token.pos.clone()));
                }

                let b = ctx.pop().unwrap();

                match b {
                    ops::Value::Bool(x) => ctx.push(ops::Value::Bool(!x)),
                    _ => {
                        let err_s: String = format!("'ike {}' er ikke støttet", b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::And => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'og' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => {
                        ctx.push(ops::Value::Bool(x && y))
                    }
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Int(x & y)),
                    (_, _) => {
                        let err_s: String = format!("'{} en {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Or => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'eller' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => {
                        ctx.push(ops::Value::Bool(x || y))
                    }
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Int(x | y)),
                    (_, _) => {
                        let err_s: String = format!("'{} anu {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Eq => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'=' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => {
                        ctx.push(ops::Value::Bool(x == y))
                    }
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Bool(x == y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Bool(x == y))
                    }
                    (ops::Value::Str(x), ops::Value::Str(y)) => {
                        ctx.push(ops::Value::Bool(ctx.str_heap[x] == ctx.str_heap[y]))
                    }
                    (_, ops::Value::TypeLiteral(ops::TypeLiteral::Int)) => {
                        if let ops::Value::Int(_) = a {
                            ctx.push(ops::Value::Bool(true))
                        } else {
                            ctx.push(ops::Value::Bool(false))
                        }
                    }
                    (_, ops::Value::TypeLiteral(ops::TypeLiteral::Str)) => {
                        if let ops::Value::Str(_) = a {
                            ctx.push(ops::Value::Bool(true))
                        } else {
                            ctx.push(ops::Value::Bool(false))
                        }
                    }
                    (_, _) => {
                        let err_s: String = format!("'{} = {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Lt => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'<' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.push(ops::Value::Bool(x < y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Bool(x < y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Bool(x < y))
                    }
                    (_, _) => {
                        let err_s: String = format!("'{} < {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Le => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'<=' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => {
                        ctx.push(ops::Value::Bool(x <= y))
                    }
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Bool(x <= y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Bool(x <= y))
                    }
                    (_, _) => {
                        let err_s: String = format!("'{} <= {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Gt => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'>' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.push(ops::Value::Bool(x > y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Bool(x > y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Bool(x > y))
                    }
                    (_, _) => {
                        let err_s: String = format!("'{} > {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Ge => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'>=' operator krever minst 2 argumenter av samme type",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (a, b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => {
                        ctx.push(ops::Value::Bool(x >= y))
                    }
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Bool(x >= y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Bool(x >= y))
                    }
                    (_, _) => {
                        let err_s: String = format!("'{} >= {}' er ikke støttet", a, b).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::If => {
                if ctx.stack.len() < 1 {
                    return Err(("'hvis' operator krever minst 1 argumentet", token.pos.clone()));
                }

                let con = ctx.pop().unwrap();

                if let ops::Value::Bool(x) = con {
                    if x {
                        i += 1;
                        continue;
                    } else {
                        i = token.arg.unwrap();
                    }
                } else {
                    let err_s: String =
                        format!("'ike {}' er ikke støttet. 'ike' only takes Bool", con).to_owned();
                    return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                }
            }
            ops::Operator::End => {
                if let Some(ptr) = token.arg {
                    if let ops::Operator::Const = prg[ptr].op {
                        if ctx.stack.len() < 1 {
                            return Err((
                                "'konst' definisjon krever et element på toppen av stabelen",
                                token.pos.clone(),
                            ));
                        }
                        let val = ctx.pop().unwrap();
                        let name = &prg[ptr + 1].name;
                        

                        if let Some(key) = name {
                            if ctx.def[key] != None {
                                let err_s: String = format!("'{}' kan ikke omdefineres ettersom den er konstant", key).to_owned();
                                return Err((
                                    Box::leak(err_s.into_boxed_str()),
                                    token.pos.clone(),
                                ));
                            } 
                            ctx.def.insert(key.to_string(), Some(val));
                        } else {
                            return Err(("Kunne ikke finne valid 'konst' navn", token.pos.clone()));
                        }
                    } else {
                        i = ptr;
                    }
                }
            }
            ops::Operator::Else => {
                if let Some(ptr) = token.arg {
                    i = ptr;
                }
            }
            ops::Operator::Do => {
                if ctx.stack.len() < 1 {
                    return Err(("'gjør' operator krever minst 1 argument", token.pos.clone()));
                }

                let con = ctx.pop().unwrap();

                if let ops::Value::Bool(x) = con {
                    if x {
                        i += 1;
                        continue;
                    } else {
                        i = token.arg.unwrap();
                    }
                } else {
                    let err_s: String =
                        format!("'gjør {}' er ikke støttet. 'gjør' only takes Bool", con)
                            .to_owned();
                    return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                }
            }
            ops::Operator::While | ops::Operator::Const => (),
            ops::Operator::Dup => {
                if ctx.stack.len() < 1 {
                    return Err(("'dup' operator krever minst 1 argument", token.pos.clone()));
                }

                let b = ctx.pop().unwrap();

                ctx.push(b);
                ctx.push(b);
            }
            ops::Operator::Drop => {
                if ctx.stack.len() < 1 {
                    return Err(("'slipp' operator krever minst 1 argument", token.pos.clone()));
                }

                let _ = ctx.pop().unwrap();
            }
            ops::Operator::Swap => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'snu' operator krever minst 1 argument",
                        token.pos.clone(),
                    ));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                ctx.push(b);
                ctx.push(a);
            }
            ops::Operator::Over => {
                if ctx.stack.len() < 2 {
                    return Err(("'over' operator krever minst 1 argument", token.pos.clone()));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                ctx.push(a);
                ctx.push(b);
                ctx.push(a);
            }
            ops::Operator::Rot => {
                if ctx.stack.len() < 3 {
                    return Err(("'rot' operator krever minst 1 argument", token.pos.clone()));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();
                let c = ctx.pop().unwrap();

                ctx.push(b);
                ctx.push(c);
                ctx.push(a);
            }
            ops::Operator::Cast => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'omgjør' operator krever minst 1 argument",
                        token.pos.clone(),
                    ));
                }

                let typ = ctx.pop().unwrap();
                let b = ctx.pop().unwrap();

                match (typ, b) {
                    (ops::Value::TypeLiteral(ops::TypeLiteral::Int), _) => match b {
                        ops::Value::Float(x) => ctx.push(ops::Value::Int(x as i32)),
                        ops::Value::Bool(x) => {
                            if x {
                                ctx.push(ops::Value::Int(1i32))
                            } else {
                                ctx.push(ops::Value::Int(0i32))
                            }
                        }
                        ops::Value::Str(x) => {
                            if let Ok(new_x) = ctx.str_heap[x].parse::<i32>() {
                                ctx.push(ops::Value::Int(new_x));
                            } else {
                                return Err((
                                    "Fikk ikke til å omgjøre til Heltall",
                                    token.pos.clone(),
                                ));
                            }
                        }
                        _ => {
                            let err_s: String =
                                format!("Kunne ikke omgjøre {} til {}", b, typ).to_owned();
                            return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                        }
                    },
                    (ops::Value::TypeLiteral(ops::TypeLiteral::Float), _) => match b {
                        ops::Value::Int(x) => ctx.push(ops::Value::Float(x as f32)),
                        ops::Value::Bool(x) => {
                            if x {
                                ctx.push(ops::Value::Float(1.))
                            } else {
                                ctx.push(ops::Value::Float(0.))
                            }
                        }
                        ops::Value::Str(x) => {
                            if let Ok(new_x) = ctx.str_heap[x].parse::<f32>() {
                                ctx.push(ops::Value::Float(new_x));
                            } else {
                                return Err((
                                    "Fikk ikke til å omgjøre til Flyttall",
                                    token.pos.clone(),
                                ));
                            }
                        }
                        _ => {
                            let err_s: String =
                                format!("Kunne ikke omgjøre {} til {}", b, typ).to_owned();
                            return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                        }
                    },
                    (ops::Value::TypeLiteral(ops::TypeLiteral::Str), _) => match b {
                        ops::Value::Int(x) => {
                            let new_x = x.to_string();
                            ctx.str_heap.push(new_x);
                            ctx.push(ops::Value::Str(ctx.str_heap.len() - 1))
                        }
                        ops::Value::Float(x) => {
                            let new_x = x.to_string();
                            ctx.str_heap.push(new_x);
                            ctx.push(ops::Value::Str(ctx.str_heap.len() - 1))
                        }
                        ops::Value::Bool(x) => {
                            let new_x = x.to_string();
                            ctx.str_heap.push(new_x);
                            ctx.push(ops::Value::Str(ctx.str_heap.len() - 1))
                        }
                        _ => {
                            let err_s: String =
                                format!("Kunne ikke omgjøre {} til {}", b, typ).to_owned();
                            return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                        }
                    },
                    (_, _) => {
                        let err_s: String = format!("Kunne ikke omgjøre {} til {}. Andre argument må være en bokstavelig type", b, typ).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                    }
                }
            }
            ops::Operator::Read => todo!(),
            ops::Operator::Write => todo!(),
            ops::Operator::Word => {
                if let Some(key) = &token.name {
                    if let Some(val) = ctx.def[key] {
                        ctx.push(val)
                    }
                }
            }
        }
        // println!("{:?}", token.op);
        i += 1;
    }

    Ok(0)
}
