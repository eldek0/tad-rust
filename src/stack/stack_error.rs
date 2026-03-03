use thiserror::Error;

#[derive(Error, Debug)]
pub enum StackError{
    #[error("Empty stack")]
    EmptyStack
}