use crate::domain::matrix::Matrix;
use crate::domain::ray::Ray;
use crate::domain::Point;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    // constructor
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = f64::tan(field_of_view / 2.0);
        let aspect = hsize as f64 / vsize as f64;

        let half_width;
        let half_height;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: crate::domain::matrix::IDENTITY.clone(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    // Constructs a new ray that starts at camera and passes through indicated x,y pixel on the canvas
    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // offset from edge of canvas to pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        // untransformed coordinates of pixel in world space
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // using camera matrix, transform canvas point and origin, then compute ray direction vector
        // (canvas is at z = -1)
        let transf_inv = self.transform.inverse().unwrap();
        let pixel = &transf_inv * &Point::new(world_x, world_y, -1.0);
        let origin = &transf_inv * &Point::ORIGIN;
        let direction = (&pixel - &origin).normalize();

        Ray::new(origin, direction)
    }
}
