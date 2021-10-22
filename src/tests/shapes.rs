use crate::domain::material::Material;
use crate::domain::matrix::Matrix;
use crate::domain::object::Object;
use crate::domain::{Point, Vector};

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

// #[test]
// NOTE: Commenting out as it is superfluous and causes mutability in a non-mutable operation.
// fn ch9_test3_intersecting_scaled_and_translated_shape_with_ray() {
//     // scaled shape
//     let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
//     let mut s = Object::new_null();
//     s.shape_mut().transformation = Matrix::new_scaling(2.0, 2.0, 2.0);
//     let xs = s.intersect(&r);
//     let saved_ray_exp = Ray::new(Point::new(0.0, 0.0, -2.5), Vector::new(0.0, 0.0, 0.5));
//
//     // translated shape
// }

#[test]
fn ch9_test4_computing_normal_on_translated_and_transformed_test_shape() {
    let mut s = Object::new_null();
    s.shape_mut().transformation = Matrix::new_translation(0.0, 1.0, 0.0);
    let n = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));
    let n_exp = Vector::new(0.0, 0.70711, -0.70711);
    assert_eq!(n, n_exp);
}
