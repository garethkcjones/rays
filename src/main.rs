use rays::{
    output, random, Camera, Chequered, Colour, Cuboid, Dielectric, DiffuseLight, Hittable,
    Lambertian2, Metal, MovingSphere, Noise, OpaqueImage, Sphere, Vector, XyRect, XzRect, YzRect,
};
use std::{env, sync::Arc};

#[must_use]
fn random_scene() -> Arc<dyn Hittable> {
    let mut world = Vec::new();

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

            let choose_mat = random::f64();

            let centre = Vector::new(a + 0.9 * random::f64(), 0.2, b + 0.9 * random::f64());

            if (centre - Vector::new(4.0, 0.2, 0.0)).abs() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => {
                        // Diffuse.
                        let albedo = Colour::random() * Colour::random();
                        let material = Lambertian2::new(albedo);
                        let centre2 = centre + Vector::new(0.0, random::f64_in(0.0, 0.5), 0.0);
                        world.push(MovingSphere::new(centre, centre2, 0.0, 1.0, 0.2, material));
                    }
                    x if x < 0.95 => {
                        // Metal.
                        let albedo = Colour::random_in(0.5, 1.0);
                        let fuzz = random::f64_in(0.0, 0.5);
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
    let mut world = Vec::new();
    let texture = Chequered::new(
        Colour::new(0.9, 0.9, 0.9),
        Colour::new(0.2, 0.3, 0.1),
        Vector::new(10.0, 10.0, 10.0),
    );
    let material = Lambertian2::new(texture);
    world.push(Sphere::new(
        Vector::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    ));
    world.push(Sphere::new(Vector::new(0.0, 10.0, 0.0), 10.0, material));
    Arc::new(world)
}

#[must_use]
fn two_perlin_spheres() -> Arc<dyn Hittable> {
    let mut world = Vec::new();
    let texture = Noise::new(4.0);
    let material = Lambertian2::new(texture);
    world.push(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    ));
    world.push(Sphere::new(Vector::new(0.0, 2.0, 0.0), 2.0, material));
    Arc::new(world)
}

#[must_use]
fn earth() -> Arc<dyn Hittable> {
    let texture = OpaqueImage::new("earthmap.jpg");
    let material = Lambertian2::new(texture);
    Sphere::new(Vector::new(0.0, 0.0, 0.0), 2.0, material)
}

#[must_use]
fn simple_light() -> Arc<dyn Hittable> {
    let mut world = Vec::new();
    let texture = Noise::new(4.0);
    let material = Lambertian2::new(texture);
    world.push(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        material.clone(),
    ));
    world.push(Sphere::new(Vector::new(0.0, 2.0, 0.0), 2.0, material));
    let material = DiffuseLight::new(Colour::new(4.0, 4.0, 4.0));
    world.push(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, material));
    Arc::new(world)
}

#[must_use]
fn cornell_box() -> Arc<dyn Hittable> {
    let mut world = Vec::new();

    let red = Lambertian2::new(Colour::new(0.65, 0.05, 0.05));
    let white = Lambertian2::new(Colour::new(0.73, 0.73, 0.73));
    let green = Lambertian2::new(Colour::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Colour::new(15.0, 15.0, 15.0));

    // Cornell Box.
    world.push(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
    world.push(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    world.push(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
    world.push(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.push(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));

    // Light.
    world.push(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light));

    // Contents.
    world.push(Cuboid::new(
        Vector::new(130.0, 0.0, 65.0),
        Vector::new(295.0, 165.0, 230.0),
        white.clone(),
    ));
    world.push(Cuboid::new(
        Vector::new(265.0, 0.0, 295.0),
        Vector::new(430.0, 330.0, 460.0),
        white,
    ));

    Arc::new(world)
}

fn main() {
    let scene_choice: u32 = env::args()
        .nth(1)
        .expect("Missing command-line argument")
        .parse()
        .expect("Invalid command-line argument");

    // Image.
    let aspect_ratio = 1.0;
    let image_width = 600;
    let image_height = (f64::from(image_width) / aspect_ratio) as _;

    let (world, background, cam) = match scene_choice {
        1 => {
            // World.
            let world = random_scene();
            let background = Colour::new(0.7, 0.8, 1.0);

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

            (world, background, cam)
        }
        2 => {
            // World.
            let world = two_spheres();
            let background = Colour::new(0.7, 0.8, 1.0);

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

            (world, background, cam)
        }
        3 => {
            // World.
            let world = two_perlin_spheres();
            let background = Colour::new(0.7, 0.8, 1.0);

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

            (world, background, cam)
        }
        4 => {
            // World.
            let world = earth();
            let background = Colour::new(0.7, 0.8, 1.0);

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

            (world, background, cam)
        }
        5 => {
            // World.
            let world = simple_light();
            let background = Colour::new(0.0, 0.0, 0.0);

            // Camera.
            let look_from = Vector::new(26.0, 3.0, 6.0);
            let look_at = Vector::new(0.0, 2.0, 0.0);
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

            (world, background, cam)
        }
        6 => {
            // World.
            let world = cornell_box();
            let background = Colour::new(0.0, 0.0, 0.0);

            // Camera.
            let look_from = Vector::new(278.0, 278.0, -800.0);
            let look_at = Vector::new(278.0, 278.0, 0.0);
            let vup = Vector::new(0.0, 1.0, 0.0);
            let vfov = 40.0;
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

            (world, background, cam)
        }
        _ => panic!("Invalid command-line argument"),
    };

    // Render.
    let samples_per_pixel = 200;
    let max_depth = 50;
    let num_threads = 32;
    let pixels = rays::run(
        num_threads,
        samples_per_pixel,
        max_depth,
        image_width,
        image_height,
        world,
        background,
        cam,
    );

    // Output.
    println!("Writing output...");
    output::write_image_file(
        "out.png",
        image_width,
        image_height,
        &pixels,
        samples_per_pixel,
    )
    .expect("Error writing output");

    println!("Done.");
}
