use crate::domain::matrix::Matrix;
use crate::domain::object::{Group, Null, Object};

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

#[test]
fn ch14_test3_adding_child_to_group() {
    let s = Null::new().build();
    let g = Group::new().add_child(s.clone().into()).build();

    assert_eq!(g.children.len(), 1);
    assert_eq!(g.children[0], Object::Null(s.clone()));
    assert_eq!(
        g.children[0].shape().parent,
        Option::Some(&g.clone().into())
    );
}
