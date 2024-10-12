use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum LoaderError {
    #[error("File not found: {file_path}")]
    FileNotFound { file_path: String },
    #[error("Too Many Options: you can choose option -e or -f, not both of them")]
    TooManyOptions,
    #[error("Too Few Options: you should use option -e or -f to read source code")]
    TooFewOptions,
}
