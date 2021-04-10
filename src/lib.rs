mod colour;
mod vec3;
pub use colour::Colour;
use std::{error::Error, io::prelude::*};
pub use vec3::Vec3;

/**
 * Runs the program.
 *
 * # Parameters
 *
 * * `output` is the stream to write the generated image to.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
pub fn run(output: &mut dyn Write, log: bool) -> Result<(), Box<dyn Error>> {
    // Image

    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render

    write!(output, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in (0..image_height).rev() {
        if log {
            let percent = (100.0 * f64::from(image_height - j) / f64::from(image_height)).round();
            eprint!(
                "\rScanlines remaining: {:5}   ({:3} % complete)",
                j, percent
            );
        }

        for i in 0..image_width {
            let r = f64::from(i) / f64::from(image_width - 1);
            let g = f64::from(j) / f64::from(image_height - 1);
            let b = 0.25;

            let pixel_colour = Colour(r, g, b);
            let (ir, ig, ib) = pixel_colour.to_rgb8();

            writeln!(output, "{} {} {}", ir, ig, ib)?;
        }
    }

    if log {
        eprint!("\nDone.\n");
    }

    Ok(())
}
