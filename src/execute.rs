use crate::{ops, Runtime};
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Byte(x + y)),
                    (ops::Value::Ptr(x), ops::Value::Int(y)) => ctx.push(ops::Value::Ptr((x.0 + y as usize, x.1, x.2))),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Float(x + y))
                    }
                    (ops::Value::Str(x), ops::Value::Str(y)) => {
                        let s1: String = ctx.read_str(&ops::Value::Str(x)).unwrap().clone();
                        let s2: String = ctx.read_str(&ops::Value::Str(y)).unwrap().clone();
                        
                        let new_str = s1 + &s2;

                        let res = ctx.write(&crate::parse::parse_char(&new_str));
                        ctx.push(ops::Value::Str(res))
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Byte(x - y)),
                    (ops::Value::Ptr(x), ops::Value::Int(y)) => ctx.push(ops::Value::Ptr((x.0 - y as usize, x.1, x.2))),
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Byte(x * y)),
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => {
                        ctx.push(ops::Value::Byte(x % y));
                        ctx.push(ops::Value::Byte(x / y))
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
                    return Err((
                        "'skriv-ut' operator krever minst 1 argument",
                        token.pos.clone(),
                    ));
                }

                let print_val = ctx.pop().unwrap();
                let _ = stdout().flush();
                match print_val {
                    ops::Value::Int(x) => print!("{}\n", x),
                    ops::Value::Float(x) => print!("{}\n", x),
                    ops::Value::Bool(x) => print!("{}\n", if x { "Sann" } else { "Usann" }),
                    ops::Value::Str(_) => print!("{}", ctx.read_str(&print_val).unwrap()),
                    ops::Value::TypeLiteral(_) => todo!("print for TypeLiter is not implemented"),
                    ops::Value::Ptr(x) => print!("{:?}", x),
                    ops::Value::Byte(x) => print!("{:#02x}\n", x),
                    ops::Value::Char(x) => print!("{}\n", x),
                    ops::Value::Null => todo!(),
                    ops::Value::FuncPtr(_) => todo!(),
                }
            }
            ops::Operator::Input => {
                let print_value = ctx.pop();

                if let Some(ops::Value::Str(x)) = print_value {
                    print!("{}", ctx.read_str(&ops::Value::Str(x)).unwrap())
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

                let unescaped_x = crate::parse::parse_char(&s);
                let res = ctx.write(&unescaped_x);
                ctx.push(ops::Value::Str(res));
            }
            ops::Operator::Not => {
                if ctx.stack.len() < 1 {
                    return Err(("'ikke' operator krever minst 1 argument", token.pos.clone()));
                }

                let b = ctx.pop().unwrap();

                match b {
                    ops::Value::Bool(x) => ctx.push(ops::Value::Bool(!x)),
                    ops::Value::Int(x) => ctx.push(ops::Value::Int(!x)),
                    ops::Value::Byte(x) => ctx.push(ops::Value::Byte(!x)),
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Byte(x & y)),
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Byte(x | y)),
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => {
                        ctx.push(ops::Value::Bool(x == y))
                    }
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Bool(x == y))
                    }
                    (ops::Value::Str(x), ops::Value::Str(y)) => {
                        let s1 = ctx.read_str(&ops::Value::Str(x)).unwrap();
                        let s2 = ctx.read_str(&ops::Value::Str(y)).unwrap();

                        ctx.push(ops::Value::Bool(s1 == s2))
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Bool(x < y)),
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => {
                        ctx.push(ops::Value::Bool(x <= y))
                    }
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Bool(x > y)),
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
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => {
                        ctx.push(ops::Value::Bool(x >= y))
                    }
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
                    return Err((
                        "'hvis' operator krever minst 1 argumentet",
                        token.pos.clone(),
                    ));
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
                                let err_s: String = format!(
                                    "'{}' kan ikke omdefineres ettersom den er konstant",
                                    key
                                )
                                .to_owned();
                                return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                            }
                            ctx.def.insert(key.to_string(), Some(val));
                        } else {
                            return Err(("Kunne ikke finne valid 'konst' navn", token.pos.clone()));
                        }
                    } else if let ops::Operator::Mem = prg[ptr].op {
                        if ctx.stack.len() < 2 {
                            return Err((
                                "'minne' definisjon krever en type og en lengde på toppen av stabelenm",
                                token.pos.clone(),
                            ));
                        }

                        let ops::Value::Int(len) = ctx.pop().unwrap() else {
                            return Err((
                                "Verdien på toppen av stabelen må være et positivt heltall",
                                token.pos.clone(),
                            ));
                        };
                        if len <= 0 {
                            return Err((
                                "Verdien på toppen av stabelen må være et positivt heltall og kan ikke vær null eller mindre",
                                token.pos.clone(),
                            ));
                        }
                        let ops::Value::TypeLiteral(typ) = ctx.pop().unwrap() else {
                            return Err((
                                "Verdien på toppen av stabelen må være en type",
                                token.pos.clone(),
                            ));
                        };
                        let name = &prg[ptr + 1].name;



                        if let Some(key) = name {
                            if ctx.def[key] != None {
                                let err_s: String = format!(
                                    "'{}' kan ikke omdefineres ettersom den er konstant",
                                    key
                                )
                                .to_owned();
                                return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                            }

                            let res = ctx.write(&vec![ops::Value::Null; len as usize]);
                            let result = (res.0, res.1, typ);
                            ctx.def.insert(key.to_string(), Some(ops::Value::Ptr(result)));
                        } else {
                            return Err(("Kunne ikke finne valid 'konst' navn", token.pos.clone()));
                        }
                    } else if let ops::Operator::Func = prg[ptr].op {
                        let ret_val = ctx.pop();
                        let ret_typ = ctx.retur();
                        match (ret_val, ret_typ) {
                            (Some(val), Some(typ)) => {
                                if val.eq(&typ) {
                                    ctx.push(val)
                                } else {
                                    let err_s: String = format!(
                                        "forvendtet '{:?}' men fant '{}'",
                                        typ, val
                                    )
                                    .to_owned();
                                    return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                                }
                            },
                            (None, Some(_)) => {return Err(("Fant ingen retur verdi", token.pos.clone()));},
                            (_,_) => ()
                        }
                        i = ctx.return_stack.pop().unwrap()
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
            ops::Operator::While | ops::Operator::Const | ops::Operator::Mem => (),
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
                    return Err((
                        "'slipp' operator krever minst 1 argument",
                        token.pos.clone(),
                    ));
                }

                let _ = ctx.pop().unwrap();
            }
            ops::Operator::Swap => {
                if ctx.stack.len() < 2 {
                    return Err(("'snu' operator krever minst 2 argumenter", token.pos.clone()));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                ctx.push(b);
                ctx.push(a);
            }
            ops::Operator::Over => {
                if ctx.stack.len() < 2 {
                    return Err(("'over' operator krever minst 2 argumenter", token.pos.clone()));
                }

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                ctx.push(a);
                ctx.push(b);
                ctx.push(a);
            }
            ops::Operator::Rot => {
                if ctx.stack.len() < 3 {
                    return Err(("'rot' operator krever minst 3 argumenter", token.pos.clone()));
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
                        "'omgjør' operator krever minst 2 argumenter",
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
                            if let Ok(new_x) = ctx.read_str(&ops::Value::Str(x)).unwrap().parse::<i32>() {
                                ctx.push(ops::Value::Int(new_x));
                            } else {
                                return Err((
                                    "Fikk ikke til å omgjøre til Helt",
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
                            if let Ok(new_x) = ctx.read_str(&ops::Value::Str(x)).unwrap().parse::<f32>() {
                                ctx.push(ops::Value::Float(new_x));
                            } else {
                                return Err((
                                    "Fikk ikke til å omgjøre til Flyt",
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
                            let new_x = crate::parse::parse_char(&x.to_string());
                            let res = ctx.write(&new_x);
                            ctx.push(ops::Value::Str(res))
                        }
                        ops::Value::Float(x) => {
                            let new_x = crate::parse::parse_char(&x.to_string());
                            let res = ctx.write(&new_x);
                            ctx.push(ops::Value::Str(res))
                        }
                        ops::Value::Bool(x) => {
                            let new_x = crate::parse::parse_char(&x.to_string());
                            let res = ctx.write(&new_x);
                            ctx.push(ops::Value::Str(res))
                        }
                        _ => {
                            let err_s: String =
                                format!("Kunne ikke omgjøre {} til {}", b, typ).to_owned();
                            return Err((Box::leak(err_s.into_boxed_str()), token.pos.clone()));
                        }
                    },
                    (ops::Value::TypeLiteral(ops::TypeLiteral::Ptr), _) => match b {
                        ops::Value::Str(x) => {
                            let new_x = (x.0, x.1, ops::TypeLiteral::Char);
                            ctx.push(ops::Value::Ptr(new_x))
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
            ops::Operator::Read => {
                if ctx.stack.len() < 1 {
                    return Err((
                        "',' operator krever minst 1 argument",
                        token.pos.clone(),
                    ));
                }

                let ptr = ctx.pop().unwrap();

                if let ops::Value::Ptr(x) = ptr {
                    let val = ctx.read(x.0).unwrap();
                    ctx.push(val)
                } else {
                    let err_s: String =
                                format!("Kunne ikke lese fra minne adresse '{}'", ptr).to_owned();
                    return Err((
                        Box::leak(err_s.into_boxed_str()),
                        token.pos.clone(),
                    ));
                }
            },
            ops::Operator::Write => {
                if ctx.stack.len() < 2 {
                    return Err((
                        "'.' operator krever minst 2 argument",
                        token.pos.clone(),
                    ));
                }
                let val = ctx.pop().unwrap();
                let ptr = ctx.pop().unwrap();

                if let ops::Value::Ptr(x) = ptr {
                    if !val.eq(&x.2) {
                        let err_s: String =
                                format!("Forvendtet {:?} men fant {}", x.2, val).to_owned();
                    return Err((
                        Box::leak(err_s.into_boxed_str()),
                        token.pos.clone(),
                    ));
                    }
                    ctx.over_write(x.0, &val)
                } else {
                    return Err((
                        "'.' operator krever at andre operator er en peker",
                        token.pos.clone(),
                    ));
                }
            },
            ops::Operator::Word => {
                if let Some(key) = &token.name {
                    if let Some(ops::Value::FuncPtr(func_ptr)) = ctx.def[key] {
                        ctx.return_stack.push(i);
                        i = func_ptr
                    } else if let Some(val) = ctx.def[key] {
                        ctx.push(val)
                    }
                }
            }
            ops::Operator::Exit => {
                if ctx.stack.len() < 1 {
                    return Err((
                        "'avslutt' operator krever minst 1 argument",
                        token.pos.clone(),
                    ));
                }

                let code = ctx.pop().unwrap();
                if let ops::Value::Int(x) = code {
                    return Ok(x as u8);
                } else {
                    return Err(("Avslutnings kode må være ett 'Helt'", token.pos.clone()));
                }
            }
            ops::Operator::Func => {
                let word_i = i + 1;
                let word_opt = &prg[word_i].name;
                if let Some(word) = word_opt {
                    ctx.def.insert(word.clone(), Some(ops::Value::FuncPtr(word_i)));
                } else {
                    return Err(("Kunne ikke finne funksjons navn", token.pos.clone()));
                }
                i = token.arg.unwrap();
            },
            ops::Operator::In => {
                let mut params: Vec<ops::TypeLiteral> = vec![];
                let mut params_collected = false;
                let mut ret_typ = None;
                while !params_collected {
                    if let Some(ops::Value::TypeLiteral(_)) = ctx.peek() {
                        let ops::Value::TypeLiteral(typ) = ctx.pop().unwrap() else {
                            return Err(("Noet gikk galt med funksjons definisjonen", token.pos.clone()));
                        };
                        params.push(typ)
                    } else if let Some(ops::Value::Null) = ctx.peek() {
                        let _ = ctx.pop();
                        match params.len() {
                            0 => {return Err(("Fant ingen returnerings type", token.pos.clone()));},
                            1 => {
                                ret_typ = params.pop()
                            }
                            _ => {return Err(("Kan ikke returnere mer enn en verdi omgangen", token.pos.clone()));},
                        }
                    } else {
                        params_collected = true
                    }
                }

                let mut params_value: Vec<ops::Value> = vec![];
                for typ in params {
                    let stack_val = ctx.pop();
                    if let Some(val) = stack_val {
                        if val.eq(&typ) {
                            params_value.push(val)
                        } else {
                            let err_s: String =
                                format!("Forventet '{:?}' men fant '{}'", typ, val).to_owned();
                            return Err((
                                Box::leak(err_s.into_boxed_str()),
                                token.pos.clone(),
                            ));
                        }
                    } else {
                        return Err((
                            "Fant ikke nokk argumenter for funksjon",
                            token.pos.clone(),
                        ));
                    }
                }
                params_value.reverse();
                ctx.swap(params_value, ret_typ)
            },
            ops::Operator::BikeShed => {
                ctx.push(ops::Value::Null)
            }
        }
        // println!("{:?}", token.op);
        i += 1;
    }

    Ok(0)
}
