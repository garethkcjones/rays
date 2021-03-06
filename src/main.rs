use rays::{
    random_f64, random_f64_in, Camera, Colour, Dielectric, Hittable, HittableList, Lambertian2,
    Material, Metal, Ray, Sphere, Vec3,
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material = Rc::new(Lambertian2::new(Colour::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let a = f64::from(a);
            let b = f64::from(b);

            let choose_mat = random_f64();

            let centre = Vec3::new(a + 0.9 * random_f64(), 0.2, b + 0.9 * random_f64());

            if (centre - Vec3::new(4.0, 0.2, 0.0)).abs() > 0.9 {
                let material: Rc<dyn Material> = match choose_mat {
                    x if x < 0.8 => {
                        // Diffuse.
                        let albedo = Colour::random() * Colour::random();
                        Rc::new(Lambertian2::new(albedo))
                    }
                    x if x < 0.95 => {
                        // Metal.
                        let albedo = Colour::random_in(0.5, 1.0);
                        let fuzz = random_f64_in(0.0, 0.5);
                        Rc::new(Metal::new(albedo, fuzz))
                    }
                    _ => {
                        // Glass.
                        Rc::new(Dielectric::new(1.5))
                    }
                };
                world.push(Box::new(Sphere::new(centre, 0.2, material)));
            }
        }
    }

    let material = Rc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Rc::new(Lambertian2::new(Colour::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Rc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    world
}

fn ray_colour(r: &Ray, world: &impl Hittable, depth: u32) -> Colour {
    // If we’ve exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
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

    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (f64::from(image_width) / aspect_ratio) as _;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World.

    let world = random_scene();

    // Camera.

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
            if let Err(error) = write_ppm_pixel(&mut output, pixel_colour, samples_per_pixel) {
                output_error(error);
            }
        }
    }

    if let Err(error) = output.flush() {
        output_error(error);
    }

    eprintln!("\nDone.");
}

fn write_ppm_pixel(
    mut output: impl Write,
    pixel: Colour,
    samples_per_pixel: u32,
) -> io::Result<()> {
    // Divide the colour by the number of samples and gamma-correct for gamma = 2.0.
    let scale = 1.0 / f64::from(samples_per_pixel);
    let r = (pixel.r * scale).sqrt();
    let g = (pixel.g * scale).sqrt();
    let b = (pixel.b * scale).sqrt();

    // Write the translated [0, 255] value of each colour component.
    let r = (256.0 * r.clamp(0.0, 0.999)) as u8;
    let g = (256.0 * g.clamp(0.0, 0.999)) as u8;
    let b = (256.0 * b.clamp(0.0, 0.999)) as u8;
    writeln!(output, "{} {} {}", r, g, b)
}
