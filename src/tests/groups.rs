use crate::domain::matrix::Matrix;
use crate::domain::object::{Group, Null, Object, Sphere};
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};

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
    assert_eq!(g.children[0].shape().id, s.shape.id);
    unsafe {
        assert_eq!(
            *(g.children[0].shape().parent.unwrap()),
            *(Box::into_raw(g.clone()))
        );
    }
}

#[test]
fn ch14_test4_intersecting_ray_with_empty_group() {
    let g = Group::new().build();
    let g_obj = g.clone().into();
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let xs = g.local_intersect(&r, &g_obj);
    assert!(xs.is_empty());
}

#[test]
fn ch14_test5_intersecting_ray_with_nonempty_group() {
    let s1 = Sphere::new().build();
    let s2 = Sphere::new()
        .transformation(Matrix::new_translation(0.0, 0.0, -3.0))
        .build();
    let s3 = Sphere::new()
        .transformation(Matrix::new_translation(5.0, 0.0, 0.0))
        .build();
    let g: Object = Group::new()
        .add_child(s1.clone().into())
        .add_child(s2.clone().into())
        .add_child(s3.clone().into())
        .build()
        .into();

    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let mut xs = g.local_intersect(&r);

    assert_eq!(xs.len(), 4);
    assert_eq!(xs.hit_unchecked().unwrap().object.shape().id, s2.shape.id);
    assert_eq!(xs.hit_unchecked().unwrap().object.shape().id, s2.shape.id);
    assert_eq!(xs.hit_unchecked().unwrap().object.shape().id, s1.shape.id);
    assert_eq!(xs.hit_unchecked().unwrap().object.shape().id, s1.shape.id);
    // xs[0] = s2
    // xs[1] = s2
    // xs[2] = s1
    // xs[3] = s1
}
