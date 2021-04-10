mod colour;
mod ray;
mod vec3;
pub use colour::Colour;
use ray::Ray;
use std::{error::Error, io::prelude::*};
pub use vec3::Vec3;

fn hit_sphere(centre: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - centre;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

/**
 * Calculates the colour of a ray of light.
 */
fn ray_colour(r: &Ray) -> Colour {
    let t = hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3(0.0, 0.0, -1.0)).unit();
        return 0.5 * Colour(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour(1.0, 1.0, 1.0) + t * Colour(0.5, 0.7, 1.0)
}

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

    let image_aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (f64::from(image_width) / image_aspect_ratio) as u32;

    // Camera

    let viewport_aspect_ratio = f64::from(image_width) / f64::from(image_height);
    let viewport_height = 2.0;
    let viewport_width = viewport_aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - 0.5 * (horizontal + vertical) - Vec3(0.0, 0.0, focal_length);

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
            let u = f64::from(i) / f64::from(image_width - 1);
            let v = f64::from(j) / f64::from(image_height - 1);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_colour = ray_colour(&r);
            let (ir, ig, ib) = pixel_colour.to_rgb8();

            writeln!(output, "{} {} {}", ir, ig, ib)?;
        }
    }

    if log {
        eprint!("\nDone.\n");
    }

    Ok(())
}
