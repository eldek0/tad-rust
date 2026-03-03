use thiserror::Error;

#[derive(Error, Debug)]
pub enum HeapError{
    #[error("Empty Heap")]
    EmptyHeap
}