use crate::domain::object::{Group, Object, Triangle};
use crate::domain::Point;

fn extract_triangle<'r, 's: 'r>(obj: &'r Object<'s>) -> &'r Triangle<'s> {
    match obj {
        Object::Triangle(triangle) => &triangle,
        _ => panic!("Expected triangle!"),
    }
}

#[test]
fn ch15_test7_ignore_unrecognized_lines() {
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
fn ch15_test8_vertex_records() {
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
fn ch15_test9_parsing_triangle_faces() {
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

#[test]
fn ch15_test10_triangulating_polygons() {
    let contents = r#"
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0
        v 0 2 0
        
        f 1 2 3 4 5
    "#;

    let r = crate::utils::obj_parser::parse_obj_file(contents);
    assert!(r.is_some());

    let p = r.unwrap();
    let g = p.default_group();

    assert!(g.get(0).is_some());
    let t1 = extract_triangle(g.get(0).unwrap());

    assert!(g.get(1).is_some());
    let t2 = extract_triangle(g.get(1).unwrap());

    assert!(g.get(2).is_some());
    let t3 = extract_triangle(g.get(2).unwrap());

    assert_eq!(t1.p1, *p.vertex(1).unwrap());
    assert_eq!(t1.p2, *p.vertex(2).unwrap());
    assert_eq!(t1.p3, *p.vertex(3).unwrap());
    assert_eq!(t2.p1, *p.vertex(1).unwrap());
    assert_eq!(t2.p2, *p.vertex(3).unwrap());
    assert_eq!(t2.p3, *p.vertex(4).unwrap());
    assert_eq!(t3.p1, *p.vertex(1).unwrap());
    assert_eq!(t3.p2, *p.vertex(4).unwrap());
    assert_eq!(t3.p3, *p.vertex(5).unwrap());
}

#[test]
fn ch15_test11_triangles_in_groups() {
    let contents = r#"
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0
        
        g FirstGroup
        f 1 2 3
        g SecondGroup       
        f 1 3 4  
    "#;

    let r = crate::utils::obj_parser::parse_obj_file(contents);
    assert!(r.is_some());

    let p = r.unwrap();
    let g1 = p.named_group(&"FirstGroup".to_string());
    let g2 = p.named_group(&"SecondGroup".to_string());

    assert!(g1.is_some());
    assert!(g1.unwrap().get(0).is_some());
    let t1 = extract_triangle(g1.unwrap().get(0).unwrap());

    assert!(g2.is_some());
    assert!(g2.unwrap().get(0).is_some());
    let t2 = extract_triangle(g2.unwrap().get(0).unwrap());

    assert_eq!(t1.p1, *p.vertex(1).unwrap());
    assert_eq!(t1.p2, *p.vertex(2).unwrap());
    assert_eq!(t1.p3, *p.vertex(3).unwrap());
    assert_eq!(t2.p1, *p.vertex(1).unwrap());
    assert_eq!(t2.p2, *p.vertex(3).unwrap());
    assert_eq!(t2.p3, *p.vertex(4).unwrap());
}

#[test]
fn ch15_test12_convert_obj_file_to_group() {
    let contents = r#"
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0
        
        g FirstGroup
        f 1 2 3
        g SecondGroup       
        f 1 3 4  
    "#;

    let r = crate::utils::obj_parser::parse_obj_file(contents);
    assert!(r.is_some());

    let g: Box<Group> = r.unwrap().into();
    assert_eq!(g.children.len(), 2);
}
