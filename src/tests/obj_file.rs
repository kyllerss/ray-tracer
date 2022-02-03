use crate::domain::Point;

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
