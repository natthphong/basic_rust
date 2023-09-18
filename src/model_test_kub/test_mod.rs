pub trait Speaking{
    fn speak(&self);
}


#[derive(Debug)]
pub enum GradeResult{
    Value(String),
    Error(String)
}