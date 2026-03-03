use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashMapError<K>{
    #[error("Value with key '{key}' does not exist")]
    KeyError { key:K }
}