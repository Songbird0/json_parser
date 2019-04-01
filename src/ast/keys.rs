use std::fmt;

/// Represents a json object key.
///
/// A key cannot be anything other than a string or number,
/// so we need to create concrete type to implement these cases.
pub trait Key {}

pub struct KeyString {
    data: String
}

impl Key for KeyString {}

impl fmt::Display for KeyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

pub struct KeyI64 {
    data: i64
}

impl Key for KeyI64 {}

impl fmt::Display for KeyI64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
