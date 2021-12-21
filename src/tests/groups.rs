use crate::domain::matrix::Matrix;
use crate::domain::object::Group;

#[test]
fn ch14_test1_creating_new_group() {
    let g = Group::new().build();
    assert_eq!(
        g.shape.transformation,
        crate::domain::matrix::IDENTITY.clone()
    );
    assert!(g.children.is_empty());
}
