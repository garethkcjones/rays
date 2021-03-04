use rays::{
    Camera, Colour, Hittable, HittableList, Lambertian2, Material, Metal, Ray, Sphere, Vec3,
};
use std::{
    io::{self, prelude::*},
    process,
    rc::Rc,
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

fn ray_colour(r: &Ray, world: &impl Hittable, depth: i32) -> Colour {
    // If we’ve exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return BLACK;
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material().scatter(&r, &rec) {
            return attenuation * ray_colour(&scattered, world, depth - 1);
        } else {
            return BLACK;
        }
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
    let max_depth = 50;

    // World.

    let mut world = HittableList::new();

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian2::new(Colour::new(0.8, 0.8, 0.0)));
    let material_centre: Rc<dyn Material> = Rc::new(Lambertian2::new(Colour::new(0.7, 0.3, 0.3)));
    let material_left: Rc<dyn Material> = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3));
    let material_right: Rc<dyn Material> = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0));

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_centre),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_right),
    )));

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
                pixel_colour += ray_colour(&r, &world, max_depth);
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
