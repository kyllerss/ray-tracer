use crate::domain::color::Color;
use crate::domain::light::Light;
use crate::domain::Point;

#[test]
fn ch6_test6_point_light_has_position_and_intensity() {
    let i_exp = Color::new(1.0, 1.0, 1.0);
    let p_exp = Point::new(0.0, 0.0, 0.0);
    let light = Light::new(p_exp.clone(), i_exp.clone());
    assert_eq!(light.position, p_exp);
    assert_eq!(light.intensity, i_exp);
}
