use crate::domain::object::Object;
use nom::error::Error;
use nom::Err;
use std::io::{BufRead, BufReader};

pub struct ObjParseResult<'a> {
    pub objects: Vec<Object<'a>>,
    pub skipped: usize,
}

// Given obj-formatted content, returns Vec of corresponding Objects
pub fn parse_obj<'a>(contents: &str) -> Result<ObjParseResult<'a>, Error<&str>> {
    let reader = BufReader::new(contents.as_bytes());

    let mut to_return = ObjParseResult {
        objects: Vec::new(),
        skipped: 0,
    };

    for line in reader.lines() {
        // Result<Vec<Object<'a>>, Error<&str>>

        to_return.skipped += 1;
    }

    Result::Ok(to_return)
}
