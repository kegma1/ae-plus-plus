use std::fmt;

#[derive(Clone)]
pub struct Instruction {
    pub op: Operator,
    pub arg: Option<Ptr>,
    pub val: Option<Value>,
    pub pos: Pos,
}

impl Instruction {
    pub fn new(
        op: Operator,
        val: Option<Value>,
        pos: Pos,
    ) -> Self {
        Instruction {
            op,
            arg: None,
            val,
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
        write!(f, "{:?}", self.pos)
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(Ptr), // index in str_heap
    TypeLiteral(TypeLiteral),
}

impl fmt::Display for Value {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
        Value::Int(_) => write!(f, "Int"),
        Value::Float(_) => write!(f, "Float"),
        Value::Bool(_) => write!(f, "Bool"),
        Value::Str(_) => write!(f, "Str"),
        Value::TypeLiteral(_) => write!(f, "TypeLitr"),
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
