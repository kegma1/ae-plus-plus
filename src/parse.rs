use crate::{ops, Runtime};
use snailquote::unescape;
// https://github.com/ttm/tokipona/blob/master/data/toki-pona_english.txt

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
        parsed_prg.push(match token.as_str() {
            "si" => ops::Instruction::new(ops::Operator::Print, None, pos),
            "spør" => ops::Instruction::new(ops::Operator::Input, None, pos),
            "+" => ops::Instruction::new(ops::Operator::Add, None, pos),
            "-" => ops::Instruction::new(ops::Operator::Sub, None, pos),
            "*" => ops::Instruction::new(ops::Operator::Mult, None, pos),
            "/" => ops::Instruction::new(ops::Operator::Div, None, pos),
            "ikke" => ops::Instruction::new(ops::Operator::Not, None, pos),
            "og" => ops::Instruction::new(ops::Operator::And, None, pos),
            "eller" => ops::Instruction::new(ops::Operator::Or, None, pos),
            "hvis" => ops::Instruction::new(ops::Operator::If, None, pos),
            "ellers" => ops::Instruction::new(ops::Operator::Else, None, pos),
            "slutt" => ops::Instruction::new(ops::Operator::End, None, pos),
            "gjør" => ops::Instruction::new(ops::Operator::Do, None, pos),
            "mens" => ops::Instruction::new(ops::Operator::While, None, pos),
            "dup" => ops::Instruction::new(ops::Operator::Dup, None, pos),
            "rot" => ops::Instruction::new(ops::Operator::Rot, None, pos),
            "over" => ops::Instruction::new(ops::Operator::Over, None, pos),
            "slipp" => ops::Instruction::new(ops::Operator::Drop, None, pos),
            "snu" => ops::Instruction::new(ops::Operator::Swap, None, pos),
            "omgjør" => ops::Instruction::new(ops::Operator::Cast, None, pos),
            "=" => ops::Instruction::new(ops::Operator::Eq, None, pos),
            ">" => ops::Instruction::new(ops::Operator::Gt, None, pos),
            ">=" => ops::Instruction::new(ops::Operator::Ge, None, pos),
            "<" => ops::Instruction::new(ops::Operator::Lt, None, pos),
            "<=" => ops::Instruction::new(ops::Operator::Le, None, pos),
            "," => ops::Instruction::new(ops::Operator::Read, None, pos),
            "." => ops::Instruction::new(ops::Operator::Write, None, pos),

            "heltall" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Int)),
                pos,
            ),
            "flyttall" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Float)),
                pos,
            ),
            "streng" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Str)),
                pos,
            ),
            "sann" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::Bool(true)),
                pos,
            ),
            "usann" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::Bool(false)),
                pos,
            ),
            x if x.parse::<i32>().is_ok() => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::Int(x.parse::<i32>().unwrap())),
                pos,
            ),
            x if x.parse::<f32>().is_ok() => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::Float(x.parse::<f32>().unwrap())),
                pos,
            ),
            x if x.chars().nth(0) == Some('"') => {
                let unescaped_x = unescape(x).unwrap();
                ctx.str_heap.push(unescaped_x);
                let i = (ctx.str_heap.len() - 1) as usize;
                ops::Instruction::new(
                    ops::Operator::Literal,
                    Some(ops::Value::Str(i)),
                    pos,
                )
            }
            "" => {
                i += 1;
                continue;
            }
            _ => {
                let err_s: String = format!("ukjent ord '{}'", token).to_owned();
                return Err((Box::leak(err_s.into_boxed_str()), pos.clone()));
            }
        });

        i += 1
    }

    Ok(parsed_prg)
}
