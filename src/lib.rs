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
fn ray_colour(r: &Ray, world: &dyn Hittable, depth: u32) -> Colour {
    // If we’ve exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Colour(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let target = rec.p() + Vec3::new_random_in_hemisphere(rec.normal());
        // let target = rec.p() + rec.normal() + Vec3::new_random_in_unit_sphere();
        // let target = rec.p() + rec.normal() + Vec3::new_random_unit();
        return 0.5 * ray_colour(&Ray::new(rec.p(), target - rec.p()), world, depth - 1);
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
 * * `max_depth` is the recursion limit for ray reflections.
 * * `cam` is the camera.
 * * `output` is the stream to write the generated image to.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
#[allow(clippy::too_many_arguments)]
pub fn render(
    world: &dyn Hittable,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    cam: &Camera,
    output: &mut dyn Write,
    log: bool,
) -> Result<(), Box<dyn Error>> {
    assert!(image_width > 1);
    assert!(image_height > 1);
    assert!(samples_per_pixel > 0);
    assert!(max_depth > 0);

    // Initialize random number generator.
    let mut rand_eng = thread_rng();
    let rand_dst = rand::distributions::Uniform::new(0.0, 1.0);

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
                let ur = rand_eng.sample(rand_dst);
                let vr = rand_eng.sample(rand_dst);

                let u = (f64::from(i) + ur) / width_scale;
                let v = (f64::from(j) + vr) / height_scale;

                let r = cam.get_ray(u, v);

                pixel_colour += ray_colour(&r, world, max_depth);
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
