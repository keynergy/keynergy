use ketos::{Error, Value};
#[derive(Debug)]
pub enum ErrorType {
    Ketos(Error),
    ArgumentError(usize),
    ReturnError(Value),
}

#[derive(Debug)]
pub struct AnalysisError {
    pub metric: String,
    pub error: ErrorType,
}
