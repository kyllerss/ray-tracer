use crate::domain::object::{Object, Triangle};
use crate::domain::Point;
use nom::IResult;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

pub struct ObjParseResult<'a> {
    vertices: Vec<Point>,
    pub objects: HashMap<String, Vec<Object<'a>>>,
    pub skipped: usize,
}

impl<'a> Default for ObjParseResult<'a> {
    fn default() -> Self {
        ObjParseResult {
            vertices: Vec::new(),
            objects: HashMap::from([(ObjParseResult::DEFAULT_GROUP_NAME.to_string(), Vec::new())]),
            skipped: 0,
        }
    }
}

impl<'a> ObjParseResult<'a> {
    const DEFAULT_GROUP_NAME: &'static str = "__DEFAULT__";

    // vertices accessor - 1-based indexing
    pub fn vertex(&self, index: usize) -> Option<&Point> {
        self.vertices.get(index - 1)
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn default_group(&self) -> &Vec<Object<'a>> {
        self.objects
            .get(ObjParseResult::DEFAULT_GROUP_NAME.into())
            .unwrap()
    }

    pub fn add_to_default_group(&mut self, obj: Object<'a>) {
        self.objects
            .get_mut(ObjParseResult::DEFAULT_GROUP_NAME)
            .unwrap()
            .push(obj);
    }

    pub fn named_group(&self, group_name: &String) -> Option<&Vec<Object<'a>>> {
        self.objects.get(group_name)
    }

    pub fn add_to_named_group(&mut self, group_name: &String, obj: Object<'a>) {
        self.objects
            .entry(group_name.clone())
            .or_default()
            .push(obj);
    }
}

fn vertex(input: &str) -> IResult<&str, Point> {
    // remove leading whitespace
    let (r, _) = nom::character::complete::space0(input)?;

    nom::sequence::tuple((
        nom::bytes::complete::tag("v"),
        nom::character::complete::char(' '),
        nom::number::complete::double,
        nom::character::complete::char(' '),
        nom::number::complete::double,
        nom::character::complete::char(' '),
        nom::number::complete::double,
    ))(r)
    .map(|(remainder, (_, _, x, _, y, _, z))| (remainder, Point::new(x, y, z)))
}

fn face<'a, 'i>(
    input: &'i str,
    parse_result: &ObjParseResult<'a>,
) -> IResult<&'i str, Vec<Triangle>> {
    // remove leading whitespace
    let (r, _) = nom::character::complete::space0(input)?;
    let (r, _) = nom::bytes::complete::tag("f")(r)?;
    let (r, _) = nom::character::complete::space0(r)?;

    let r = nom::multi::separated_list1(
        nom::character::complete::char(' '),
        nom::character::complete::digit1,
    )(r);
    let (remainder, vertices) = r?;

    // fan triangles
    let mut triangles = vec![];
    for index in 1..(vertices.len() - 1) {
        let v1_index = 1;
        let v2_index = vertices[index].parse::<usize>().unwrap();
        let v3_index = vertices[index + 1].parse::<usize>().unwrap();

        let p1 = parse_result.vertex(v1_index);
        let p2 = parse_result.vertex(v2_index);
        let p3 = parse_result.vertex(v3_index);

        if p1.is_none() || p2.is_none() || p3.is_none() {
            panic!("Reference to non-existent vertix.");
        }

        let t = Triangle::new(
            p1.unwrap().clone(),
            p2.unwrap().clone(),
            p3.unwrap().clone(),
        );
        triangles.push(t);
    }

    IResult::Ok((remainder, triangles))
}

fn group(input: &str) -> IResult<&str, String> {
    // remove leading whitespace
    let (r, _) = nom::character::complete::space0(input)?;

    nom::sequence::tuple((
        nom::bytes::complete::tag("g"),
        nom::character::complete::char(' '),
        nom::character::complete::alphanumeric1,
    ))(r)
    .map(|(remainder, (_, _, group_name))| (remainder, group_name.into()))
}

// Given obj-formatted content, returns Vec of corresponding Objects
pub fn parse_obj_file<'a>(contents: &str) -> Option<ObjParseResult<'a>> {
    let reader = BufReader::new(contents.as_bytes());

    let mut to_return = ObjParseResult::default();

    let mut current_group: Option<String> = Option::None;
    for line in reader.lines() {
        if line.is_err() {
            return Option::None;
        }

        let l = line.unwrap();

        if let IResult::Ok((_, vertex)) = vertex(l.as_ref()) {
            to_return.vertices.push(vertex);
        } else if let IResult::Ok((_, triangles)) = face(l.as_ref(), &to_return) {
            for t in triangles {
                if let Option::Some(group_name) = &current_group {
                    to_return.add_to_named_group(group_name, t.into());
                } else {
                    to_return.add_to_default_group(t.into());
                }
            }
        } else if let IResult::Ok((_, group_name)) = group(l.as_ref()) {
            current_group = Option::Some(group_name);
        } else {
            to_return.skipped += 1;
        }
    }

    Option::Some(to_return)
}
