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
            "toki" => ops::Instruction::new(ops::Operator::Print, None, pos),
            "o!" => ops::Instruction::new(ops::Operator::Input, None, pos),
            "+" => ops::Instruction::new(ops::Operator::Add, None, pos),
            "-" => ops::Instruction::new(ops::Operator::Sub, None, pos),
            "*" => ops::Instruction::new(ops::Operator::Mult, None, pos),
            "/" => ops::Instruction::new(ops::Operator::Div, None, pos),
            "ike" => ops::Instruction::new(ops::Operator::Not, None, pos),
            "en" => ops::Instruction::new(ops::Operator::And, None, pos),
            "anu" => ops::Instruction::new(ops::Operator::Or, None, pos),
            "la" => ops::Instruction::new(ops::Operator::If, None, pos),
            "ante-la" => ops::Instruction::new(ops::Operator::Else, None, pos),
            "pini" => ops::Instruction::new(ops::Operator::End, None, pos),
            "pali" => ops::Instruction::new(ops::Operator::Do, None, pos),
            "tenpo" => ops::Instruction::new(ops::Operator::While, None, pos),
            "sin" => ops::Instruction::new(ops::Operator::Dup, None, pos),
            "sike" => ops::Instruction::new(ops::Operator::Rot, None, pos),
            "sewi" => ops::Instruction::new(ops::Operator::Over, None, pos),
            "pakala" => ops::Instruction::new(ops::Operator::Drop, None, pos),
            "esun" => ops::Instruction::new(ops::Operator::Swap, None, pos),
            "kama" => ops::Instruction::new(ops::Operator::Cast, None, pos),
            "=" => ops::Instruction::new(ops::Operator::Eq, None, pos),
            ">" => ops::Instruction::new(ops::Operator::Gt, None, pos),
            ">=" => ops::Instruction::new(ops::Operator::Ge, None, pos),
            "<" => ops::Instruction::new(ops::Operator::Lt, None, pos),
            "<=" => ops::Instruction::new(ops::Operator::Le, None, pos),

            "nanpa" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Int)),
                pos,
            ),
            "nanpa-waso" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Float)),
                pos,
            ),
            "sitelen" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::TypeLiteral(ops::TypeLiteral::Str)),
                pos,
            ),
            "lon" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Value::Bool(true)),
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
                let err_s: String = format!("Unknown word '{}'", token).to_owned();
                return Err((Box::leak(err_s.into_boxed_str()), pos.clone()));
            }
        });

        i += 1
    }

    Ok(parsed_prg)
}
