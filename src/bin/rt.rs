use indicatif::{ProgressIterator, ProgressStyle};

use raytraceweekend::{Colour, Hittable, HittableList, Point3, Ray, Sphere, Vec3, unit_vector};

fn ray_color(r: &Ray, world: &HittableList) -> Colour {
    // Find the first object that intersects the ray, and return those details
    if let Some(hit_record) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (hit_record.normal + Colour::new(1.0, 1.0, 1.0));
    }
    // No objects were found, so.. continue to the horizon.
    let unit_direction = unit_vector(&r.direction);
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
}

fn main() {
    env_logger::init();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;

    // Build the image based on the width and ratio, ensuring it is at least 1
    let image_height: i32 = match f64::from(image_width) / aspect_ratio {
        val if val > 1.0 => val as i32,
        _ => 1,
    };

    // World
    log::info!("Initialising the world");

    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    log::info!("Creating image {image_width} * {image_height}");

    // Define the Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
    let camera_centre = Point3::new(0.0, 0.0, 0.0);
    log::debug!(
        "Focal Length: {focal_length}, Viewport Height: {viewport_height}, Viewport Width: {viewport_width}, Camera Centre: {camera_centre}"
    );

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    log::debug!("viewport_u: {viewport_u:?}, viewport_v: {viewport_v:?}");

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / f64::from(image_width);
    let pixel_delta_v = viewport_v / f64::from(image_height);
    log::debug!("pixel_delta_u: {pixel_delta_u:?}, pixel_data_v: {pixel_delta_v:?}");

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    log::debug!("pixel00_loc: {pixel00_loc:?}");

    log::info!("Rendering image");
    // Define the style
    let style = ProgressStyle::with_template(
        "[{eta_precise}/{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap();
    println!("P3\n{image_width} {image_height}\n255\n");
    for j in (0..image_height).progress_with_style(style) {
        let row_offset = f64::from(j) * pixel_delta_v;
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (f64::from(i) * pixel_delta_u) + row_offset;
            let ray_direction = pixel_center - camera_centre;
            let r = Ray::new(camera_centre, ray_direction);

            let pixel_colour = ray_color(&r, &world);
            println!("{}", pixel_colour.write_colour());
        }
    }
    log::info!("Done");
}
