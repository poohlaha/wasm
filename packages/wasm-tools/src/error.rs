use thiserror::Error;

#[derive(Error, Debug)]
pub enum WasmError {
    #[error("`{0}` is empty !")]
    Empty(String),

    #[error("{0}")]
    Error(String),
}
