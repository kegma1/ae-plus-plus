use crate::{ops, Runtime};
use snailquote::unescape;
// https://github.com/ttm/tokipona/blob/master/data/toki-pona_english.txt

enum Mode {
    Normal,
    Define,
    Function,
}

pub fn parse(
    prg: Vec<(String, ops::Pos)>,
    ctx: &mut Runtime,
) -> Result<Vec<ops::Instruction>, (&'static str, ops::Pos)> {
    // println!("{:?}",prg);

    let mut parsed_prg: Vec<ops::Instruction> = vec![];

    let unwraped_prg = prg;
    let mut i = 0;
    let prg_len = unwraped_prg.len();
    let mut state = Mode::Normal;
    while i < prg_len {
        let (token, pos) = unwraped_prg[i].clone();
        // println!("{}", i);
        parsed_prg.push(match token.as_str() {
            "skriv-ut" => ops::Instruction::new(ops::Operator::Print, None, None, pos),
            "spør" => ops::Instruction::new(ops::Operator::Input, None, None, pos),
            "+" => ops::Instruction::new(ops::Operator::Add, None, None, pos),
            "-" => ops::Instruction::new(ops::Operator::Sub, None, None, pos),
            "*" => ops::Instruction::new(ops::Operator::Mult, None, None, pos),
            "/" => ops::Instruction::new(ops::Operator::Div, None, None, pos),
            "--" => ops::Instruction::new(ops::Operator::BikeShed, None, None, pos),
            "ikke" => ops::Instruction::new(ops::Operator::Not, None, None, pos),
            "og" => ops::Instruction::new(ops::Operator::And, None, None, pos),
            "eller" => ops::Instruction::new(ops::Operator::Or, None, None, pos),
            "hvis" => ops::Instruction::new(ops::Operator::If, None, None, pos),
            "ellers" => ops::Instruction::new(ops::Operator::Else, None, None, pos),
            "slutt" => ops::Instruction::new(ops::Operator::End, None, None, pos),
            "gjør" => ops::Instruction::new(ops::Operator::Do, None, None, pos),
            "inni" => ops::Instruction::new(ops::Operator::In, None, None, pos),
            "når" => ops::Instruction::new(ops::Operator::While, None, None, pos),
            "dup" => ops::Instruction::new(ops::Operator::Dup, None, None, pos),
            "rot" => ops::Instruction::new(ops::Operator::Rot, None, None, pos),
            "over" => ops::Instruction::new(ops::Operator::Over, None, None, pos),
            "slipp" => ops::Instruction::new(ops::Operator::Drop, None, None, pos),
            "snu" => ops::Instruction::new(ops::Operator::Swap, None, None, pos),
            "avslutt" => ops::Instruction::new(ops::Operator::Exit, None, None, pos),
            "omgjør" => ops::Instruction::new(ops::Operator::Cast, None, None, pos),
            "konst" => {
                state = Mode::Define;
                ops::Instruction::new(ops::Operator::Const, None, None, pos)
            }
            "minne" => {
                state = Mode::Define;
                ops::Instruction::new(ops::Operator::Mem, None, None, pos)
            }
            "funk" => {
                state = Mode::Function;
                ops::Instruction::new(ops::Operator::Func, None, None, pos)
            }
            "=" => ops::Instruction::new(ops::Operator::Eq, None, None, pos),
            ">" => ops::Instruction::new(ops::Operator::Gt, None, None, pos),
            ">=" => ops::Instruction::new(ops::Operator::Ge, None, None, pos),
            "<" => ops::Instruction::new(ops::Operator::Lt, None, None, pos),
            "<=" => ops::Instruction::new(ops::Operator::Le, None, None, pos),
            "," => ops::Instruction::new(ops::Operator::Read, None, None, pos),
            "." => ops::Instruction::new(ops::Operator::Write, None, None, pos),

            "Helt" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Int)),
                None,
                pos,
            ),
            "Flyt" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Float)),
                None,
                pos,
            ),
            "Str" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Str)),
                None,
                pos,
            ),
            "Bokst" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Char)),
                None,
                pos,
            ),
            "Peker" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Ptr)),
                None,
                pos,
            ),
            "Sann" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::Bool(true)),
                None,
                pos,
            ),
            "Usann" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::Bool(false)),
                None,
                pos,
            ),
            "Byte" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Byte)),
                None,
                pos,
            ),
            x if x.parse::<i32>().is_ok() => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::Int(x.parse::<i32>().unwrap())),
                None,
                pos,
            ),
            x if x.parse::<f32>().is_ok() => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::Float(x.parse::<f32>().unwrap())),
                None,
                pos,
            ),
            x if x.chars().nth(0) == Some('"') => {
                let unescaped_x = parse_char(x);

                if unescaped_x.len() <= 1 {
                    ops::Instruction::new(ops::Operator::Literal, Some(unescaped_x[0]), None, pos)
                } else {
                    let res = ctx.write(&unescaped_x);
                    ops::Instruction::new(
                        ops::Operator::Literal,
                        Some(ops::Value::Str(res)),
                        None,
                        pos,
                    )
                }
            }
            // x if x.chars().nth(0) == Some('b') => {
            //     if x.len() != 9 {
            //         return Err(("Kunne ikke oversette 'Byte'", pos));
            //     }

            //     let mut byte = x.chars();
            //     byte.next();
            //     let res = u8::from_str_radix(byte.as_str(), 2);
            //     if let Ok(u) = res {
            //         ops::Instruction::new(
            //             ops::Operator::Literal,
            //             Some(ops::Value::Byte(u)),
            //             None,
            //             pos,
            //         )
            //     } else {
            //         return Err(("Kunne ikke oversette 'Byte'", pos));
            //     }
            // }
            // x if x.chars().nth(0) == Some('x') => {
            //     if x.len() != 3 {
            //         return Err(("Kunne ikke oversette 'Byte'", pos));
            //     }

            //     let mut byte = x.chars();
            //     byte.next();
            //     let res = u8::from_str_radix(byte.as_str(), 16);
            //     if let Ok(u) = res {
            //         ops::Instruction::new(
            //             ops::Operator::Literal,
            //             Some(ops::Value::Byte(u)),
            //             None,
            //             pos,
            //         )
            //     } else {
            //         return Err(("Kunne ikke oversette 'Byte'", pos));
            //     }
            // }
            "" => {
                i += 1;
                continue;
            }
            _ => match state {
                Mode::Normal => {
                    if !ctx.def.contains_key(&token) {
                        let err_s: String = format!("ukjent ord '{}'", token).to_owned();
                        return Err((Box::leak(err_s.into_boxed_str()), pos.clone()));
                    }

                    ops::Instruction::new(ops::Operator::Word, None, Some(token), pos)
                }
                Mode::Define => {
                    ctx.def.insert(token.clone(), None);
                    state = Mode::Normal;
                    ops::Instruction::new(ops::Operator::Word, None, Some(token), pos)
                }
                Mode::Function => {
                    ctx.def.insert(token.clone(), None);
                    state = Mode::Normal;
                    ops::Instruction::new(ops::Operator::Word, None, Some(token), pos)
                },
            },
        });

        i += 1
    }

    // for x in &parsed_prg {
    //     println!("{:?}", x);
    // }

    Ok(parsed_prg)
}

pub fn parse_char(x: &str) -> Vec<ops::Value> {
    let quoted = unescape(x).unwrap();
    let unescaped_x = quoted.chars();
    let mut res = vec![];
    for x in unescaped_x {
        res.push(ops::Value::Char(x))
    }
    res
}
