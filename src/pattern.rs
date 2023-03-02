
use std::str::FromStr;

// TODO ALL OF THIS FILE

#[derive(Debug)]
pub struct BytePattern {}

#[derive(Debug)]
pub struct StringPattern {}


impl FromStr for BytePattern {
    type Err = String;

    /// TODO
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO ADD CONFIG OPTIONS.
        Err("no options".to_owned())
    }
}

impl FromStr for StringPattern {
    type Err = String;

    /// TODO
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO ADD CONFIG OPTIONS.
        Err("no options".to_owned())
    }
}