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
            "toki" => ops::Instruction::new(ops::Operator::Print, None, None, pos),
            "o!" => ops::Instruction::new(ops::Operator::Input, None, None, pos),
            "+" => ops::Instruction::new(ops::Operator::Add, None, None, pos),
            "-" => ops::Instruction::new(ops::Operator::Sub, None, None, pos),
            "*" => ops::Instruction::new(ops::Operator::Mult, None, None, pos),
            "/" => ops::Instruction::new(ops::Operator::Div, None, None, pos),
            "ike" => ops::Instruction::new(ops::Operator::Not, None, None, pos),
            "en" => ops::Instruction::new(ops::Operator::And, None, None, pos),
            "anu" => ops::Instruction::new(ops::Operator::Or, None, None, pos),
            "la" => ops::Instruction::new(ops::Operator::If, None, None, pos),
            "ante-la" => ops::Instruction::new(ops::Operator::Else, None, None, pos),
            "pini" => ops::Instruction::new(ops::Operator::End, None, None, pos),
            "pali" => ops::Instruction::new(ops::Operator::Do, None, None, pos),
            "tenpo" => ops::Instruction::new(ops::Operator::While, None, None, pos),
            "sin" => ops::Instruction::new(ops::Operator::Dup, None, None, pos),
            "pakala" => ops::Instruction::new(ops::Operator::Drop, None, None, pos),
            "esun" => ops::Instruction::new(ops::Operator::Swap, None, None, pos),
            "=" => ops::Instruction::new(ops::Operator::Eq, None, None, pos),
            ">" => ops::Instruction::new(ops::Operator::Lt, None, None, pos),
            ">=" => ops::Instruction::new(ops::Operator::Le, None, None, pos),
            "<" => ops::Instruction::new(ops::Operator::Gt, None, None, pos),
            "<=" => ops::Instruction::new(ops::Operator::Ge, None, None, pos),

            "lon" => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Type::Bool),
                Some([255; 4]),
                pos,
            ),
            x if x.parse::<i32>().is_ok() => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Type::Int),
                Some(x.parse::<i32>().unwrap().to_be_bytes()),
                pos,
            ),
            x if x.parse::<f32>().is_ok() => ops::Instruction::new(
                ops::Operator::Literal,
                Some(ops::Type::Float),
                Some(x.parse::<f32>().unwrap().to_be_bytes()),
                pos,
            ),
            x if x.chars().nth(0) == Some('"') => {
                let unescaped_x = unescape(x).unwrap();
                ctx.str_heap.push(unescaped_x);
                let i = (ctx.str_heap.len() - 1) as u32;
                ops::Instruction::new(
                    ops::Operator::Literal,
                    Some(ops::Type::Str),
                    Some(i.to_be_bytes()),
                    pos,
                )
            }
            _ => {
                i += 1;
                continue;
            }
        });

        i += 1
    }

    Ok(parsed_prg)
}
