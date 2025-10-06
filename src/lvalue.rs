use std::fmt;
use crate::vm::ExeState;

#[derive(Clone)]
pub enum LValue {
    Nil,
    String(String),
    Function(fn(&mut ExeState) -> i32),
}

impl fmt::Debug for LValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            LValue::Nil => write!(f, "nil"),
            LValue::String(s) => write!(f, "{s}"),
            LValue::Function(_) => write!(f, "function"),
        }
    }
}
