use crate::ops;
use snailquote::unescape;
// https://github.com/ttm/tokipona/blob/master/data/toki-pona_english.txt

pub fn parse(
    prg: Result<Vec<(String, ops::Pos)>, &'static str>,
    ctx: &mut ops::Runtime,
) -> Result<Vec<ops::Lexeme>, &'static str> {
    // println!("{:?}",prg);
    if let Err(e) = prg {
        return Err(e);
    }

    let mut parsed_prg: Vec<ops::Lexeme> = vec![];

    let unwraped_prg = prg.unwrap();
    let mut i = 0;
    let prg_len = unwraped_prg.len();
    while i < prg_len {
        let (token, pos) = unwraped_prg[i].clone();
        // println!("{}", i);
        parsed_prg.push((
            match token.as_str() {
                "toki" => ops::Operator::Print,
                "o!" => ops::Operator::Input,
                "+" => ops::Operator::Add,
                "-" => ops::Operator::Sub,
                "*" => ops::Operator::Mult,
                "/" => ops::Operator::Div,
                "ike" => ops::Operator::Not,
                "en" => ops::Operator::And,
                "anu" => ops::Operator::Or,
                "lon" => ops::Operator::Literal(ops::Type::Bool(true)),
                "la" => ops::Operator::If(None),
                "ante-la" => ops::Operator::Else(None),
                "pini" => ops::Operator::End(None),
                "pali" => ops::Operator::Do(None),
                "tenpo" => ops::Operator::While,
                "sin" => ops::Operator::Dup,
                "pakala" => ops::Operator::Drop,
                "esun" => ops::Operator::Swap,
                "=" => ops::Operator::Eq,
                ">" => ops::Operator::Lt,
                ">=" => ops::Operator::Le,
                "<" => ops::Operator::Gt,
                "<=" => ops::Operator::Ge,
                x if x.parse::<i32>().is_ok() => {
                    ops::Operator::Literal(ops::Type::Int(x.parse::<i32>().unwrap()))
                }
                x if x.parse::<f32>().is_ok() => {
                    ops::Operator::Literal(ops::Type::Float(x.parse::<f32>().unwrap()))
                }
                x if x.chars().nth(0) == Some('"') => {
                    let unescaped_x = unescape(x).unwrap();
                    ctx.str_heap.push(unescaped_x);
                    let i = ctx.str_heap.len() - 1;
                    ops::Operator::Literal(ops::Type::Str(i))
                }
                _ => {
                    i += 1;
                    continue;
                }
            },
            pos,
        ));

        i += 1
    }

    Ok(parsed_prg)
}
