use std::fmt;

pub struct KhaiiiError {
    error: String
}

impl KhaiiiError {
    pub fn new(error: String) -> KhaiiiError {
        return KhaiiiError {
            error,
        }
    }
}

impl fmt::Display for KhaiiiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl fmt::Debug for KhaiiiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}