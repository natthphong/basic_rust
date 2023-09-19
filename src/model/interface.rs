use std::fmt::{Display, Formatter};

pub trait Speaking{
    fn speak(&self);
}



pub enum GradeResult{
    Value(String),
    Error(String)
}

impl Display for GradeResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GradeResult::Value(value) => write!(f, "{}", value),
            GradeResult::Error(error) => write!(f, "{}", error),
        }
    }
}

