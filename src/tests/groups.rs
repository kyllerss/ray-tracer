use crate::domain::matrix::Matrix;
use crate::domain::object::{Group, Null, Object, Sphere};
use crate::domain::ray::Ray;
use crate::domain::{Point, Vector};
use std::f64::consts::PI;

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
    assert_eq!(s.shape.parent(), Option::None);
}

#[test]
fn ch14_test3_adding_child_to_group() {
    let s = Null::new().build();
    let g = Group::new().add_child(s.clone().into()).build();

    assert_eq!(g.children.len(), 1);
    assert_eq!(g.children[0].shape().id, s.shape.id);
    assert_eq!(g.children[0].shape().parent().unwrap(), g.into());
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

#[test]
fn ch14_test6_intersecting_transformed_group() {
    let s = Sphere::new()
        .transformation(Matrix::new_translation(5.0, 0.0, 0.0))
        .build()
        .into();
    let g: Object = Group::new()
        .transformation(Matrix::new_scaling(2.0, 2.0, 2.0))
        .add_child(s)
        .build()
        .into();

    let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::new(0.0, 0.0, 1.0));
    let xs = g.intersect(&r);
    assert_eq!(xs.len(), 2);
}

#[test]
fn ch14_test7_convert_point_from_world_to_object_space() {
    let s: Object = Sphere::new()
        .transformation(Matrix::new_translation(5.0, 0.0, 0.0))
        .build()
        .into();
    let g2: Object = Group::new()
        .transformation(Matrix::new_scaling(2.0, 2.0, 2.0))
        .add_child(s)
        .build()
        .into();
    let g1 = Group::new()
        .transformation(Matrix::new_rotation_y(PI / 2.0))
        .add_child(g2)
        .build();

    let s_inner: Object = match &g1.children[0] {
        Object::Group(group) => match &group.children[0] {
            Object::Sphere(sphere) => sphere.clone(),
            _ => panic!(),
        },
        _ => panic!(),
    }
    .into();

    let p = s_inner.world_to_object(&Point::new(-2.0, 0.0, -10.0));
    let p_exp = Point::new(0.0, 0.0, -1.0);
    assert_eq!(p, p_exp);
}

#[test]
fn ch14_test8_convert_normal_from_object_to_world_space() {
    let sphere = Sphere::new()
        .transformation(Matrix::new_translation(5.0, 0.0, 0.0))
        .build();
    let g2 = Group::new()
        .transformation(Matrix::new_scaling(1.0, 2.0, 3.0))
        .add_child(sphere.into())
        .build();
    let g1 = Group::new()
        .transformation(Matrix::new_rotation_y(PI / 2.0))
        .add_child(g2.into())
        .build();

    let inner_sphere = match &g1.children[0] {
        Object::Group(inner_g2) => &inner_g2.children[0],
        _ => panic!("Unexpected structure!"),
    };

    let n = inner_sphere.normal_to_world(&Vector::new(
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
    ));
    let n_exp = Vector::new(0.2857, 0.4286, -0.8571);

    assert_eq!(n, n_exp);
}

#[test]
fn ch14_test9_finding_normal_of_child_object() {
    let s = Sphere::new()
        .transformation(Matrix::new_translation(5.0, 0.0, 0.0))
        .build();
    let g2 = Group::new()
        .transformation(Matrix::new_scaling(1.0, 2.0, 3.0))
        .add_child(s.into())
        .build();
    let g1 = Group::new()
        .transformation(Matrix::new_rotation_y(PI / 2.0))
        .add_child(g2.into())
        .build();

    let inner_sphere = match &g1.children[0] {
        Object::Group(inner_g2) => &inner_g2.children[0],
        _ => panic!("Unexpected structure!"),
    };

    let n = inner_sphere.normal_at(&Point::new(1.7321, 1.1547, -5.5774));
    let n_exp = Vector::new(0.2857, 0.4286, -0.8571);

    assert_eq!(n, n_exp);
}
