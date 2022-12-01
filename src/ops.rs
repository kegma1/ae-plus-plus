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
            write!(f, "{}, ", x).unwrap();
        }
        if let Some(x) = &self.val {
            write!(f, "{}, ", x).unwrap();
        }
        if let Some(x) = &self.name {
            write!(f, "{}, ", x).unwrap();
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    Str((Ptr, usize)), // Pointer to data in memory, size of string
    Byte(u8),          //ikke implementert
    Char(char),
    Ptr((Ptr, usize, TypeLiteral)), // ikke implementert

    TypeLiteral(TypeLiteral),
    FuncPtr(FuncPtr),
    Null,
}

impl Value {
    pub fn eq(&self, typ: &TypeLiteral) -> bool {
        match self {
            Value::Int(_) => typ == &TypeLiteral::Int,
            Value::Float(_) => typ == &TypeLiteral::Float,
            Value::Bool(_) => typ == &TypeLiteral::Bool,
            Value::Str(_) => typ == &TypeLiteral::Str,
            Value::Byte(_) => typ == &TypeLiteral::Byte,
            Value::Char(_) => typ == &TypeLiteral::Char,
            Value::Ptr(_) => typ == &TypeLiteral::Ptr,
            _ => false,
        }
    }

    pub fn to_string(&self, ctx: &crate::Runtime) -> String {
        match self {
            Value::Int(x) => x.to_string(),
            Value::Float(x) => x.to_string(),
            Value::Bool(x) => {
                if *x {
                    String::from("sann")
                } else {
                    String::from("usann")
                }
            }
            Value::Str(_) => ctx.read_str(self).unwrap(),
            Value::Byte(x) => String::from(format!("{:#02x}", x)),
            Value::Char(x) => x.to_string(),
            Value::Ptr((ptr, len, typ)) => String::from(format!("[{}; {}] -> {}", typ, len, ptr)),
            Value::TypeLiteral(x) => String::from(format!("{}", x)),
            Value::Null => String::from("null"),
            _ => String::from("Kan ikke skrives"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(_) => write!(f, "Helt"),
            Value::Float(_) => write!(f, "Flyt"),
            Value::Bool(_) => write!(f, "Bool"),
            Value::Str(_) => write!(f, "Str"),
            Value::TypeLiteral(_) => write!(f, "TypeLitr"),
            Value::Ptr(_) => write!(f, "Peker"),
            Value::Byte(_) => write!(f, "Byte"),
            Value::Char(_) => write!(f, "Bokst"),
            Value::Null => write!(f, "Null"),
            _ => write!(f, ""),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeLiteral {
    Int,
    Float,
    Bool,
    Str,
    Byte,
    Char,
    Ptr,
}

impl fmt::Display for TypeLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeLiteral::Int => write!(f, "Helt"),
            TypeLiteral::Float => write!(f, "Flyt"),
            TypeLiteral::Bool => write!(f, "Bool"),
            TypeLiteral::Str => write!(f, "Str"),
            TypeLiteral::Ptr => write!(f, "Peker"),
            TypeLiteral::Byte => write!(f, "Byte"),
            TypeLiteral::Char => write!(f, "Bokst"),
        }
    }
}

pub type Pos = (usize, usize, String);
pub type Ptr = usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Literal, // Value
    Const,
    Mem,
    Func,
    Let,

    Word,
    BikeShed,

    Add,
    Sub,
    Mult,
    Div,

    Cast,

    Print,
    PrintLn,
    Input,
    Debug,

    Read,
    Write,
    Exit,

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
    Elif, // Option<Ptr>
    End,  // Option<Ptr>
    Do,   // Option<Ptr>
    In,   //  Option<Ptr>
    While,

    Dup,
    Drop,
    Swap,
    Over,
    Rot,
    // Null
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncPtr {
    pub ptr: usize,
    pub params: Vec<TypeLiteral>,
    pub returns: Vec<TypeLiteral>,
}

// skal gjøre pekere bedre senere.
// #[warn(dead_code)]
// pub struct Pointer {
//     pub ptr: usize,
//     pub len: usize,
//     pub typ: TypeLiteral,
//     max_ptr: usize,
//     min_ptr: usize
// }
// #[warn(dead_code)]
// impl Pointer {
//     pub fn new(ptr: usize, len: usize, typ: TypeLiteral,) -> Self {
//         Pointer {
//             ptr,
//             len,
//             typ,
//             min_ptr: ptr,
//             max_ptr: ptr + len
//         }
//     }

//     pub fn inc(&mut self, offset:usize) -> Option<&mut Self> {
//         let new_ptr = self.ptr + offset;
//         if new_ptr > self.max_ptr {
//             return None;
//         }
//         self.ptr = new_ptr;
//         Some(self)
//     }

//     pub fn dec(&mut self, offset:usize) -> Option<&mut Self> {
//         let new_ptr = self.ptr - offset;
//         if new_ptr < self.min_ptr {
//             return None;
//         }
//         self.ptr = new_ptr;
//         Some(self)
//     }
// }
