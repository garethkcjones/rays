mod camera;
mod colour;
mod hittable;
mod material;
mod ray;
mod vec3;
pub use camera::Camera;
pub use colour::Colour;
use hittable::HitRecord;
pub use hittable::{Hittable, Sphere};
pub use material::{Dielectric, Lambertian0, Lambertian1, Lambertian2, Material, Metal};
use rand::prelude::*;
use ray::Ray;
use std::{convert::TryFrom, error::Error, io::prelude::*, panic, sync::Arc, thread};
pub use vec3::Vec3;

/**
 * Calculates the colour of a ray of light.
 */
#[must_use]
fn ray_colour(r: &Ray, world: &dyn Hittable, depth: u32) -> Colour {
    // If we’ve exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Colour(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material_ref().scatter(r, &rec) {
            return attenuation * ray_colour(&scattered, world, depth - 1);
        }
        return Colour(0.0, 0.0, 0.0);
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
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
fn render(
    world: Arc<dyn Hittable>,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    cam: Arc<Camera>,
    log: bool,
) -> Result<Box<[Colour]>, Box<dyn Error + Send + Sync>> {
    #![allow(clippy::many_single_char_names)]

    assert!(image_width > 1);
    assert!(image_height > 1);
    assert!(samples_per_pixel > 0);
    assert!(max_depth > 0);

    // Initialize random number generator.
    let mut rand_eng = thread_rng();
    let rand_dst = rand::distributions::Uniform::new(0.0, 1.0);

    // Render.

    let mut pixels =
        Vec::with_capacity(usize::try_from(image_width)? * usize::try_from(image_height)?);

    let width_scale = f64::from(image_width - 1);
    let height_scale = f64::from(image_height - 1);

    for j in (0..image_height).rev() {
        if log {
            let percent = (100.0 * f64::from(image_height - j) / f64::from(image_height)).round();
            eprint!(
                "\rScanlines remaining: {:5}   ({:3} % complete)",
                j, percent
            );
        }

        let j = f64::from(j);

        for i in 0..image_width {
            let i = f64::from(i);

            let mut pixel_colour = Colour(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let ur = rand_eng.sample(rand_dst);
                let vr = rand_eng.sample(rand_dst);

                let u = (i + ur) / width_scale;
                let v = (j + vr) / height_scale;

                let r = cam.get_ray(u, v);

                pixel_colour += ray_colour(&r, world.as_ref(), max_depth);
            }

            pixels.push(pixel_colour);
        }
    }

    if log {
        eprintln!();
    }

    Ok(pixels.into_boxed_slice())
}

/**
 * Writes an image file.
 *
 * # Parameters
 *
 * * `output` is the stream to write the generated image to.
 * * `pixels` is the image data.
 * * `image_width` and `image_height` are the image dimesions, in pixels.
 * * `samples_per_pixel` is the number of samples per pixel.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
fn write_file(
    output: &mut dyn Write,
    pixels: &[Colour],
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    log: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    assert!(image_width > 1);
    assert!(image_height > 1);
    assert!(samples_per_pixel > 0);

    if log {
        eprintln!("Writing output...");
    }

    write!(output, "P3\n{} {}\n255\n", image_width, image_height)?;
    for pixel_colour in pixels {
        let (ir, ig, ib) = pixel_colour.to_rgb8(samples_per_pixel);
        writeln!(output, "{} {} {}", ir, ig, ib)?;
    }

    if log {
        eprintln!("Done.");
    }

    Ok(())
}

/**
 * Runs the program.
 *
 * # Parameters
 *
 * * `num_threads` is the number of threads to distribute rendering over.
 * * `world` contains the hittable objects in the scene.
 * * `image_width` and `image_height` are the image dimesions, in pixels.
 * * `samples_per_pixel` is the number of samples per pixel.
 * * `max_depth` is the recursion limit for ray reflections.
 * * `cam` is the camera.
 * * `output` is the stream to write the generated image to.
 * * If `log` is `true`, progress is reported to the standard error stream.
 */
#[allow(clippy::too_many_arguments)]
pub fn run(
    num_threads: u32,
    world: Arc<dyn Hittable>,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    cam: Arc<Camera>,
    output: &mut dyn Write,
    log: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let samples_per_thread = samples_per_pixel / num_threads;
    let remaining_samples = samples_per_pixel % num_threads;

    // Spawn threads.
    let mut threads = Vec::with_capacity(num_threads as usize - 1);
    for thread_num in 1..num_threads {
        let samples_per_pixel = if thread_num <= remaining_samples {
            samples_per_thread + 1
        } else {
            samples_per_thread
        };
        let world = Arc::clone(&world);
        let cam = Arc::clone(&cam);
        threads.push(thread::spawn(move || {
            render(
                world,
                image_width,
                image_height,
                samples_per_pixel,
                max_depth,
                cam,
                false,
            )
        }));
    }

    // This thread.
    let mut pixels = render(
        world,
        image_width,
        image_height,
        samples_per_thread,
        max_depth,
        cam,
        log,
    )?;

    // Join threads.
    for (i, thread) in threads.into_iter().enumerate() {
        if log {
            eprint!("\rWaiting for thread {:2} of {}...", i + 2, num_threads);
        }
        let thread_pixels = match thread.join() {
            Ok(pixels) => pixels?,
            Err(x) => panic::resume_unwind(x),
        };
        assert_eq!(pixels.len(), thread_pixels.len());
        for (pixel, thread_pixel) in pixels.iter_mut().zip(thread_pixels.iter()) {
            *pixel += *thread_pixel;
        }
    }
    if log {
        eprintln!();
    }

    write_file(
        output,
        &pixels,
        image_width,
        image_height,
        samples_per_pixel,
        log,
    )
}
