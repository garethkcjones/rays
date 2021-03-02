use rays::{Colour, Ray, Vec3};

fn ray_colour(r: &Ray) -> Colour {
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

    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * WHITE + t * BLUE
}

fn main() {
    // Image.

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (f64::from(image_width) / aspect_ratio) as _;

    // Camera.

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    // Render.

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:5}", j);
        for i in 0..image_width {
            let u = f64::from(i) / f64::from(image_width - 1);
            let v = f64::from(j) / f64::from(image_height - 1);

            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let r = Ray { origin, direction };

            let pixel_colour = ray_colour(&r);

            println!("{}", pixel_colour);
        }
    }

    eprintln!("\nDone.");
}
