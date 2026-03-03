use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueueError{
    #[error("Empty queue")]
    EmptyQueue
}