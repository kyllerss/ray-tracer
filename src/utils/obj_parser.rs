use crate::domain::object::{Group, Object, SmoothTriangle, Triangle};
use crate::domain::{Point, Vector};
use nom::IResult;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

pub struct ObjParseResult<'a> {
    vertices: Vec<Point>,
    normals: Vec<Vector>,
    pub objects: HashMap<String, Vec<Object<'a>>>,
    pub skipped: usize,
}

impl<'a> Default for ObjParseResult<'a> {
    fn default() -> Self {
        ObjParseResult {
            vertices: Vec::new(),
            normals: Vec::new(),
            objects: HashMap::new(),
            skipped: 0,
        }
    }
}

impl<'a> From<ObjParseResult<'a>> for Box<Group<'a>> {
    fn from(parser: ObjParseResult<'a>) -> Self {
        let mut g = Group::builder();
        for group in parser.objects.values() {
            let mut inner_group = Group::builder();
            for obj in group {
                inner_group = inner_group.add_child(obj.clone());
            }

            g = g.add_child(inner_group.build().into());
        }

        g.build()
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

    pub fn normal(&self, index: usize) -> Option<&Vector> {
        self.normals.get(index - 1)
    }

    pub fn normal_count(&self) -> usize {
        self.normals.len()
    }

    pub fn default_group(&self) -> Option<&Vec<Object<'a>>> {
        self.objects.get(ObjParseResult::DEFAULT_GROUP_NAME.into())
    }

    pub fn add_to_default_group(&mut self, obj: Object<'a>) {
        self.objects
            .entry(ObjParseResult::DEFAULT_GROUP_NAME.to_string())
            .or_default()
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

    pub fn collapse_to_single_group(&self) -> Option<Box<Object<'a>>> {
        if self.objects.is_empty() {
            return Option::None;
        }

        let mut result = Group::builder();
        for obj in self.objects.values().flatten() {
            result = result.add_child(obj.clone());
        }

        let g: Object = result.build().into();
        Option::Some(g.into())
    }
}

fn vertex(input: &str) -> IResult<&str, Point> {
    // remove leading whitespace
    let (r, _) = nom::character::complete::space0(input)?;

    nom::sequence::tuple((
        nom::bytes::complete::tag("v"),
        nom::character::complete::space1,
        nom::number::complete::double,
        nom::character::complete::space1,
        nom::number::complete::double,
        nom::character::complete::space1,
        nom::number::complete::double,
    ))(r)
    .map(|(remainder, (_, _, x, _, y, _, z))| (remainder, Point::new(x, y, z)))
}

fn normal(input: &str) -> IResult<&str, Vector> {
    // remove leading whitespace
    let (r, _) = nom::character::complete::space0(input)?;

    nom::sequence::tuple((
        nom::bytes::complete::tag("vn"),
        nom::character::complete::space1,
        nom::number::complete::double,
        nom::character::complete::space1,
        nom::number::complete::double,
        nom::character::complete::space1,
        nom::number::complete::double,
    ))(r)
    .map(|(remainder, (_, _, x, _, y, _, z))| (remainder, Vector::new(x, y, z)))
}

fn face<'a, 'i>(
    input: &'i str,
    parse_result: &ObjParseResult<'a>,
) -> IResult<&'i str, (Vec<Triangle<'a>>, Vec<SmoothTriangle<'a>>)> {
    // remove leading whitespace
    let (r, _) = nom::character::complete::space0(input)?;
    let (r, _) = nom::bytes::complete::tag("f")(r)?;
    let (r, _) = nom::character::complete::space0(r)?;

    let r = nom::multi::separated_list1(
        nom::character::complete::char(' '),
        nom::sequence::tuple((
            nom::character::complete::digit1,
            nom::combinator::opt(nom::character::complete::char('/')),
            nom::combinator::opt(nom::character::complete::digit1),
            nom::combinator::opt(nom::character::complete::char('/')),
            nom::combinator::opt(nom::character::complete::digit1),
        )),
    )(r);

    let (remainder, vertex_components) = r?;

    // fan triangles
    let mut triangles = vec![];
    let mut smooth_triangles = vec![];

    for index in 1..(vertex_components.len() - 1) {
        let index_1 = 0;
        let index_2 = index;
        let index_3 = index + 1;

        let (v1_index, _, _, _, vn1_index) = vertex_components[index_1];
        let (v2_index, _, _, _, vn2_index) = vertex_components[index_2];
        let (v3_index, _, _, _, vn3_index) = vertex_components[index_3];

        let p1 = parse_result
            .vertex(v1_index.parse::<usize>().unwrap())
            .expect("Reference to non-exsistent vertix #1.");
        let p2 = parse_result
            .vertex(v2_index.parse::<usize>().unwrap())
            .expect("Reference to non-exsistent vertix #2.");
        let p3 = parse_result
            .vertex(v3_index.parse::<usize>().unwrap())
            .expect("Reference to non-exsistent vertix #3.");

        if vn1_index.is_some() && vn2_index.is_some() && vn3_index.is_some() {
            let vn1 = parse_result
                .normal(vn1_index.unwrap().parse::<usize>().unwrap())
                .expect("Reference to non-exsistent vertix normal #1.");
            let vn2 = parse_result
                .normal(vn2_index.unwrap().parse::<usize>().unwrap())
                .expect("Reference to non-exsistent vertix normal #2.");
            let vn3 = parse_result
                .normal(vn3_index.unwrap().parse::<usize>().unwrap())
                .expect("Reference to non-exsistent vertix normal #3.");

            let st = SmoothTriangle::builder(
                p1.clone(),
                p2.clone(),
                p3.clone(),
                vn1.clone(),
                vn2.clone(),
                vn3.clone(),
            )
            .build();
            smooth_triangles.push(st);
        } else {
            let t = Triangle::builder(p1.clone(), p2.clone(), p3.clone()).build();
            triangles.push(t);
        }
    }

    IResult::Ok((remainder, (triangles, smooth_triangles)))
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
        } else if let IResult::Ok((_, normal)) = normal(l.as_ref()) {
            to_return.normals.push(normal);
        } else if let IResult::Ok((_, (triangles, smooth_triangles))) = face(l.as_ref(), &to_return)
        {
            // process regular triangles
            for t in triangles {
                if let Option::Some(group_name) = &current_group {
                    to_return.add_to_named_group(group_name, t.into());
                } else {
                    to_return.add_to_default_group(t.into());
                }
            }

            // process smooth triangles
            for t in smooth_triangles {
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
