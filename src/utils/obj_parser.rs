use crate::domain::object::Object;
use crate::domain::Point;
use nom::bytes::complete::tag;
use nom::error::{Error, VerboseError};
use nom::number::complete::{double, float};
use nom::sequence::tuple;
use nom::{Err, IResult, Parser};
use std::io::{BufRead, BufReader};

pub struct ObjParseResult<'a> {
    vertices: Vec<Point>,
    pub objects: Vec<Object<'a>>,
    pub skipped: usize,
}

impl<'a> ObjParseResult<'a> {
    // vertices accessor - 1-based indexing
    pub fn vertex(&self, index: usize) -> Option<&Point> {
        self.vertices.get(index - 1)
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
}

fn vertex(input: &str) -> IResult<&str, Point> {
    // remove leading whitespace
    let (r, _) = nom::character::complete::space0(input)?;

    let (remainder, (_, _, x, _, y, _, z)) = tuple((
        tag("v"),
        nom::character::complete::char(' '),
        nom::number::complete::double,
        nom::character::complete::char(' '),
        nom::number::complete::double,
        nom::character::complete::char(' '),
        nom::number::complete::double,
    ))(r)?;

    IResult::Ok((remainder, Point::new(x, y, z)))
}

// Given obj-formatted content, returns Vec of corresponding Objects
pub fn parse_obj_file<'a>(contents: &str) -> Option<ObjParseResult<'a>> {
    let reader = BufReader::new(contents.as_bytes());

    let mut to_return = ObjParseResult {
        vertices: Vec::new(),
        objects: Vec::new(),
        skipped: 0,
    };

    for line in reader.lines() {
        if line.is_err() {
            return Option::None;
        }

        let l = line.unwrap();

        // vertex parsing
        let v = vertex(l.as_ref());

        if let IResult::Ok((_, vertex)) = v {
            to_return.vertices.push(vertex);
        } else {
            to_return.skipped += 1;
        }

        // triangle parsing
    }

    Option::Some(to_return)
}
