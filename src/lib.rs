use std::{error::Error, io::prelude::*};

/**
 * Runs the program.
 *
 * # Parameters
 *
 * * `output` is the stream to write the generated image to.
 */
pub fn run(output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    // Image

    let image_width: u32 = 256;
    let image_height: u32 = 256;

    // Render

    write!(output, "P3\n{} {}\n255\n", image_width, image_height)?;

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = f64::from(i) / f64::from(image_width - 1);
            let g = f64::from(j) / f64::from(image_height - 1);
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            writeln!(output, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
