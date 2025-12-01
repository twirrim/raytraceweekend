use raytraceweekend::Colour;

fn main() {
    env_logger::init();
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255\n");
    for j in 0..image_height {
        log::info!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_colour = Colour::new(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.0,
            );
            pixel_colour.write_colour();
        }
    }
    log::info!("Done");
}
