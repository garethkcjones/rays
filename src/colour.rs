use std::ops;

/**
 * Type for representing colours.
 */
#[derive(Clone, Copy, Debug, Default)]
pub struct Colour(pub f64, pub f64, pub f64);

impl Colour {
    #[must_use]
    pub const fn r(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn g(self) -> f64 {
        self.1
    }

    #[must_use]
    pub const fn b(self) -> f64 {
        self.2
    }

    #[must_use]
    pub fn to_rgb8(self, samples_per_pixel: u32) -> (u8, u8, u8) {
        assert!(samples_per_pixel > 0);

        // Divide the colour by the number of samples.
        let scale = f64::from(samples_per_pixel).recip();
        let Colour(mut r, mut g, mut b) = self * scale;

        r = r.clamp(0.0, 1.0);
        g = g.clamp(0.0, 1.0);
        b = b.clamp(0.0, 1.0);

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        (ir, ig, ib)
    }
}

impl ops::Add for Colour {
    type Output = Self;
    fn add(self, c: Self) -> Self::Output {
        let Colour(r1, g1, b1) = self;
        let Colour(r2, g2, b2) = c;
        let r = r1 + r2;
        let g = g1 + g2;
        let b = b1 + b2;
        Self(r, g, b)
    }
}

impl ops::Mul for Colour {
    type Output = Self;
    fn mul(self, c: Self) -> Self::Output {
        let Colour(r1, g1, b1) = self;
        let Colour(r2, g2, b2) = c;
        let r = r1 * r2;
        let g = g1 * g2;
        let b = b1 * b2;
        Self(r, g, b)
    }
}

impl ops::Mul<f64> for Colour {
    type Output = Self;
    fn mul(self, s: f64) -> Self::Output {
        let Colour(r, g, b) = self;
        let r = r * s;
        let g = g * s;
        let b = b * s;
        Self(r, g, b)
    }
}

impl ops::Div<f64> for Colour {
    type Output = Self;
    fn div(self, s: f64) -> Self::Output {
        let Colour(r, g, b) = self;
        let r = r / s;
        let g = g / s;
        let b = b / s;
        Self(r, g, b)
    }
}

impl ops::Mul<Colour> for f64 {
    type Output = Colour;
    fn mul(self, c: Colour) -> Self::Output {
        let Colour(r, g, b) = c;
        let r = self * r;
        let g = self * g;
        let b = self * b;
        Colour(r, g, b)
    }
}

impl ops::AddAssign for Colour {
    fn add_assign(&mut self, c: Self) {
        *self = *self + c;
    }
}

impl ops::MulAssign for Colour {
    fn mul_assign(&mut self, c: Self) {
        *self = *self * c;
    }
}

impl ops::MulAssign<f64> for Colour {
    fn mul_assign(&mut self, s: f64) {
        *self = *self * s;
    }
}

impl ops::DivAssign<f64> for Colour {
    fn div_assign(&mut self, s: f64) {
        *self = *self / s;
    }
}
