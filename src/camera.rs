use indicatif::{ProgressIterator, ProgressStyle};

use crate::hit::{Hittable, HittableList};
use crate::ray::Ray;
use crate::{Colour, Point3, Vec3, random_f64, random_on_hemisphere, unit_vector};

#[derive(Debug)]
pub struct Camera {
    image_width: usize, // rendered image width in pixel count
    image_height: usize,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: usize,
    pixel_samples_scale: f64,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 1.0;
        let image_width = 400;
        let samples_per_pixel = 10;
        Camera::new(aspect_ratio, image_width, samples_per_pixel)
    }
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, samples_per_pixel: usize) -> Self {
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
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        Self {
            image_width,
            image_height,
            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
        }
    }

    fn ray_colour(r: &Ray, world: &HittableList) -> Colour {
        // Find the first object that intersects the ray, and return those details
        if let Some(hit_record) = world.hit(r, &(0.0..f64::INFINITY)) {
            let direction = random_on_hemisphere(&hit_record.normal);
            return 0.5 * Camera::ray_colour(&Ray::new(hit_record.p, direction), world);
            // return 0.5 * (hit_record.normal + Colour::new(1.0, 1.0, 1.0));
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
            for i in 0..self.image_width {
                let mut pixel_colour: Colour = Colour::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_colour += Camera::ray_colour(&r, world);
                }
                println!(
                    "{}",
                    (self.pixel_samples_scale * pixel_colour).write_colour()
                );
            }
        }
        log::info!("Done");
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.centre;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        return Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0);
    }
}
