mod colour;
mod hittable;
mod ray;
mod vec3;
pub use colour::Colour;
pub use hittable::{Hittable, Sphere};
use ray::Ray;
use std::{error::Error, io::prelude::*};
pub use vec3::Vec3;

fn hit_sphere(centre: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - centre;
    let a = r.direction().dot(r.direction());
    let half_b = oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    #[allow(clippy::suspicious_operation_groupings)]
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
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
 * Renders a scene.
 *
 * # Parameters
 *
 * * * `image_width` and `image_height` are the image dimesions, in pixels.
 * * `viewport_width` and `viewport_height` are the viewport dimensions, in
 *    virtual co-ordinates.
 * * `focal_length` is the camera focal length.
 * * `output` is the stream to write the generated image to.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
pub fn render(
    image_width: u32,
    image_height: u32,
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
    output: &mut dyn Write,
    log: bool,
) -> Result<(), Box<dyn Error>> {
    // Geometry.

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
