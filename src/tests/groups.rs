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
    let s1: Object = Sphere::new().build().into();
    let s2: Object = Sphere::new()
        .transformation(Matrix::new_translation(0.0, 0.0, -3.0))
        .build()
        .into();
    let s3: Object = Sphere::new()
        .transformation(Matrix::new_translation(5.0, 0.0, 0.0))
        .build()
        .into();
    let g: Object = Group::new()
        .add_child(s1.clone())
        .add_child(s2.clone())
        .add_child(s3.clone())
        .build()
        .into();

    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let mut xs = g.local_intersect(&r);

    assert_eq!(xs.len(), 4);
    let r1 = xs.hit_unchecked().unwrap();
    let r2 = xs.hit_unchecked().unwrap();
    let r3 = xs.hit_unchecked().unwrap();
    let r4 = xs.hit_unchecked().unwrap();
    assert_eq!(r1.object.shape().id, s2.shape().id);
    assert_eq!(r2.object.shape().id, s2.shape().id);
    assert_eq!(r3.object.shape().id, s1.shape().id);
    assert_eq!(r4.object.shape().id, s1.shape().id);
}
