use raytraceweekend::hit::HittableList;
use raytraceweekend::sphere::Sphere;
use raytraceweekend::Vec3;
use raytraceweekend::camera::Camera;

fn main() {
    env_logger::init();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;

    // World
    log::info!("Initialising the world");

    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera: Camera = Camera::new(aspect_ratio, image_width);
    camera.render(&world);

    log::info!("Done");
}
