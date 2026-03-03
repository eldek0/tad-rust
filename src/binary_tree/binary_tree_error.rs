use thiserror::Error;

#[derive(Error, Debug)]
pub enum BinaryTreeError<K>{
    #[error("Key '{key}' does not exist")]
    KeyError { key:K },
    #[error("Key '{key}' already exist")]
    KeyExistError { key:K },
    #[error("Root already exist")]
    RootError,
    #[error("Parent with key '{parent_key}' does not exist")]
    ParentKeyError {parent_key:K},
    #[error("Parent with key {parent_key} does not have childs")]
    ParentWithNoChildsError {parent_key:K},
}