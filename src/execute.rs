use crate::{ops, Runtime};
// use snailquote::unescape;
// use std::io::{stdin, stdout, Write};



pub fn execute(ctx: &mut Runtime, prg: & Vec<ops::Instruction>) -> Result<u8, (&'static str, ops::Pos)> {
    let mut i = 0;
    while i < prg.len() {
        let token = &prg[i];

        match token.op {
            ops::Operator::Literal => {
                ctx.stack.push(ops::Value { typ: token.typ.unwrap(), val: token.val.unwrap()})
            },
            ops::Operator::Add => (),
            ops::Operator::Sub => (),
            ops::Operator::Mult => (),
            ops::Operator::Div => (),
            ops::Operator::Print => {
                if ctx.stack.len() < 1 {
                    return Err((
                        "'toki' requires 1 argument on the top of the stack",
                        token.pos.clone(),
                    ));   
                }
                let print_value = ctx.stack.pop().unwrap();

                match print_value.typ {
                    ops::Type::Int => println!("{}", parse_int(&print_value.val)),
                    ops::Type::Float => println!("{}", parse_float(&print_value.val)),
                    ops::Type::Bool => println!("{}", parse_bool(&print_value.val)),
                    ops::Type::Str => print!("{}", ctx.str_heap[parse_ptr(&print_value.val)]),
                }
                
            },
            ops::Operator::Input => (),
            ops::Operator::Not => (),
            ops::Operator::And => (),
            ops::Operator::Or => (),
            ops::Operator::Eq => (),
            ops::Operator::Lt => (),
            ops::Operator::Le => (),
            ops::Operator::Gt => (),
            ops::Operator::Ge => (),
            ops::Operator::If => (),
            ops::Operator::Else => (),
            ops::Operator::End => (),
            ops::Operator::Do => (),
            ops::Operator::While => (),
            ops::Operator::Dup => (),
            ops::Operator::Drop => (),
            ops::Operator::Swap => (),
        }

        i += 1;
    }

    Ok(0)
}

fn parse_bool(val: &[u8; 4]) -> bool { if val == &[255; 4] { true } else { false }}
fn parse_ptr(val: &[u8; 4]) -> usize { u32::from_be_bytes(*val) as usize}
fn parse_int(val: &[u8; 4]) -> i32 {i32::from_be_bytes(*val)}
fn parse_float(val: &[u8; 4]) -> f32 {f32::from_be_bytes(*val)}
