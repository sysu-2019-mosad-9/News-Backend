#[derive(Debug)]
pub enum ServerError {
    StrError(&'static str),
}

impl From<&'static str> for ServerError {
    fn from(err: &'static str) -> Self {
        ServerError::StrError(err)
    }
}
