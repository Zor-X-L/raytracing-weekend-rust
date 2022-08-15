use crate::color::{Color, write_color};
use crate::vec3::Float;

mod vec3;
mod color;

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
                i as Float / (image_width - 1) as Float,
                j as Float / (image_height - 1) as Float,
                0.25,
            );
            write_color(&mut std::io::stdout(), &pixel_color).expect("Error");
        }
    }

    eprint!("\nDone.\n");
}
