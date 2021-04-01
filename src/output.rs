use crate::Colour;
use image::{ImageResult, Rgb, RgbImage};
use std::path::Path;

pub fn write_image_file(
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

pub fn write_pixel(buffer: &mut RgbImage, x: u32, y: u32, pixel: Colour, samples_per_pixel: u32) {
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
