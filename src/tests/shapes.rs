use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::Object;

#[test]
fn ch9_test1_default_transformation_and_assigning_on_shapes() {
    let s = Object::new_null();
    assert_eq!(
        s.shape().transformation,
        crate::domain::matrix::IDENTITY.clone()
    );

    let mut s = Object::new_null();
    let t = Matrix::new_translation(2.0, 3.0, 4.0);
    s.shape_mut().transformation = t.clone();
    assert_eq!(s.shape().transformation, t);
}

#[test]
fn ch9_test2_default_material_and_assign_on_shapes() {
    let s = Object::new_null();
    assert_eq!(s.shape().material, Material::new());

    let mut s = Object::new_null();
    let mut m = Material::new();
    m.ambient = 1.0;
    s.shape_mut().material = m.clone();
    assert_eq!(s.shape().material, m);
}
