use rays::{Camera, Colour, Dielectric, Lambertian2, Metal, Sphere, Vec3};
use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    io::{self, prelude::*, BufWriter},
    path::Path,
    process,
};

/**
 * Builds and renders a scene.
 */
fn render(output: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    // Image.

    let image_aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (f64::from(image_width) / image_aspect_ratio) as _;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World.

    let material_ground = Lambertian2::new_material(Colour(0.8, 0.8, 0.0));
    let material_centre = Lambertian2::new_material(Colour(0.1, 0.2, 0.5));
    let material_left = Dielectric::new_material(1.5);
    let material_right = Metal::new_material(Colour(0.8, 0.6, 0.2), 0.0);

    let world = vec![
        Sphere::new_hittable(Vec3(0.0, -100.5, -1.0), 100.0, material_ground),
        Sphere::new_hittable(Vec3(0.0, 0.0, -1.0), 0.5, material_centre),
        Sphere::new_hittable(Vec3(-1.0, 0.0, -1.0), 0.5, material_left.clone()),
        Sphere::new_hittable(Vec3(-1.0, 0.0, -1.0), -0.45, material_left),
        Sphere::new_hittable(Vec3(1.0, 0.0, -1.0), 0.5, material_right),
    ];

    // Camera.

    let viewport_aspect_ratio = f64::from(image_width) / f64::from(image_height);

    let cam = Camera::new(
        Vec3(-2.0, 2.0, 1.0),
        Vec3(0.0, 0.0, -1.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        viewport_aspect_ratio,
    );

    // Render.

    rays::render(
        &world,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        &cam,
        output,
        true,
    )
}

/**
 * Runs the program.
 */
fn run(args: &[OsString]) -> Result<(), Box<dyn Error>> {
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
