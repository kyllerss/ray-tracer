use crate::domain::object::Object;
use nom::error::Error;

// Given obj-formatted content, returns Vec of corresponding Objects
pub fn parse_obj<'a>(contents: &str) -> Result<Vec<Object<'a>>, Error<&str>> {
    Result::Ok(vec![])
}
