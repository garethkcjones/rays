mod camera;
mod colour;
mod hittable;
mod ray;
mod vec3;
pub use camera::Camera;
pub use colour::Colour;
pub use hittable::{Hittable, Sphere};
use rand::prelude::*;
use ray::Ray;
use std::{error::Error, io::prelude::*};
pub use vec3::Vec3;

/**
 * Calculates the colour of a ray of light.
 */
#[must_use]
fn ray_colour(r: &Ray, world: &dyn Hittable) -> Colour {
    if let Some(rec) = world.hit(r, 0.0..f64::INFINITY) {
        let Vec3(x, y, z) = rec.normal();
        return 0.5 * Colour(x + 1.0, y + 1.0, z + 1.0);
    }

    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour(1.0, 1.0, 1.0) + t * Colour(0.5, 0.7, 1.0)
}

/**
 * Renders a scene.
 *
 * # Parameters
 *
 * * `world` contains the hittable objects in the scene.
 * * `image_width` and `image_height` are the image dimesions, in pixels.
 * * `samples_per_pixel` is the number of samples per pixel.
 * * `cam` is the camera.
 * * `output` is the stream to write the generated image to.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
pub fn render(
    world: &dyn Hittable,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    cam: &Camera,
    output: &mut dyn Write,
    log: bool,
) -> Result<(), Box<dyn Error>> {
    assert!(image_width > 1);
    assert!(image_height > 1);
    assert!(samples_per_pixel > 0);

    // Initialize random number generator.
    let mut rand_eng = thread_rng();

    // Render.

    let width_scale = f64::from(image_width - 1);
    let height_scale = f64::from(image_height - 1);

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
            let mut pixel_colour = Colour(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + rand_eng.gen::<f64>()) / width_scale;
                let v = (f64::from(j) + rand_eng.gen::<f64>()) / height_scale;

                let r = cam.get_ray(u, v);

                pixel_colour += ray_colour(&r, world);
            }

            let (ir, ig, ib) = pixel_colour.to_rgb8(samples_per_pixel);

            writeln!(output, "{} {} {}", ir, ig, ib)?;
        }
    }

    if log {
        eprint!("\nDone.\n");
    }

    Ok(())
}
