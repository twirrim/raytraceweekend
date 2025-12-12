use indicatif::{ProgressIterator, ProgressStyle};

use raytraceweekend::{Colour, Point3, Ray, Vec3, hit_sphere, unit_vector};

fn ray_color(r: &Ray) -> Colour {
    // Add a sphere to the image
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let normal = unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * Colour::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0);
    }
    let unit_direction = unit_vector(&r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0);
}

fn main() {
    env_logger::init();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;

    // Build the image based on the width and ratio, ensuring it is at least 1
    let image_height: i32 = match image_width as f64 / aspect_ratio {
        val if val > 1.0 => val as i32,
        _ => 1,
    };
    log::info!("Creating image {image_width} * {image_height}");

    // Define the Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_centre = Point3::new(0.0, 0.0, 0.0);
    log::debug!(
        "Focal Length: {focal_length}, Viewport Height: {viewport_height}, Viewport Width: {viewport_width}, Camera Centre: {camera_centre}"
    );

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    log::debug!("viewport_u: {:?}, viewport_v: {:?}", viewport_u, viewport_v);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;
    log::debug!(
        "pixel_delta_u: {:?}, pixel_data_v: {:?}",
        pixel_delta_u,
        pixel_delta_v
    );

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    log::debug!("pixel00_loc: {:?}", pixel00_loc);

    log::info!("Rendering image");
    // Define the style
    let style = ProgressStyle::with_template(
        "[{eta_precise}/{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap();
    println!("P3\n{image_width} {image_height}\n255\n");
    for j in (0..image_height).progress_with_style(style) {
        let row_offset = j as f64 * pixel_delta_v;
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + row_offset;
            let ray_direction = pixel_center - camera_centre;
            let r = Ray::new(camera_centre, ray_direction);

            let pixel_colour = ray_color(&r);
            println!("{}", pixel_colour.write_colour());
        }
    }
    log::info!("Done");
}
