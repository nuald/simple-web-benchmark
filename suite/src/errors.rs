use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ProcessError {
    code: Option<i32>,
}

impl ProcessError {
    pub fn new(code: Option<i32>) -> ProcessError {
        ProcessError { code }
    }
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.code)
    }
}

impl Error for ProcessError {}

#[derive(Debug)]
pub struct PidError {}

impl fmt::Display for PidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pid file not found")
    }
}

impl Error for PidError {}

#[derive(Debug)]
pub struct UnknownFileTypeError {}

impl fmt::Display for UnknownFileTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown file type")
    }
}

impl Error for UnknownFileTypeError {}
