use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinkedListError{
    #[error("Index '{index}' out of bounds")]
    OutOfBounds { index:usize }
}