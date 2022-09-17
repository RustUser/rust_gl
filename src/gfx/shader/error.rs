use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Error {
    CompilationError(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CompilationError(error) => {
                f.write_fmt(format_args!("Formatting error: {}", error))
            }
        }
    }
}

impl std::error::Error for Error {

}