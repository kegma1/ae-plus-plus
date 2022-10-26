use std::env;

mod execute;
mod lex;
mod ops;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut ctx = ops::Runtime::new();

    let lexed = lex::lex(path);
    let mut parsed = parse::parse(lexed, &mut ctx).unwrap();
    //    println!("{:?}", cross_reference(&mut parsed).unwrap());
    let res = execute::execute(&mut ctx, cross_reference(&mut parsed).unwrap());
    if let Err(e) = res {
        println!("ERROR: {}, {}:{}:{}", e.0, e.1 .0, e.1 .1, e.1 .2)
    }
}

fn cross_reference(
    prg: &mut Vec<ops::Lexeme>,
) -> Result<Vec<ops::Lexeme>, (&'static str, ops::Pos)> {
    let mut stack: Vec<usize> = vec![];
    for i in 0..prg.len() {
        let token = prg[i].0.clone();
        match token {
            ops::Operator::If(_) => stack.push(i),
            ops::Operator::While => stack.push(i),
            ops::Operator::Else(_) => {
                let if_i = stack.pop().unwrap();
                if let ops::Operator::If(_) = prg[if_i].0 {
                    prg[if_i].0 = ops::Operator::If(Some(i + 1));
                    stack.push(i)
                } else {
                    return Err(("'ante-la' can only close 'la' blocks", prg[if_i].1.clone()));
                }
            }
            ops::Operator::End(_) => {
                let block_i = stack.pop().unwrap();

                if prg[block_i].0 == ops::Operator::If(None) {
                    prg[block_i].0 = ops::Operator::If(Some(i));
                } else if prg[block_i].0 == ops::Operator::Else(None) {
                    prg[block_i].0 = ops::Operator::Else(Some(i));
                } else if let ops::Operator::Do(Some(while_i)) = prg[block_i].0 {
                    prg[i].0 = ops::Operator::End(Some(while_i));
                    prg[block_i].0 = ops::Operator::Do(Some(i));
                }
            }
            ops::Operator::Do(_) => {
                let while_i = stack.pop().unwrap();
                prg[i].0 = ops::Operator::Do(Some(while_i));
                stack.push(i)
            }
            _ => (),
        }
    }

    Ok(prg.clone())
}
