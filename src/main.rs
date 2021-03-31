use image::{ImageResult, Rgb, RgbImage};
use rays::{
    random_f64, random_f64_in, Camera, Chequered, Colour, Dielectric, Hittable, Lambertian2, Metal,
    MovingSphere, Noise, Ray, Sphere, Vector,
};
use std::{
    env,
    io::{self, prelude::*},
    path::Path,
    sync::Arc,
    thread,
};

#[must_use]
fn random_scene() -> Arc<dyn Hittable> {
    let mut world = Vec::<Arc<dyn Hittable>>::new();

    let texture = Chequered::new(
        Colour::new(0.9, 0.9, 0.9),
        Colour::new(0.2, 0.3, 0.1),
        Vector::new(10.0, 10.0, 10.0),
    );
    let material = Lambertian2::new(texture);
    world.push(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let a = f64::from(a);
            let b = f64::from(b);

            let choose_mat = random_f64();

            let centre = Vector::new(a + 0.9 * random_f64(), 0.2, b + 0.9 * random_f64());

            if (centre - Vector::new(4.0, 0.2, 0.0)).abs() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => {
                        // Diffuse.
                        let albedo = Colour::random() * Colour::random();
                        let material = Lambertian2::new(albedo);
                        let centre2 = centre + Vector::new(0.0, random_f64_in(0.0, 0.5), 0.0);
                        world.push(MovingSphere::new(centre, centre2, 0.0, 1.0, 0.2, material));
                    }
                    x if x < 0.95 => {
                        // Metal.
                        let albedo = Colour::random_in(0.5, 1.0);
                        let fuzz = random_f64_in(0.0, 0.5);
                        let material = Metal::new(albedo, fuzz);
                        world.push(Sphere::new(centre, 0.2, material));
                    }
                    _ => {
                        // Glass.
                        let material = Dielectric::new(1.5);
                        world.push(Sphere::new(centre, 0.2, material));
                    }
                }
            }
        }
    }

    let material = Dielectric::new(1.5);
    world.push(Sphere::new(Vector::new(0.0, 1.0, 0.0), 1.0, material));

    let material = Lambertian2::new(Colour::new(0.4, 0.2, 0.1));
    world.push(Sphere::new(Vector::new(-4.0, 1.0, 0.0), 1.0, material));

    let material = Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0);
    world.push(Sphere::new(Vector::new(4.0, 1.0, 0.0), 1.0, material));

    Arc::new(world)
}

#[must_use]
fn two_spheres() -> Arc<dyn Hittable> {
    let mut world = Vec::<Arc<dyn Hittable>>::new();
    let texture = Chequered::new(
        Colour::new(0.9, 0.9, 0.9),
        Colour::new(0.2, 0.3, 0.1),
        Vector::new(10.0, 10.0, 10.0),
    );
    let material = Lambertian2::new(texture);
    world.push(Sphere::new(
        Vector::new(0.0, -10.0, 0.0),
        10.0,
        Arc::clone(&material),
    ));
    world.push(Sphere::new(Vector::new(0.0, 10.0, 0.0), 10.0, material));
    Arc::new(world)
}

#[must_use]
fn two_perlin_spheres() -> Arc<dyn Hittable> {
    let mut world = Vec::<Arc<dyn Hittable>>::new();
    let texture = Noise::new(4.0);
    let material = Lambertian2::new(texture);
    world.push(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&material),
    ));
    world.push(Sphere::new(Vector::new(0.0, 2.0, 0.0), 2.0, material));
    Arc::new(world)
}

#[must_use]
fn ray_colour(r: &Ray, world: &dyn Hittable, depth: u32) -> Colour {
    let black = Colour::new(0.0, 0.0, 0.0);
    let white = Colour::new(1.0, 1.0, 1.0);
    let blue = Colour::new(0.5, 0.7, 1.0);

    // If we’ve exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return black;
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material().scatter(&r, &rec) {
            return attenuation * ray_colour(&scattered, world, depth - 1);
        } else {
            return black;
        }
    }

    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * white + t * blue
}

#[must_use]
fn render(
    thread_num: u32,
    world: &dyn Hittable,
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
                let u = (f64::from(i) + rays::random_f64()) / f64::from(image_width - 1);
                let v = (f64::from(j) + rays::random_f64()) / f64::from(image_height - 1);
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, world, max_depth);
            }
            pixels.push(pixel_colour);
        }
    }

    if thread_num == 0 {
        println!("\rMain thread scanlines remaining: {:5} ({:3}%)", 0, 100);
    }

    pixels.into_boxed_slice()
}

#[must_use]
fn render_thread(
    thread_num: u32,
    world: Arc<dyn Hittable>,
    cam: Arc<Camera>,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> Box<[Colour]> {
    render(
        thread_num,
        world.as_ref(),
        cam.as_ref(),
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    )
}

fn main() {
    let scene_choice: u32 = env::args()
        .nth(1)
        .expect("Missing command-line argument")
        .parse()
        .expect("Invalid command-line argument");

    // Image.
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (f64::from(image_width) / aspect_ratio) as _;

    let (world, cam) = match scene_choice {
        1 => {
            // World.
            let world = random_scene();

            // Camera.
            let look_from = Vector::new(13.0, 2.0, 3.0);
            let look_at = Vector::new(0.0, 0.0, 0.0);
            let vup = Vector::new(0.0, 1.0, 0.0);
            let vfov = 20.0;
            let aperture = 0.1;
            let dist_to_focus = 10.0;
            let cam = Arc::new(Camera::new(
                look_from,
                look_at,
                vup,
                vfov,
                aspect_ratio,
                aperture,
                dist_to_focus,
                0.0,
                1.0,
            ));

            (world, cam)
        }
        2 => {
            // World.
            let world = two_spheres();

            // Camera.
            let look_from = Vector::new(13.0, 2.0, 3.0);
            let look_at = Vector::new(0.0, 0.0, 0.0);
            let vup = Vector::new(0.0, 1.0, 0.0);
            let vfov = 20.0;
            let aperture = 0.0;
            let dist_to_focus = 10.0;
            let cam = Arc::new(Camera::new(
                look_from,
                look_at,
                vup,
                vfov,
                aspect_ratio,
                aperture,
                dist_to_focus,
                0.0,
                1.0,
            ));

            (world, cam)
        }
        3 => {
            // World.
            let world = two_perlin_spheres();

            // Camera.
            let look_from = Vector::new(13.0, 2.0, 3.0);
            let look_at = Vector::new(0.0, 0.0, 0.0);
            let vup = Vector::new(0.0, 1.0, 0.0);
            let vfov = 20.0;
            let aperture = 0.0;
            let dist_to_focus = 10.0;
            let cam = Arc::new(Camera::new(
                look_from,
                look_at,
                vup,
                vfov,
                aspect_ratio,
                aperture,
                dist_to_focus,
                0.0,
                1.0,
            ));

            (world, cam)
        }
        _ => panic!("Invalid command-line argument"),
    };

    // Render.
    let samples_per_pixel = 100;
    let max_depth = 50;
    let num_threads = 32;
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

    // Output.
    println!("Writing output...");
    write_image_file(
        "out.png",
        image_width,
        image_height,
        &pixels,
        samples_per_pixel,
    )
    .expect("Error writing output");

    println!("Done.");
}

fn write_image_file(
    output: impl AsRef<Path>,
    image_width: u32,
    image_height: u32,
    pixels: &[Colour],
    samples_per_pixel: u32,
) -> ImageResult<()> {
    let mut buffer = RgbImage::new(image_width, image_height);

    for y in 0..image_height {
        let x0 = y as usize * image_width as usize;
        let y = image_height - y - 1;
        for x in 0..image_width {
            let idx = x0 + x as usize;
            let pixel = pixels[idx];
            write_pixel(&mut buffer, x, y, pixel, samples_per_pixel);
        }
    }

    buffer.save(output)
}

fn write_pixel(buffer: &mut RgbImage, x: u32, y: u32, pixel: Colour, samples_per_pixel: u32) {
    // Divide the colour by the number of samples and gamma-correct for gamma = 2.0.
    let scale = 1.0 / f64::from(samples_per_pixel);
    let r = (pixel.r() * scale).sqrt();
    let g = (pixel.g() * scale).sqrt();
    let b = (pixel.b() * scale).sqrt();

    // Write the translated [0, 255] value of each colour component.
    let r = (256.0 * r.clamp(0.0, 0.999)) as u8;
    let g = (256.0 * g.clamp(0.0, 0.999)) as u8;
    let b = (256.0 * b.clamp(0.0, 0.999)) as u8;

    buffer.put_pixel(x, y, Rgb([r, g, b]));
}
