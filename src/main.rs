use crate::vec3::{Color, write_color};

mod vec3;

fn main() {

    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            write_color(&mut std::io::stdout(), &pixel_color).expect("Error");
        }
    }

    eprint!("\nDone.\n");
}
