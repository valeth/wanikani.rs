use serde_json::Error as JSONError;

#[derive(Debug)]
pub enum Error {
    JSONError(JSONError),
    RetrievalError
}

impl From<JSONError> for Error {
    fn from(error: JSONError) -> Self {
        Error::JSONError(error)
    }
}
