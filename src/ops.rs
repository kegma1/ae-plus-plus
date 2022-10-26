#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(usize), // index in str_heap
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Int(x) => write!(f, "{}", x),
            Type::Float(x) => write!(f, "{}", x),
            Type::Bool(x) => write!(f, "{}", x),
            Type::Str(x) => write!(f, "{}", x),
        }
    }
}

pub type Pos = (usize, usize, String);
pub type Lexeme = (Operator, Pos);
pub type Ptr = usize;
// type Argument = Option<Ptr | Value | Type>;
// // type Value = (Type, Any);
// type Instruction = (Operator, Argument);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Literal(Type),

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

    If(Option<Ptr>),
    Else(Option<Ptr>),
    End(Option<Ptr>),
    Do(Option<Ptr>),
    While,

    Dup,
    Drop,
    Swap,
    // Null
}

#[derive(Debug, PartialEq)]
pub struct Runtime {
    pub stack: Vec<Type>,
    pub str_heap: Vec<String>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            stack: vec![],
            str_heap: vec![],
        }
    }
}
