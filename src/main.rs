use rays::{Camera, Colour, Hittable, HittableList, Ray, Sphere, Vec3};
use std::{
    io::{self, prelude::*},
    process,
};

const BLACK: Colour = Colour {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

const WHITE: Colour = Colour {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

const BLUE: Colour = Colour {
    r: 0.5,
    g: 0.7,
    b: 1.0,
};

fn output_error(error: io::Error) -> ! {
    eprintln!("\nError writing output: {}", error);
    process::exit(1);
}

fn ray_colour(r: &Ray, world: &impl Hittable) -> Colour {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5
            * Colour {
                r: rec.normal.x + 1.0,
                g: rec.normal.y + 1.0,
                b: rec.normal.z + 1.0,
            };
    }

    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * WHITE + t * BLUE
}

fn main() {
    // Image.

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (f64::from(image_width) / aspect_ratio) as _;
    let samples_per_pixel = 100;

    // World.

    let mut world = HittableList::new();

    world.push(Box::new(Sphere {
        centre: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    }));
    world.push(Box::new(Sphere {
        centre: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    }));

    let world = world;

    // Camera.

    let cam = Camera::new();

    // Render.

    let output = io::stdout();
    let mut output = output.lock();
    if let Err(error) = writeln!(&mut output, "P3\n{} {}\n255", image_width, image_height) {
        output_error(error);
    }

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:5}", j);
        for i in 0..image_width {
            let mut pixel_colour = BLACK;
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + rays::random_f64()) / f64::from(image_width - 1);
                let v = (f64::from(j) + rays::random_f64()) / f64::from(image_height - 1);
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world);
            }
            if let Err(error) = pixel_colour.write_to(&mut output, samples_per_pixel) {
                output_error(error);
            }
        }
    }

    if let Err(error) = output.flush() {
        output_error(error);
    }

    eprintln!("\nDone.");
}
