#![allow(clippy::many_single_char_names)]

mod bounding;
mod camera;
mod hit;
mod material;
pub mod output;
mod perlin;
pub mod random;
mod ray;
mod texture;
mod triplet;
pub use bounding::{Aabb, BvhNode};
pub use camera::Camera;
pub use hit::{HitRecord, Hittable, MovingSphere, Sphere, XyRect, XzRect, YzRect};
pub use material::{
    Dielectric, DiffuseLight, Lambertian0, Lambertian1, Lambertian2, Material, Metal,
};
pub use perlin::Perlin;
pub use ray::Ray;
use std::{
    io::{self, prelude::*},
    sync::Arc,
    thread,
};
pub use texture::{Chequered, Noise, OpaqueImage, SolidColour, Texture};
pub use triplet::{Colour, Vector};

#[must_use]
fn ray_colour(r: &Ray, background: Colour, world: &dyn Hittable, depth: u32) -> Colour {
    // If we’ve exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.material().emitted(rec.u(), rec.v(), rec.p());

        if let Some((scattered, attenuation)) = rec.material().scatter(&r, &rec) {
            emitted + attenuation * ray_colour(&scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        // If the ray hits nothing, return the background colour.
        background
    }
}

#[allow(clippy::too_many_arguments)]
#[must_use]
fn render(
    thread_num: u32,
    world: &dyn Hittable,
    background: Colour,
    cam: &Camera,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> Box<[Colour]> {
    let mut pixels = Vec::with_capacity(image_width as usize * image_height as usize);

    for j in 0..image_height {
        if thread_num == 0 {
            let percent = (100.0 * f64::from(j) / f64::from(image_height)).round() as u32;
            print!(
                "\rMain thread scanlines remaining: {:5} ({:3}%)",
                image_height - j,
                percent
            );
            io::stdout().flush().expect("Error writing to stdout");
        }

        for i in 0..image_width {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + random::f64()) / f64::from(image_width - 1);
                let v = (f64::from(j) + random::f64()) / f64::from(image_height - 1);
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, background, world, max_depth);
            }
            pixels.push(pixel_colour);
        }
    }

    if thread_num == 0 {
        println!("\rMain thread scanlines remaining: {:5} ({:3}%)", 0, 100);
    }

    pixels.into_boxed_slice()
}

#[allow(clippy::too_many_arguments)]
#[must_use]
fn render_thread(
    thread_num: u32,
    world: Arc<dyn Hittable>,
    background: Colour,
    cam: Arc<Camera>,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> Box<[Colour]> {
    render(
        thread_num,
        world.as_ref(),
        background,
        cam.as_ref(),
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
}

#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn run(
    num_threads: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    image_width: u32,
    image_height: u32,
    world: Arc<dyn Hittable>,
    background: Colour,
    cam: Arc<Camera>,
) -> Box<[Colour]> {
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
            render_thread(
                thread_num,
                world,
                background,
                cam,
                image_width,
                image_height,
                samples_per_pixel,
                max_depth,
            )
        }));
    }

    // This thread.
    let mut pixels = render(
        0,
        world.as_ref(),
        background,
        cam.as_ref(),
        image_width,
        image_height,
        samples_per_thread,
        max_depth,
    );

    // Join threads.
    for (i, thread) in threads.into_iter().enumerate() {
        print!("\rWaiting for thread {:2} of {}...", i + 2, num_threads);
        io::stdout().flush().expect("Error writing to stdout");
        let thread_pixels = thread.join().expect("Worker thread error");
        assert_eq!(pixels.len(), thread_pixels.len());
        for (pixel, thread_pixel) in pixels.iter_mut().zip(thread_pixels.iter()) {
            *pixel += thread_pixel;
        }
    }
    println!();

    pixels
}
