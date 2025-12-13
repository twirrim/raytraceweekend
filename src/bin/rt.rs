use raytraceweekend::Vec3;
use raytraceweekend::camera::Camera;
use raytraceweekend::hit::HittableList;
use raytraceweekend::sphere::Sphere;

fn main() {
    env_logger::init();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 3840;
    let samples_per_pixel: usize = 100;

    // World
    log::info!("Initialising the world");

    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera: Camera = Camera::new(aspect_ratio, image_width, samples_per_pixel);
    camera.render(&world);

    log::info!("Done");
}
