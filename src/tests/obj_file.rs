#[test]
fn ch15_test1_ignore_unrecognized_lines() {
    let contents = r#"There was a young lady named Bright
        who traveled much faster than light.
        She set out one day
        in a relative way,
        and came back the previous night."#;

    let objs = crate::utils::obj_parser::parse_obj(contents);
    assert!(objs.is_ok());
    assert_eq!(objs.unwrap().skipped, 5);
}
