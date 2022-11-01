use crate::ops;

pub fn cross_reference(
    prg: &mut Vec<ops::Instruction>,
    _ctx: &crate::Runtime,
) -> Result<Vec<ops::Instruction>, (&'static str, ops::Pos)> {
    let mut stack: Vec<usize> = vec![];
    for i in 0..prg.len() {
        let token = prg[i].op.clone();
        match token {
            ops::Operator::If => stack.push(i),
            ops::Operator::While => stack.push(i),
            ops::Operator::Const => stack.push(i),
            ops::Operator::Else => {
                let if_i = stack.pop().unwrap();
                if prg[if_i].op == ops::Operator::If {
                    prg[if_i].arg = Some(i);
                    stack.push(i)
                } else {
                    return Err((
                        "'ellers' kan bare stenge 'hvis' blokker",
                        prg[if_i].pos.clone(),
                    ));
                }
            }
            ops::Operator::End => {
                let block_i = stack.pop().unwrap();

                if prg[block_i].op == ops::Operator::If {
                    prg[block_i].arg = Some(i);
                } else if prg[block_i].op == ops::Operator::Else {
                    prg[block_i].arg = Some(i);
                } else if prg[block_i].op == ops::Operator::Do {
                    prg[i].arg = prg[block_i].arg;
                    prg[block_i].arg = Some(i);
                } else if prg[block_i].op == ops::Operator::Const {
                    prg[i].arg = Some(block_i);
                }
            }
            ops::Operator::Do => {
                let while_i = stack.pop().unwrap();
                prg[i].arg = Some(while_i);
                stack.push(i)
            }
            _ => (),
        }
    }
    // for (i, inst) in prg.iter().enumerate() {
    //     println!("{}: {}", i, inst)
    // }
    if stack.len() > 0 {
        return Err(("ikke stengt blokk", prg[stack.pop().unwrap()].pos.clone()));
    }

    Ok(prg.clone())
}
