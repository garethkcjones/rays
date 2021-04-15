use rand::prelude::*;
use rays::{Camera, Colour, Dielectric, Hittable, Lambertian2, Metal, Sphere, Vec3};
use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    io::{self, prelude::*, BufWriter},
    path::Path,
    process,
    sync::Arc,
};

#[must_use]
fn random_scene() -> Arc<dyn Hittable> {
    let mut rand_eng = thread_rng();
    let rand_dst = rand::distributions::Uniform::new(0.0, 1.0);

    let mut world = Vec::new();

    let ground_material = Lambertian2::new_material(Colour(0.5, 0.5, 0.5));
    world.push(Sphere::new_hittable(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        let a = f64::from(a);
        for b in -11..11 {
            let b = f64::from(b);

            let choose_mat = rand_eng.sample(rand_dst);
            let centre = Vec3(
                a + 0.9 * rand_eng.sample(rand_dst),
                0.2,
                b + 0.9 * rand_eng.sample(rand_dst),
            );

            if (centre - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse.
                    let albedo = Colour::new_random(0.0, 1.0) * Colour::new_random(0.0, 1.0);
                    let sphere_material = Lambertian2::new_material(albedo);
                    world.push(Sphere::new_hittable(centre, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // Metal.
                    let albedo = Colour::new_random(0.5, 1.0);
                    let fuzz = 0.5 * rand_eng.sample(rand_dst);
                    let sphere_material = Metal::new_material(albedo, fuzz);
                    world.push(Sphere::new_hittable(centre, 0.2, sphere_material));
                } else {
                    // Glass.
                    let sphere_material = Dielectric::new_material(1.5);
                    world.push(Sphere::new_hittable(centre, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new_material(1.5);
    world.push(Sphere::new_hittable(Vec3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian2::new_material(Colour(0.4, 0.2, 0.1));
    world.push(Sphere::new_hittable(Vec3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new_material(Colour(0.7, 0.6, 0.5), 0.0);
    world.push(Sphere::new_hittable(Vec3(4.0, 1.0, 0.0), 1.0, material3));

    Arc::new(world)
}

/**
 * Builds and renders a scene.
 */
fn render(output: &mut dyn Write) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Image.

    let image_aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (f64::from(image_width) / image_aspect_ratio) as _;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World.

    let world = random_scene();

    // Camera.

    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aspect_ratio = f64::from(image_width) / f64::from(image_height);
    let aperture = 0.1;
    let dist_to_focus = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    let cam = Arc::new(cam);

    // Render.

    let num_threads = 32;

    rays::run(
        num_threads,
        world,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        cam,
        output,
        true,
    )
}

/**
 * Runs the program.
 */
fn run(args: &[OsString]) -> Result<(), Box<dyn Error + Send + Sync>> {
    match args.len() {
        0 | 1 => {
            // No output file name specified on command-line.  Use stdout.
            render(&mut io::stdout().lock())?;
        }

        2 => {
            // Get the output file name from the command-line.
            let filename = Path::new(&args[1]);

            let mut output = match File::create(filename) {
                Ok(output) => BufWriter::new(output),
                Err(x) => {
                    return Err(
                        format!("cannot open output file “{}”: {}", filename.display(), x).into(),
                    )
                }
            };

            render(&mut output)?;

            if let Err(x) = output.flush() {
                return Err(format!("error writing to “{}”: {}", filename.display(), x).into());
            }
        }

        _ => {
            return Err("too many command-line arguments".into());
        }
    }

    Ok(())
}

/**
 * Returns the program name from the command-line.
 */
#[must_use]
fn get_progname(args: &[OsString]) -> &str {
    if let Some(progname) = args.get(0) {
        if let Some(progname) = Path::new(progname).file_name() {
            if let Some(progname) = progname.to_str() {
                if !progname.is_empty() {
                    return progname;
                }
            }
        }
    }
    "rays"
}

/**
 * Entry point.
 *
 * Usage: `rays [OUTPUT_FILE]`
 */
fn main() {
    let args: Vec<_> = env::args_os().collect();

    let progname = get_progname(&args);

    if let Err(x) = run(&args) {
        eprintln!("{}: {}", progname, x);
        process::exit(1);
    }
}
