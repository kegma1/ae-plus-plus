use crate::{ops, report_err, Runtime};
use std::io::{stdin, stdout, Write};

macro_rules! check_stack_min {
    ($ctx:expr, $tok:expr, $min_len:expr, $err_msg:expr) => {
        if $ctx.stack.len() < $min_len {
            report_err!($tok.pos, $err_msg)
        }
    };
}

pub fn execute(
    ctx: &mut Runtime,
    prg: &Vec<ops::Instruction>,
) -> Result<u8, (&'static str, ops::Pos)> {
    let mut i = 0;
    while i < prg.len() {
        // println!("{}", ctx.current_scope);
        let token = &prg[i];

        match token.op {
            ops::Operator::Literal => ctx.push(token.val.clone().unwrap()),
            ops::Operator::Add => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'+' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Int(x + y)),
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Byte(x + y)),
                    (ops::Value::Ptr(x), ops::Value::Int(y)) => {
                        ctx.push(ops::Value::Ptr((x.0 + *y as usize, x.1, x.2)))
                    }
                    (ops::Value::Int(y), ops::Value::Ptr(x)) => {
                        ctx.push(ops::Value::Ptr((x.0 + *y as usize, x.1, x.2)))
                    }
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Float(x + y))
                    }
                    (ops::Value::Str(x), ops::Value::Str(y)) => {
                        let s1: String = ctx.read_str(&ops::Value::Str(*x)).unwrap().clone();
                        let s2: String = ctx.read_str(&ops::Value::Str(*y)).unwrap().clone();

                        let new_str = s1 + &s2;

                        let res = ctx.write(&crate::parse::parse_char(&new_str));
                        ctx.push(ops::Value::Str(res))
                    }
                    (ops::Value::Str(x), ops::Value::Char(y)) => {
                        let s1: String = ctx.read_str(&ops::Value::Str(*x)).unwrap().clone();

                        let new_str = s1 + &y.to_string();

                        let res = ctx.write(&crate::parse::parse_char(&new_str));
                        ctx.push(ops::Value::Str(res))
                    }
                    (_, _) => {
                        report_err!(token.pos, "'{} + {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::Sub => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'-' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Int(x - y)),
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Byte(x - y)),
                    (ops::Value::Ptr(x), ops::Value::Int(y)) => {
                        ctx.push(ops::Value::Ptr((x.0 - *y as usize, x.1, x.2)))
                    }
                    (ops::Value::Int(y), ops::Value::Ptr(x)) => {
                        ctx.push(ops::Value::Ptr((x.0 - *y as usize, x.1, x.2)))
                    }
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Float(x - y))
                    }
                    (_, _) => {
                        report_err!(token.pos, "'{} - {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::Mult => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'*' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Int(x * y)),
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Byte(x * y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Float(x * y))
                    }
                    (_, _) => {
                        report_err!(token.pos, "'{} * {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::Div => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'/' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
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
                        report_err!(token.pos, "'{} / {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::Print => {
                check_stack_min!(ctx, token, 1, "'skriv' operator krever minst 1 argument");

                let print_val = ctx.pop().unwrap();
                print!("{}", print_val.to_string(ctx))
            }
            ops::Operator::PrintLn => {
                check_stack_min!(ctx, token, 1, "'skrivnl' operator krever minst 1 argument");

                let print_val = ctx.pop().unwrap();
                print!("{}\n", print_val.to_string(ctx))
            }
            ops::Operator::Input => {
                let print_value = ctx.pop();

                if let Some(x) = print_value {
                    print!("{}", x.to_string(ctx))
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
                check_stack_min!(ctx, token, 1, "'ikke' operator krever minst 1 argument");

                let b = ctx.pop().unwrap();

                match b {
                    ops::Value::Bool(x) => ctx.push(ops::Value::Bool(!x)),
                    ops::Value::Int(x) => ctx.push(ops::Value::Int(!x)),
                    ops::Value::Byte(x) => ctx.push(ops::Value::Byte(!x)),
                    _ => {
                        report_err!(token.pos, "'ikke {}' er ikke støttet", b);
                    }
                }
            }
            ops::Operator::And => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'og' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => {
                        ctx.push(ops::Value::Bool(*x && *y))
                    }
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Int(x & y)),
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Byte(x & y)),
                    (_, _) => {
                        report_err!(token.pos, "'{} og {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::Or => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'eller' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => {
                        ctx.push(ops::Value::Bool(*x || *y))
                    }
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Int(x | y)),
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Byte(x | y)),
                    (_, _) => {
                        report_err!(token.pos, "'{} eller {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::Eq => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'=' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
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
                        let s1 = ctx.read_str(&ops::Value::Str(*x)).unwrap();
                        let s2 = ctx.read_str(&ops::Value::Str(*y)).unwrap();

                        ctx.push(ops::Value::Bool(s1 == s2))
                    }
                    (ops::Value::Char(x), ops::Value::Char(y)) => {
                        ctx.push(ops::Value::Bool(x == y))
                    }
                    (x, ops::Value::TypeLiteral(y)) => ctx.push(ops::Value::Bool(x.eq(&y))),
                    (_, _) => {
                        report_err!(token.pos, "'{} = {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::Lt => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'<' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.push(ops::Value::Bool(x < y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Bool(x < y)),
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Bool(x < y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Bool(x < y))
                    }
                    (_, _) => {
                        report_err!(token.pos, "'{} < {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::Le => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'<=' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
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
                        report_err!(token.pos, "'{} <= {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::Gt => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'>' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
                    (ops::Value::Bool(x), ops::Value::Bool(y)) => ctx.push(ops::Value::Bool(x > y)),
                    (ops::Value::Int(x), ops::Value::Int(y)) => ctx.push(ops::Value::Bool(x > y)),
                    (ops::Value::Byte(x), ops::Value::Byte(y)) => ctx.push(ops::Value::Bool(x > y)),
                    (ops::Value::Float(x), ops::Value::Float(y)) => {
                        ctx.push(ops::Value::Bool(x > y))
                    }
                    (_, _) => {
                        report_err!(token.pos, "'{} > {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::Ge => {
                check_stack_min!(
                    ctx,
                    token,
                    2,
                    "'>=' operator krever minst 2 argumenter av samme type"
                );

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                match (&a, &b) {
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
                        report_err!(token.pos, "'{} >= {}' er ikke støttet", a, b);
                    }
                }
            }
            ops::Operator::If => (),
            ops::Operator::End => {
                let def_copy = ctx.def.clone();
                for (key, (_, scope)) in &def_copy {
                    if scope >= &ctx.current_scope {
                        ctx.def.remove(key);
                    }
                }
                ctx.current_scope -= 1;
                if let Some(ptr) = token.arg {
                    match prg[ptr].op {
                        ops::Operator::Const => {
                            check_stack_min!(
                                ctx,
                                token,
                                1,
                                "'konst' definisjon krever et element på toppen av stabelen"
                            );

                            let val = ctx.pop().unwrap();
                            let name = &prg[ptr + 1].name;

                            if let Some(key) = name {
                                if let (Some(_), _) = ctx.def[key] {
                                    report_err!(
                                        token.pos,
                                        "'{}' kan ikke omdefineres ettersom den er konstant",
                                        key
                                    );
                                }
                                ctx.def.insert(key.to_string(), (Some(val), ctx.current_scope));
                            } else {
                                return Err((
                                    "Kunne ikke finne valid 'konst' navn",
                                    token.pos.clone(),
                                ));
                            }
                        }
                        ops::Operator::Mem => {
                            check_stack_min!(
                            ctx,
                            token,
                            2,
                            "'minne' definisjon krever en type og en lengde på toppen av stabelen"
                        );

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
                                if let (Some(_), _) = ctx.def[key] {
                                    report_err!(
                                        token.pos,
                                        "'{}' kan ikke omdefineres ettersom den er konstant",
                                        key
                                    );
                                }

                                let res = ctx.write(&vec![ops::Value::Null; len as usize]);
                                let result = (res.0, res.1, typ);
                                ctx.def
                                    .insert(key.to_string(), (Some(ops::Value::Ptr(result)), ctx.current_scope));
                            } else {
                                return Err((
                                    "Kunne ikke finne valid 'konst' navn",
                                    token.pos.clone(),
                                ));
                            }
                        }
                        ops::Operator::Func => {
                            let func = &prg[ptr + 1];
                            let Some(func_name) = &func.name else {report_err!(token.pos, "fant ikke funksjons navn");};
                            let (Some(ops::Value::FuncPtr(func_ptr)), _) = ctx.def[func_name].clone() else {
                                report_err!(token.pos, "fant ikke funksjons navn");
                            };
                            let Some(res) = ctx.retur(&func_ptr) else {
                                report_err!(token.pos, "ikke rette retur verdier for funksjon '{}'", func_name);
                            };

                            i = res
                        }
                        ops::Operator::Let => (),
                        _ => {
                            i = ptr;
                        }
                    }
                }
            }
            ops::Operator::Else | ops::Operator::Elif => {
                if let Some(ptr) = token.arg {
                    let def_copy = ctx.def.clone();
                    for (key, (_, scope)) in &def_copy {
                    if scope >= &ctx.current_scope {
                        ctx.def.remove(key);
                    }
                }
                    ctx.current_scope -= 1;
                    i = ptr;
                }
            }
            ops::Operator::Do => {
                check_stack_min!(ctx, token, 1, "'gjør' operator krever minst 1 argument");

                let con = ctx.pop().unwrap();

                if let ops::Value::Bool(x) = con {
                    if x {
                        i += 1;
                        ctx.current_scope += 1;
                        continue;
                    } else {
                        let next_i = token.arg.unwrap();
                        if prg[next_i].op == ops::Operator::Else {
                            ctx.current_scope += 1;
                            i = next_i
                        } else {
                            i = next_i
                        }
                    }
                } else {
                    report_err!(
                        token.pos,
                        "'gjør {}' er ikke støttet. 'gjør' only takes Bool",
                        con
                    );
                }
            }
            ops::Operator::While => (),
            ops::Operator::Mem => {
                let name = &prg[i + 1];
                if name.op == ops::Operator::Word {
                    let Some(key) = &name.name else {
                        report_err!(token.pos, "Kunne ikke finne navn");
                    };
                    ctx.def.insert(key.to_string(), (None, ctx.current_scope));
                    ctx.current_scope += 1;
                } else {
                    report_err!(token.pos, "Kunne ikke finne navn til minne");
                }
                i += 1
            }
            ops::Operator::Const => {
                let name = &prg[i + 1];
                if name.op == ops::Operator::Word {
                    let Some(key) = &name.name else {
                        report_err!(token.pos, "Kunne ikke finne navn");
                    };
                    ctx.def.insert(key.to_string(), (None, ctx.current_scope));
                    ctx.current_scope += 1;
                } else {
                    report_err!(token.pos, "Kunne ikke finne navn til konstant");
                }
                i += 1
            }
            ops::Operator::Dup => {
                if ctx.stack.len() < 1 {
                    return Err(("'dup' operator krever minst 1 argument", token.pos.clone()));
                }

                let b = ctx.pop().unwrap();

                ctx.push(b.clone());
                ctx.push(b);
            }
            ops::Operator::Drop => {
                check_stack_min!(ctx, token, 1, "'slipp' operator krever minst 1 argument");

                let _ = ctx.pop().unwrap();
            }
            ops::Operator::Swap => {
                check_stack_min!(ctx, token, 2, "'snu' operator krever minst 2 argumenter");

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                ctx.push(b);
                ctx.push(a);
            }
            ops::Operator::Over => {
                check_stack_min!(ctx, token, 2, "'over' operator krever minst 2 argumenter");

                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                ctx.push(a.clone());
                ctx.push(b);
                ctx.push(a);
            }
            ops::Operator::Rot => {
                check_stack_min!(ctx, token, 3, "'rot' operator krever minst 3 argumenter");

                let c = ctx.pop().unwrap();
                let b = ctx.pop().unwrap();
                let a = ctx.pop().unwrap();

                ctx.push(b);
                ctx.push(c);
                ctx.push(a);
            }
            ops::Operator::Cast => {
                check_stack_min!(ctx, token, 2, "'omgjør' operator krever minst 2 argumenter");

                let typ = ctx.pop().unwrap();
                let b = ctx.pop().unwrap();

                match (&typ, &b) {
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
                            if let Ok(new_x) =
                                ctx.read_str(&ops::Value::Str(x)).unwrap().parse::<i32>()
                            {
                                ctx.push(ops::Value::Int(new_x));
                            } else {
                                return Err((
                                    "Fikk ikke til å omgjøre til Helt",
                                    token.pos.clone(),
                                ));
                            }
                        }
                        _ => {
                            report_err!(token.pos, "Kunne ikke omgjøre {} til {}", b, typ);
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
                            if let Ok(new_x) =
                                ctx.read_str(&ops::Value::Str(x)).unwrap().parse::<f32>()
                            {
                                ctx.push(ops::Value::Float(new_x));
                            } else {
                                return Err((
                                    "Fikk ikke til å omgjøre til Flyt",
                                    token.pos.clone(),
                                ));
                            }
                        }
                        _ => {
                            report_err!(token.pos, "Kunne ikke omgjøre {} til {}", b, typ);
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
                        ops::Value::Ptr(x) => {
                            if x.2 != ops::TypeLiteral::Char {
                                report_err!(token.pos, "Forventet 'Bokst' fant '{:?} ", x.2);
                            }
                            ctx.push(ops::Value::Str((x.0, x.1)))
                        }
                        _ => {
                            report_err!(token.pos, "Kunne ikke omgjøre {} til {}", b, typ);
                        }
                    },
                    (ops::Value::TypeLiteral(ops::TypeLiteral::Ptr), _) => match b {
                        ops::Value::Str(x) => {
                            let new_x = (x.0, x.1, ops::TypeLiteral::Char);
                            ctx.push(ops::Value::Ptr(new_x))
                        }
                        _ => {
                            report_err!(token.pos, "Kunne ikke omgjøre {} til {}", b, typ);
                        }
                    },
                    (_, _) => {
                        report_err!(token.pos,"Kunne ikke omgjøre {} til {}. Andre argument må være en bokstavelig type", b, typ);
                    }
                }
            }
            ops::Operator::Read => {
                check_stack_min!(ctx, token, 1, "',' operator krever minst 1 argument");

                let ptr = ctx.pop().unwrap();

                if let ops::Value::Ptr(x) = ptr {
                    let val = ctx.read(x.0).unwrap();
                    ctx.push(val)
                } else {
                    report_err!(token.pos, "Kunne ikke lese fra minne adresse '{}'", ptr);
                }
            }
            ops::Operator::Write => {
                check_stack_min!(ctx, token, 2, "'.' operator krever minst 2 argument");

                let val = ctx.pop().unwrap(); // burde switches rundt, peker først så verdi
                let ptr = ctx.pop().unwrap();

                if let ops::Value::Ptr(x) = ptr {
                    if !val.eq(&x.2) {
                        report_err!(token.pos, "Forventet {:?} men fant {}", x.2, val);
                    }
                    ctx.over_write(x.0, &val)
                } else {
                    return Err((
                        "'.' operator krever at andre operator er en peker",
                        token.pos.clone(),
                    ));
                }
            }
            ops::Operator::Word => {
                if let Some(key) = &token.name {
                    if let Some((Some(ops::Value::FuncPtr(func_ptr)), _)) = ctx.def.clone().get(key) {
                        let Some(res) = ctx.call(&func_ptr, i) else {
                            report_err!(token.pos, "feil argumenter for funksjon '{}'", key);
                        };
                        i = res
                    } else if let Some((Some(val), _)) = ctx.def.clone().get(key) {
                        ctx.push(val.clone())
                    } else {
                        report_err!(token.pos, "Ukjent ord '{}'", key);
                    }
                }
            }
            ops::Operator::Exit => {
                check_stack_min!(ctx, token, 1, "'avslutt' operator krever minst 1 argument");
                let code = ctx.pop().unwrap();
                if let ops::Value::Int(x) = code {
                    let _ = stdout().flush();
                    return Ok(x as u8);
                } else {
                    return Err(("Avslutnings kode må være ett 'Helt'", token.pos.clone()));
                }
            }
            ops::Operator::Func => {
                let name = &prg[i + 1];
                if name.op == ops::Operator::Word {
                    let Some(key) = &name.name else {
                        report_err!(token.pos, "Kunne ikke finne navn");
                    };
                    let mut params: Vec<ops::TypeLiteral> = vec![];
                    let mut returns: Vec<ops::TypeLiteral> = vec![];
                    let mut all_params_found = false;

                    let mut j = i + 1;
                    while &prg[j].op != &ops::Operator::In {
                        let current_argument = &prg[j];
                        if !all_params_found {
                            match &current_argument.val {
                                Some(ops::Value::TypeLiteral(arg_typ)) => {params.push(*arg_typ)},
                                Some(_) => {report_err!(current_argument.pos, "Forventet 'TypeLitr' men fant {}", current_argument.val.as_ref().unwrap());},
                                None => ()
                            }
                        } else {
                            match &current_argument.val {
                                Some(ops::Value::TypeLiteral(arg_typ)) => {returns.push(*arg_typ)},
                                Some(_) => {report_err!(current_argument.pos, "Forventet 'TypeLitr' men fant {}", current_argument.val.as_ref().unwrap());},
                                None => ()
                            }
                        }

                        if current_argument.op == ops::Operator::BikeShed {
                            all_params_found = true
                        }

                        j += 1;
                    }

                    let func_ptr = ops::FuncPtr {
                        ptr: j,
                        params,
                        returns,
                    };

                    ctx.def
                        .insert(key.to_string(), (Some(ops::Value::FuncPtr(func_ptr)), ctx.current_scope));
                } else {
                    report_err!(token.pos, "Kunne ikke finne navn til funksjon");
                }
                i = token.arg.unwrap()
            }
            ops::Operator::In => (),
            ops::Operator::BikeShed => (),
            ops::Operator::Let => {
                let mut j = i + 1;
                let mut vars = vec![];
                while let ops::Operator::Word = prg[j].op {
                    let name = prg[j].name.as_ref().unwrap();
                    vars.push(name);
                    j += 1
                }
                vars.reverse();
                if let ops::Operator::In = prg[j].op {
                    ctx.current_scope += 1;
                    for name in vars {
                        let Some(val) = ctx.pop() else {
                            report_err!(token.pos, "Ikke nokk verdier på stabelen for let-binding");
                        };
                        ctx.def.insert(name.clone(), (Some(val), ctx.current_scope));
                    }
                    i = j
                } else {
                    report_err!(prg[j].pos, "forventet 'inni' men fant '{:?}'", prg[j].op);
                }
            }
            ops::Operator::Debug => {
                let _ = stdout().flush();
                println!("");
                let width = termsize::get().unwrap().cols.into();

                print!("Stabel: ");
                let mut stack = String::from("");
                for v in &ctx.stack {
                    stack.push_str(&format!("{}, ", v.to_string(ctx)));
                }

                if (stack.len() + 8) <= width {
                    print!("{}", stack);
                } else {
                    print!(
                        "...{}",
                        &stack[(stack.len() - (width - 11))..(stack.len() - 1)]
                    );
                }
                print!("\n");
            }
        }
        // println!("{:?}", token.op);
        i += 1;
    }

    Ok(0)
}
