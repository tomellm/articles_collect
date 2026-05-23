#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("internal db err {0}")]
    DbInternal(String),
}

impl StorageError {
    pub fn db_int_err(err: impl std::error::Error) -> Self {
        Self::DbInternal(err.to_string())
    }
}
