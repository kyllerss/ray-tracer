use crate::domain::object::{Object, Triangle};
use crate::domain::Point;

fn extract_triangle<'r, 's: 'r>(obj: &'r Object<'s>) -> &'r Triangle {
    match obj {
        Object::Triangle(triangle) => &triangle,
        _ => panic!("Expected triangle!"),
    }
}

#[test]
fn ch15_test1_ignore_unrecognized_lines() {
    let contents = r#"There was a young lady named Bright
        who traveled much faster than light.
        She set out one day
        in a relative way,
        and came back the previous night."#;

    let r = crate::utils::obj_parser::parse_obj_file(contents);
    assert!(r.is_some());
    assert_eq!(r.unwrap().skipped, 5);
}

#[test]
fn ch15_test2_vertex_records() {
    let contents = r#"
        v -1 1 0
        v -1.0000 0.5000 0.0000
        v 1 0 0
        v 1 1 0
    "#;

    let r = crate::utils::obj_parser::parse_obj_file(contents);
    assert!(r.is_some());

    let p = r.unwrap();
    assert_eq!(p.vertex_count(), 4);
    assert_eq!(p.vertex(1).unwrap(), &Point::new(-1.0, 1.0, 0.0));
    assert_eq!(p.vertex(2).unwrap(), &Point::new(-1.0, 0.5, 0.0));
    assert_eq!(p.vertex(3).unwrap(), &Point::new(1.0, 0.0, 0.0));
    assert_eq!(p.vertex(4).unwrap(), &Point::new(1.0, 1.0, 0.0));
}

#[test]
fn ch15_test3_parsing_triangle_faces() {
    let contents = r#"
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0
        
        f 1 2 3 
        f 1 3 4
    "#;

    let r = crate::utils::obj_parser::parse_obj_file(contents);
    assert!(r.is_some());

    let p = r.unwrap();
    let g = p.default_group();

    assert!(g.get(0).is_some());
    let t1 = extract_triangle(g.get(0).unwrap());

    assert!(g.get(1).is_some());
    let t2 = extract_triangle(g.get(1).unwrap());

    assert_eq!(t1.p1, *p.vertex(1).unwrap());
    assert_eq!(t1.p2, *p.vertex(2).unwrap());
    assert_eq!(t1.p3, *p.vertex(3).unwrap());
    assert_eq!(t2.p1, *p.vertex(1).unwrap());
    assert_eq!(t2.p2, *p.vertex(3).unwrap());
    assert_eq!(t2.p3, *p.vertex(4).unwrap());
}
