use crate::{ops, Runtime};
use snailquote::unescape;

pub fn parse(
    prg: Vec<(String, ops::Pos)>,
    ctx: &mut Runtime,
) -> Result<Vec<ops::Instruction>, (&'static str, ops::Pos)> {
    // println!("{:?}",prg);

    let mut parsed_prg: Vec<ops::Instruction> = vec![];

    let unwraped_prg = prg;
    let mut i = 0;
    let prg_len = unwraped_prg.len();
    while i < prg_len {
        let (token, pos) = unwraped_prg[i].clone();
        // println!("{}", i);
        match parse_token(token.clone(), pos.clone(), ctx, &mut parsed_prg) {
            Some(instruct) => parsed_prg.push(instruct),
            None => ()
        }

        i += 1
    }

    // for x in &parsed_prg {
    //     println!("{:?}", x);
    // }

    Ok(parsed_prg)
}

fn parse_token(token: String, pos: (usize, usize, String), ctx: &mut Runtime, parsed_prg: &mut Vec<ops::Instruction>) -> Option<ops::Instruction> {
    Some(match token.as_str() {
        "skriv" => ops::Instruction::new(ops::Operator::Print, None, None, pos),
        "skrivnl" => ops::Instruction::new(ops::Operator::PrintLn, None, None, pos),
        "spør" => ops::Instruction::new(ops::Operator::Input, None, None, pos),
        "feilsøk" => ops::Instruction::new(ops::Operator::Debug, None, None, pos),
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
        "ellvis" => ops::Instruction::new(ops::Operator::Elif, None, None, pos),
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
        "konst" => ops::Instruction::new(ops::Operator::Const, None, None, pos),
        "minne" => ops::Instruction::new(ops::Operator::Mem, None, None, pos),
        "funk" => ops::Instruction::new(ops::Operator::Func, None, None, pos),
        "let" => ops::Instruction::new(ops::Operator::Let, None, None, pos),
        "=" => ops::Instruction::new(ops::Operator::Eq, None, None, pos),
        ">" => ops::Instruction::new(ops::Operator::Gt, None, None, pos),
        ">=" => ops::Instruction::new(ops::Operator::Ge, None, None, pos),
        "<" => ops::Instruction::new(ops::Operator::Lt, None, None, pos),
        "<=" => ops::Instruction::new(ops::Operator::Le, None, None, pos),
        "@" => ops::Instruction::new(ops::Operator::Read, None, None, pos),
        "->" => ops::Instruction::new(ops::Operator::Write, None, None, pos),

        "Helt" => ops::Instruction::new(
            ops::Operator::Literal,
            Some(ops::Value::TypeLiteral(ops::TypeLiteral::Int)),
            None,
            pos,
        ),
        "Bool" => ops::Instruction::new(
            ops::Operator::Literal,
            Some(ops::Value::TypeLiteral(ops::TypeLiteral::Bool)),
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
        "Pek" => ops::Instruction::new(
            ops::Operator::Literal,
            Some(ops::Value::TypeLiteral(ops::TypeLiteral::Ptr)),
            None,
            pos,
        ),
        "sann" => ops::Instruction::new(
            ops::Operator::Literal,
            Some(ops::Value::Bool(true)),
            None,
            pos,
        ),
        "usann" => ops::Instruction::new(
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
                ops::Instruction::new(
                    ops::Operator::Literal,
                    Some(unescaped_x[0].clone()),
                    None,
                    pos,
                )
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
        "" => {
            None?
        }
        _ => {
            let mut word = parse_word(token, pos, ctx);
            parsed_prg.append(&mut word);
            None?
            // ops::Instruction::new(ops::Operator::Word, None, Some(token), pos)
        }
    })
}

fn parse_word(word: String, pos: (usize, usize, String), ctx: &mut Runtime) -> Vec<ops::Instruction> {
    let mut indexing = vec![];
    let mut res = vec![];
    let mut i = 0;
    let mut rw: Option<ops::Instruction> = None;
    'outer: while i < word.len() {
        match word.as_bytes()[i] as char {
            '-' => {
               if word.as_bytes()[i + 1] as char == '>' {
                    if let None = rw {
                        rw = Some(ops::Instruction::new(ops::Operator::Write, None, None, pos.clone()));
                        i += 1
                    }
               }
            }
            '@' => {
                if let None = rw {
                    rw = Some(ops::Instruction::new(ops::Operator::Read, None, None, pos.clone()))
                }
            },
            '[' => {
                let mut index: String = String::from("");
                for c in word.as_str()[i + 1..].chars() {
                    if c == ']' {
                        match parse_token(index.clone(), pos.clone(), ctx, &mut indexing) {
                            Some(instruct) => {
                                indexing.push(instruct);
                                indexing.push(ops::Instruction::new(ops::Operator::Add, None, None, pos.clone()));
                            },
                            None => indexing.push(ops::Instruction::new(ops::Operator::Add, None, None, pos.clone()))
                        }
                        i += index.len() + 1;
                        break;
                    } else {
                        index = format!("{}{}", index, c)
                    }
                }
            }
            _ => {
                res.push(ops::Instruction::new(ops::Operator::Word, None, Some(word.as_str()[i..].to_string()), pos.clone()));
                if !indexing.is_empty() {
                    res.append(&mut indexing)
                }

                if let Some(r_or_w) = rw {
                    res.push(r_or_w)
                }
                // res.push(ops::Instruction::new(ops::Operator::Word, None, Some(word.as_str()[i..].to_string()), pos.clone())); 
                break 'outer;
            }
        }
        i += 1
    }

    res
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
