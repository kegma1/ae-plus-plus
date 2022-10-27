use std::{any::Any, sync::Arc, fmt};

#[derive(Clone)]
pub struct Instruction {
    pub op: Operator,
    pub arg: Option<Ptr>,
    pub typ: Option<Type>,
    pub val: Option<Arc<dyn Any>>,
    pub pos: Pos,
}

impl Instruction {
    pub fn new(
        op: Operator,
        typ: Option<Type>,
        val: Option<Arc<dyn Any>>,
        pos: Pos,
    ) -> Self {
        Instruction {
            op: op,
            arg: None,
            typ: typ,
            val: val,
            pos: pos,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, ", self.op).unwrap();
        if let Some(x) = self.arg {
            write!(f, "{:?}, ", x).unwrap();
        }
        if let Some(x) = self.typ {
            write!(f, "{:?}, ", x).unwrap();
        }
        if let Some(x) = &self.val {
            write!(f, "{:?}, ", x).unwrap();
        }
        write!(f, "{:?}", self.pos)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str, // index in str_heap
}

pub type Pos = (usize, usize, String);
pub type Ptr = usize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Literal, // Value

    Add,
    Sub,
    Mult,
    Div,

    Print,
    Input,

    Not,
    And,
    Or,

    Eq,
    Lt,
    Le,
    Gt,
    Ge,

    If,   // Option<Ptr>
    Else, // Option<Ptr>
    End,  // Option<Ptr>
    Do,   // Option<Ptr>
    While,

    Dup,
    Drop,
    Swap,
    // Null
}
