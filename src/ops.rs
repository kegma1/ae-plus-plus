use std::fmt;

#[derive(Clone, Debug)]
pub struct Instruction {
    pub op: Operator,
    pub arg: Option<Ptr>,
    pub val: Option<Value>,
    pub name: Option<String>,
    pub pos: Pos,
}

impl Instruction {
    pub fn new(op: Operator, val: Option<Value>, name: Option<String>, pos: Pos) -> Self {
        Instruction {
            op,
            arg: None,
            val,
            name,
            pos,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, ", self.op).unwrap();
        if let Some(x) = self.arg {
            write!(f, "{:?}, ", x).unwrap();
        }
        if let Some(x) = self.val {
            write!(f, "{:?}, ", x).unwrap();
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(Ptr), // index in str_heap
    Ptr(Ptr),
    TypeLiteral(TypeLiteral),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(_) => write!(f, "Heltall"),
            Value::Float(_) => write!(f, "Flyttall"),
            Value::Bool(_) => write!(f, "Bool"),
            Value::Str(_) => write!(f, "Str"),
            Value::TypeLiteral(_) => write!(f, "TypeLitr"),
            Value::Ptr(_) => write!(f, "Peker"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeLiteral {
    Int,
    Float,
    Bool,
    Str,
}

pub type Pos = (usize, usize, String);
pub type Ptr = usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Literal, // Value
    Const,

    Word,

    Add,
    Sub,
    Mult,
    Div,

    Cast,

    Print,
    Input,

    Read,  // not yet implemented
    Write, // not yet implemented

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
    Over,
    Rot,
    // Null
}