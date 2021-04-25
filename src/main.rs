use rand::prelude::*;
use rays::{
    Block, Camera, Chequer, Colour, ConstantMedium, Dielectric, DiffuseLight, Hittable, Image,
    Lambertian2, Metal, MovingSphere, Noise, RotateY, Sphere, Translate, Vec3, XyRect, XzRect,
    YzRect,
};
use std::{
    env,
    error::Error,
    ffi::{OsStr, OsString},
    fs::File,
    io::{self, prelude::*, BufWriter},
    path::Path,
    process,
    sync::Arc,
};

#[must_use]
fn random_scene() -> Arc<dyn Hittable> {
    let mut rand_eng = thread_rng();

    let mut world = Vec::new();

    let chequer = Chequer::new_texture(
        Vec3(10.0, 10.0, 10.0),
        Colour(0.2, 0.3, 0.1),
        Colour(0.9, 0.9, 0.9),
    );
    let ground_material = Lambertian2::new_material(chequer);
    world.push(Sphere::new_hittable(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        let a = f64::from(a);
        for b in -11..11 {
            let b = f64::from(b);

            let choose_mat: f64 = rand_eng.gen();
            let centre = Vec3(
                a + rand_eng.gen_range(0.0..0.9),
                0.2,
                b + rand_eng.gen_range(0.0..0.9),
            );

            if (centre - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse.
                    let albedo = Colour::new_random(0.0..1.0) * Colour::new_random(0.0..1.0);
                    let sphere_material = Lambertian2::new_material(albedo);
                    let centre2 = centre + Vec3(0.0, rand_eng.gen_range(0.0..0.5), 0.0);
                    world.push(MovingSphere::new_hittable(
                        centre,
                        centre2,
                        0.0..1.0,
                        0.2,
                        sphere_material,
                    ));
                } else if choose_mat < 0.95 {
                    // Metal.
                    let albedo = Colour::new_random(0.5..1.0);
                    let fuzz = rand_eng.gen_range(0.0..0.5);
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

#[must_use]
fn two_spheres() -> Arc<dyn Hittable> {
    let chequer = Chequer::new_texture(
        Vec3(10.0, 10.0, 10.0),
        Colour(0.2, 0.3, 0.1),
        Colour(0.9, 0.9, 0.9),
    );

    let objects = vec![
        Sphere::new_hittable(
            Vec3(0.0, -10.0, 0.0),
            10.0,
            Lambertian2::new_material(chequer.clone()),
        ),
        Sphere::new_hittable(
            Vec3(0.0, 10.0, 0.0),
            10.0,
            Lambertian2::new_material(chequer),
        ),
    ];

    Arc::new(objects)
}

#[must_use]
fn two_perlin_spheres() -> Arc<dyn Hittable> {
    let pertext = Noise::new_texture(4.0);
    let objects = vec![
        Sphere::new_hittable(
            Vec3(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian2::new_material(pertext.clone()),
        ),
        Sphere::new_hittable(Vec3(0.0, 2.0, 0.0), 2.0, Lambertian2::new_material(pertext)),
    ];

    Arc::new(objects)
}

fn earth() -> Result<Arc<dyn Hittable>, Box<dyn Error + Send + Sync>> {
    let earth_texture = Image::new_texture("earthmap.jpg")?;
    let earth_surface = Lambertian2::new_material(earth_texture);
    let globe = Sphere::new_hittable(Vec3(0.0, 0.0, 0.0), 2.0, earth_surface);
    Ok(globe)
}

#[must_use]
fn simple_light() -> Arc<dyn Hittable> {
    let pertext = Noise::new_texture(4.0);
    let difflight = DiffuseLight::new_material(Colour(4.0, 4.0, 4.0));

    let objects = vec![
        Sphere::new_hittable(
            Vec3(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian2::new_material(pertext.clone()),
        ),
        Sphere::new_hittable(Vec3(0.0, 2.0, 0.0), 2.0, Lambertian2::new_material(pertext)),
        XyRect::new_hittable(3.0..5.0, 1.0..3.0, -2.0, difflight),
    ];

    Arc::new(objects)
}

#[must_use]
fn cornell_box() -> Arc<dyn Hittable> {
    let red = Lambertian2::new_material(Colour(0.65, 0.05, 0.05));
    let white = Lambertian2::new_material(Colour(0.73, 0.73, 0.73));
    let green = Lambertian2::new_material(Colour(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_material(Colour(15.0, 15.0, 15.0));

    let mut box1 = Block::new_hittable(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = RotateY::new_hittable(box1, 15.0);
    box1 = Translate::new_hittable(box1, Vec3(265.0, 0.0, 295.0));

    let mut box2 = Block::new_hittable(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 165.0, 165.0),
        white.clone(),
    );
    box2 = RotateY::new_hittable(box2, -18.0);
    box2 = Translate::new_hittable(box2, Vec3(130.0, 0.0, 65.0));

    let objects = vec![
        YzRect::new_hittable(0.0..555.0, 0.0..555.0, 555.0, green),
        YzRect::new_hittable(0.0..555.0, 0.0..555.0, 0.0, red),
        XzRect::new_hittable(213.0..343.0, 227.0..332.0, 554.0, light),
        XzRect::new_hittable(0.0..555.0, 0.0..555.0, 0.0, white.clone()),
        XzRect::new_hittable(0.0..555.0, 0.0..555.0, 555.0, white.clone()),
        XyRect::new_hittable(0.0..555.0, 0.0..555.0, 555.0, white),
        box1,
        box2,
    ];

    Arc::new(objects)
}

#[must_use]
fn cornell_smoke() -> Arc<dyn Hittable> {
    let red = Lambertian2::new_material(Colour(0.65, 0.05, 0.05));
    let white = Lambertian2::new_material(Colour(0.73, 0.73, 0.73));
    let green = Lambertian2::new_material(Colour(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_material(Colour(7.0, 7.0, 7.0));

    let mut box1 = Block::new_hittable(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = RotateY::new_hittable(box1, 15.0);
    box1 = Translate::new_hittable(box1, Vec3(265.0, 0.0, 295.0));
    box1 = ConstantMedium::new_hittable(box1, 0.01, Colour(0.0, 0.0, 0.0));

    let mut box2 = Block::new_hittable(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 165.0, 165.0),
        white.clone(),
    );
    box2 = RotateY::new_hittable(box2, -18.0);
    box2 = Translate::new_hittable(box2, Vec3(130.0, 0.0, 65.0));
    box2 = ConstantMedium::new_hittable(box2, 0.01, Colour(1.0, 1.0, 1.0));

    let objects = vec![
        YzRect::new_hittable(0.0..555.0, 0.0..555.0, 555.0, green),
        YzRect::new_hittable(0.0..555.0, 0.0..555.0, 0.0, red),
        XzRect::new_hittable(113.0..443.0, 127.0..432.0, 554.0, light),
        XzRect::new_hittable(0.0..555.0, 0.0..555.0, 0.0, white.clone()),
        XzRect::new_hittable(0.0..555.0, 0.0..555.0, 555.0, white.clone()),
        XyRect::new_hittable(0.0..555.0, 0.0..555.0, 555.0, white),
        box1,
        box2,
    ];

    Arc::new(objects)
}

/**
 * Builds and renders a scene.
 */
fn render(scene: u32, output: &mut dyn Write) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Scene parameters.
    let (
        world,
        background,
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        time0,
        time1,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    );

    match scene {
        1 => {
            // Image.
            let image_aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            image_height = (f64::from(image_width) / image_aspect_ratio) as _;
            samples_per_pixel = 100;
            max_depth = 50;

            // World.
            world = random_scene();
            background = Colour(0.7, 0.8, 1.0);

            // Camera.
            lookfrom = Vec3(13.0, 2.0, 3.0);
            lookat = Vec3(0.0, 0.0, 0.0);
            vup = Vec3(0.0, 1.0, 0.0);
            vfov = 20.0;
            aspect_ratio = f64::from(image_width) / f64::from(image_height);
            aperture = 0.1;
            dist_to_focus = 10.0;
            time0 = 0.0;
            time1 = 1.0;
        }

        2 => {
            // Image.
            let image_aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            image_height = (f64::from(image_width) / image_aspect_ratio) as _;
            samples_per_pixel = 100;
            max_depth = 50;

            // World.
            world = two_spheres();
            background = Colour(0.7, 0.8, 1.0);

            // Camera.
            lookfrom = Vec3(13.0, 2.0, 3.0);
            lookat = Vec3(0.0, 0.0, 0.0);
            vup = Vec3(0.0, 1.0, 0.0);
            vfov = 20.0;
            aspect_ratio = f64::from(image_width) / f64::from(image_height);
            aperture = 0.0;
            dist_to_focus = 10.0;
            time0 = 0.0;
            time1 = 1.0;
        }

        3 => {
            // Image.
            let image_aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            image_height = (f64::from(image_width) / image_aspect_ratio) as _;
            samples_per_pixel = 100;
            max_depth = 50;

            // World.
            world = two_perlin_spheres();
            background = Colour(0.7, 0.8, 1.0);

            // Camera.
            lookfrom = Vec3(13.0, 2.0, 3.0);
            lookat = Vec3(0.0, 0.0, 0.0);
            vup = Vec3(0.0, 1.0, 0.0);
            vfov = 20.0;
            aspect_ratio = f64::from(image_width) / f64::from(image_height);
            aperture = 0.0;
            dist_to_focus = 10.0;
            time0 = 0.0;
            time1 = 1.0;
        }

        4 => {
            // Image.
            let image_aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            image_height = (f64::from(image_width) / image_aspect_ratio) as _;
            samples_per_pixel = 100;
            max_depth = 50;

            // World.
            world = earth()?;
            background = Colour(0.7, 0.8, 1.0);

            // Camera.
            lookfrom = Vec3(13.0, 2.0, 3.0);
            lookat = Vec3(0.0, 0.0, 0.0);
            vup = Vec3(0.0, 1.0, 0.0);
            vfov = 20.0;
            aspect_ratio = f64::from(image_width) / f64::from(image_height);
            aperture = 0.0;
            dist_to_focus = 10.0;
            time0 = 0.0;
            time1 = 1.0;
        }

        5 => {
            // Image.
            let image_aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            image_height = (f64::from(image_width) / image_aspect_ratio) as _;
            samples_per_pixel = 400;
            max_depth = 50;

            // World.
            world = simple_light();
            background = Colour(0.0, 0.0, 0.0);

            // Camera.
            lookfrom = Vec3(26.0, 3.0, 6.0);
            lookat = Vec3(0.0, 2.0, 0.0);
            vup = Vec3(0.0, 1.0, 0.0);
            vfov = 20.0;
            aspect_ratio = f64::from(image_width) / f64::from(image_height);
            aperture = 0.0;
            dist_to_focus = 10.0;
            time0 = 0.0;
            time1 = 1.0;
        }

        6 => {
            // Image.
            let image_aspect_ratio = 1.0;
            image_width = 600;
            image_height = (f64::from(image_width) / image_aspect_ratio) as _;
            samples_per_pixel = 200;
            max_depth = 50;

            // World.
            world = cornell_box();
            background = Colour(0.0, 0.0, 0.0);

            // Camera.
            lookfrom = Vec3(278.0, 278.0, -800.0);
            lookat = Vec3(278.0, 278.0, 0.0);
            vup = Vec3(0.0, 1.0, 0.0);
            vfov = 40.0;
            aspect_ratio = f64::from(image_width) / f64::from(image_height);
            aperture = 0.0;
            dist_to_focus = 10.0;
            time0 = 0.0;
            time1 = 1.0;
        }

        7 => {
            // Image.
            let image_aspect_ratio = 1.0;
            image_width = 600;
            image_height = (f64::from(image_width) / image_aspect_ratio) as _;
            samples_per_pixel = 200;
            max_depth = 50;

            // World.
            world = cornell_smoke();
            background = Colour(0.0, 0.0, 0.0);

            // Camera.
            lookfrom = Vec3(278.0, 278.0, -800.0);
            lookat = Vec3(278.0, 278.0, 0.0);
            vup = Vec3(0.0, 1.0, 0.0);
            vfov = 40.0;
            aspect_ratio = f64::from(image_width) / f64::from(image_height);
            aperture = 0.0;
            dist_to_focus = 10.0;
            time0 = 0.0;
            time1 = 1.0;
        }

        x => return Err(format!("invalid scene number: {}", x).into()),
    }

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        time0..time1,
    );
    let cam = Arc::new(cam);

    // Render.

    let num_threads = 32;

    rays::run(
        num_threads,
        world,
        background,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        cam,
        output,
        true,
    )
}

fn scene_number(arg: &OsStr) -> Result<u32, Box<dyn Error + Send + Sync>> {
    match arg.to_str() {
        Some(arg) => arg
            .parse()
            .map_err(|x| format!("invalid scene number “{}”: {}", arg, x).into()),
        None => Err(format!("invalid scene number “{}”", arg.to_string_lossy()).into()),
    }
}

/**
 * Runs the program.
 */
fn run(args: &[OsString]) -> Result<(), Box<dyn Error + Send + Sync>> {
    match args.len() {
        0 | 1 => return Err("no scene number specified".into()),

        2 => {
            let scene = scene_number(&args[1])?;

            // No output file name specified on command-line.  Use stdout.
            render(scene, &mut io::stdout().lock())?;
        }

        3 => {
            let scene = scene_number(&args[1])?;

            // Get the output file name from the command-line.
            let filename = Path::new(&args[2]);

            let mut output = match File::create(filename) {
                Ok(output) => BufWriter::new(output),
                Err(x) => {
                    return Err(
                        format!("cannot open output file “{}”: {}", filename.display(), x).into(),
                    )
                }
            };

            render(scene, &mut output)?;

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
