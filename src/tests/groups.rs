use crate::domain::matrix::Matrix;
use crate::domain::object::{Group, Null};

#[test]
fn ch14_test1_creating_new_group() {
    let g = Group::new().build();
    assert_eq!(
        g.shape.transformation,
        crate::domain::matrix::IDENTITY.clone()
    );
    assert!(g.children.is_empty());
}

#[test]
fn ch14_test2_shape_has_parent_attribute() {
    let s = Null::new().build();
    assert_eq!(s.shape.parent, Option::None);
}
