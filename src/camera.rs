use indicatif::{ProgressIterator, ProgressStyle};

use crate::hit::{Hittable, HittableList};
use crate::ray::Ray;
use crate::{Colour, Point3, Vec3, unit_vector};

#[derive(Debug)]
pub struct Camera {
    image_width: usize, // rendered image width in pixel count
    image_height: usize,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 1.0;
        let image_width = 400;
        Camera::new(aspect_ratio, image_width)        
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
        // Build the image based on the width and ratio, ensuring it is at least 1
        let image_height: usize = match image_width as f64 / aspect_ratio {
            val if val > 1.0 => val as usize,
            _ => 1,
        };

        // Define the Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let centre: Point3 = Point3::new(0.0, 0.0, 0.0);
        log::debug!(
            "Focal Length: {focal_length}, Viewport Height: {viewport_height}, Viewport Width: {viewport_width}, Centre: {centre}"
        );

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        log::debug!("viewport_u: {viewport_u:?}, viewport_v: {viewport_v:?}");

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        log::debug!("pixel_delta_u: {pixel_delta_u:?}, pixel_data_v: {pixel_delta_v:?}");

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        log::debug!("pixel00_loc: {pixel00_loc:?}");

        Self {
            image_width,
            image_height,
            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_colour(r: &Ray, world: &HittableList) -> Colour {
        // Find the first object that intersects the ray, and return those details
        if let Some(hit_record) = world.hit(r, &(0.0..f64::INFINITY)) {
            return 0.5 * (hit_record.normal + Colour::new(1.0, 1.0, 1.0));
        }
        // No objects were found, so.. continue to the horizon.
        let unit_direction = unit_vector(&r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }

    pub fn render(&self, world: &HittableList) {
        log::info!("Rendering image");
        // Define the style
        let style = ProgressStyle::with_template(
            "[{eta_precise}/{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap();
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in (0..self.image_height).progress_with_style(style) {
            let row_offset = j as f64 * self.pixel_delta_v;
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + row_offset;
                let ray_direction = pixel_center - self.centre;
                let r = Ray::new(self.centre, ray_direction);

                let pixel_colour = Camera::ray_colour(&r, world);
                println!("{}", pixel_colour.write_colour());
            }
        }
        log::info!("Done");
    }
}
