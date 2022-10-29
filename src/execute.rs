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

                match print_val  {
                    ops::Value::Int(x) => println!("{}", x),
                    ops::Value::Float(x) => println!("{}", x),
                    ops::Value::Bool(x) => println!("{}", x),
                    ops::Value::Str(x) => println!("{}", ctx.str_heap[x]),
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
        }
        // println!("{:?}", token.op);
        i += 1;
    }

    Ok(0)
}

