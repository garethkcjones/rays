use crate::{random_f64, random_f64_in};
use std::{
    fmt,
    io::{self, prelude::*},
    ops,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    #[must_use]
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    #[must_use]
    pub fn random() -> Self {
        let r = random_f64();
        let g = random_f64();
        let b = random_f64();
        Self { r, g, b }
    }

    #[must_use]
    pub fn random_in(min: f64, max: f64) -> Self {
        let r = random_f64_in(min, max);
        let g = random_f64_in(min, max);
        let b = random_f64_in(min, max);
        Self { r, g, b }
    }

    pub fn write_to(&self, mut output: impl Write, samples_per_pixel: i32) -> io::Result<()> {
        // Divide the colour by the number of samples and gamma-correct for gamma = 2.0.
        let scale = 1.0 / f64::from(samples_per_pixel);
        let r = (self.r * scale).sqrt();
        let g = (self.g * scale).sqrt();
        let b = (self.b * scale).sqrt();

        // Write the translated [0, 255] value of each colour component.
        let r = (256.0 * r.clamp(0.0, 0.999)) as i32;
        let g = (256.0 * g.clamp(0.0, 0.999)) as i32;
        let b = (256.0 * b.clamp(0.0, 0.999)) as i32;
        writeln!(output, "{} {} {}", r, g, b)
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

impl From<(f64, f64, f64)> for Colour {
    fn from(v: (f64, f64, f64)) -> Self {
        Self {
            r: v.0,
            g: v.1,
            b: v.2,
        }
    }
}

impl From<([f64; 3])> for Colour {
    fn from(v: [f64; 3]) -> Self {
        Self {
            r: v[0],
            g: v[1],
            b: v[2],
        }
    }
}

// Vector operators.

impl ops::Neg for Colour {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let Self { r, g, b } = self;
        let r = -r;
        let g = -g;
        let b = -b;
        Self { r, g, b }
    }
}

impl ops::Add for Colour {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let r = self.r + other.r;
        let g = self.g + other.g;
        let b = self.b + other.b;
        Self { r, g, b }
    }
}

impl ops::Sub for Colour {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let r = self.r - other.r;
        let g = self.g - other.g;
        let b = self.b - other.b;
        Self { r, g, b }
    }
}

impl ops::Mul for Colour {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let r = self.r * other.r;
        let g = self.g * other.g;
        let b = self.b * other.b;
        Self { r, g, b }
    }
}

impl ops::Div for Colour {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        let r = self.r / other.r;
        let g = self.g / other.g;
        let b = self.b / other.b;
        Self { r, g, b }
    }
}

// Scalar operators.

impl ops::Add<f64> for Colour {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        let r = self.r + other;
        let g = self.g + other;
        let b = self.b + other;
        Self { r, g, b }
    }
}

impl ops::Add<Colour> for f64 {
    type Output = Colour;
    fn add(self, other: Colour) -> Self::Output {
        let r = self + other.r;
        let g = self + other.g;
        let b = self + other.b;
        Colour { r, g, b }
    }
}

impl ops::Sub<f64> for Colour {
    type Output = Self;
    fn sub(self, other: f64) -> Self::Output {
        let r = self.r - other;
        let g = self.g - other;
        let b = self.b - other;
        Self { r, g, b }
    }
}

impl ops::Sub<Colour> for f64 {
    type Output = Colour;
    fn sub(self, other: Colour) -> Self::Output {
        let r = self - other.r;
        let g = self - other.g;
        let b = self - other.b;
        Colour { r, g, b }
    }
}

impl ops::Mul<f64> for Colour {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        let r = self.r * other;
        let g = self.g * other;
        let b = self.b * other;
        Self { r, g, b }
    }
}

impl ops::Mul<Colour> for f64 {
    type Output = Colour;
    fn mul(self, other: Colour) -> Self::Output {
        let r = self * other.r;
        let g = self * other.g;
        let b = self * other.b;
        Colour { r, g, b }
    }
}

impl ops::Div<f64> for Colour {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        let r = self.r / other;
        let g = self.g / other;
        let b = self.b / other;
        Self { r, g, b }
    }
}

impl ops::Div<Colour> for f64 {
    type Output = Colour;
    fn div(self, other: Colour) -> Self::Output {
        let r = self / other.r;
        let g = self / other.g;
        let b = self / other.b;
        Colour { r, g, b }
    }
}

// Assignment operators.

impl ops::AddAssign for Colour {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl ops::SubAssign for Colour {
    fn sub_assign(&mut self, other: Self) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
    }
}

impl ops::MulAssign for Colour {
    fn mul_assign(&mut self, other: Self) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}

impl ops::DivAssign for Colour {
    fn div_assign(&mut self, other: Self) {
        self.r /= other.r;
        self.g /= other.g;
        self.b /= other.b;
    }
}

impl ops::AddAssign<f64> for Colour {
    fn add_assign(&mut self, other: f64) {
        self.r += other;
        self.g += other;
        self.b += other;
    }
}

impl ops::SubAssign<f64> for Colour {
    fn sub_assign(&mut self, other: f64) {
        self.r -= other;
        self.g -= other;
        self.b -= other;
    }
}

impl ops::MulAssign<f64> for Colour {
    fn mul_assign(&mut self, other: f64) {
        self.r *= other;
        self.g *= other;
        self.b *= other;
    }
}

impl ops::DivAssign<f64> for Colour {
    fn div_assign(&mut self, other: f64) {
        self.r /= other;
        self.g /= other;
        self.b /= other;
    }
}

// Indirect operators.

impl ops::Neg for &Colour {
    type Output = <Colour as ops::Neg>::Output;
    fn neg(self) -> Self::Output {
        -*self
    }
}

impl ops::Add for &Colour {
    type Output = <Colour as ops::Add>::Output;
    fn add(self, other: Self) -> Self::Output {
        *self + *other
    }
}

impl ops::Add<&Colour> for Colour {
    type Output = <Colour as ops::Add>::Output;
    fn add(self, other: &Colour) -> Self::Output {
        self + *other
    }
}

impl ops::Add<Colour> for &Colour {
    type Output = <Colour as ops::Add>::Output;
    fn add(self, other: Colour) -> Self::Output {
        *self + other
    }
}

impl ops::Sub for &Colour {
    type Output = <Colour as ops::Sub>::Output;
    fn sub(self, other: Self) -> Self::Output {
        *self - *other
    }
}

impl ops::Sub<&Colour> for Colour {
    type Output = <Colour as ops::Sub>::Output;
    fn sub(self, other: &Colour) -> Self::Output {
        self - *other
    }
}

impl ops::Sub<Colour> for &Colour {
    type Output = <Colour as ops::Sub>::Output;
    fn sub(self, other: Colour) -> Self::Output {
        *self - other
    }
}

impl ops::Mul for &Colour {
    type Output = <Colour as ops::Mul>::Output;
    fn mul(self, other: Self) -> Self::Output {
        *self * *other
    }
}

impl ops::Mul<&Colour> for Colour {
    type Output = <Colour as ops::Mul>::Output;
    fn mul(self, other: &Colour) -> Self::Output {
        self * *other
    }
}

impl ops::Mul<Colour> for &Colour {
    type Output = <Colour as ops::Mul>::Output;
    fn mul(self, other: Colour) -> Self::Output {
        *self * other
    }
}

impl ops::Div for &Colour {
    type Output = <Colour as ops::Div>::Output;
    fn div(self, other: Self) -> Self::Output {
        *self / *other
    }
}

impl ops::Div<&Colour> for Colour {
    type Output = <Colour as ops::Div>::Output;
    fn div(self, other: &Colour) -> Self::Output {
        self / *other
    }
}

impl ops::Div<Colour> for &Colour {
    type Output = <Colour as ops::Div>::Output;
    fn div(self, other: Colour) -> Self::Output {
        *self / other
    }
}

impl ops::Add<f64> for &Colour {
    type Output = <Colour as ops::Add<f64>>::Output;
    fn add(self, other: f64) -> Self::Output {
        *self + other
    }
}

impl ops::Add<&f64> for Colour {
    type Output = <Colour as ops::Add<f64>>::Output;
    fn add(self, other: &f64) -> Self::Output {
        self + *other
    }
}

impl ops::Add<&f64> for &Colour {
    type Output = <Colour as ops::Add<f64>>::Output;
    fn add(self, other: &f64) -> Self::Output {
        *self + *other
    }
}

impl ops::Add<Colour> for &f64 {
    type Output = <f64 as ops::Add<Colour>>::Output;
    fn add(self, other: Colour) -> Self::Output {
        *self + other
    }
}

impl ops::Add<&Colour> for f64 {
    type Output = <f64 as ops::Add<Colour>>::Output;
    fn add(self, other: &Colour) -> Self::Output {
        self + *other
    }
}

impl ops::Add<&Colour> for &f64 {
    type Output = <f64 as ops::Add<Colour>>::Output;
    fn add(self, other: &Colour) -> Self::Output {
        *self + *other
    }
}

impl ops::Sub<f64> for &Colour {
    type Output = <Colour as ops::Sub<f64>>::Output;
    fn sub(self, other: f64) -> Self::Output {
        *self - other
    }
}

impl ops::Sub<&f64> for Colour {
    type Output = <Colour as ops::Sub<f64>>::Output;
    fn sub(self, other: &f64) -> Self::Output {
        self - *other
    }
}

impl ops::Sub<&f64> for &Colour {
    type Output = <Colour as ops::Sub<f64>>::Output;
    fn sub(self, other: &f64) -> Self::Output {
        *self - *other
    }
}

impl ops::Sub<Colour> for &f64 {
    type Output = <f64 as ops::Sub<Colour>>::Output;
    fn sub(self, other: Colour) -> Self::Output {
        *self - other
    }
}

impl ops::Sub<&Colour> for f64 {
    type Output = <f64 as ops::Sub<Colour>>::Output;
    fn sub(self, other: &Colour) -> Self::Output {
        self - *other
    }
}

impl ops::Sub<&Colour> for &f64 {
    type Output = <f64 as ops::Sub<Colour>>::Output;
    fn sub(self, other: &Colour) -> Self::Output {
        *self - *other
    }
}

impl ops::Mul<f64> for &Colour {
    type Output = <Colour as ops::Mul<f64>>::Output;
    fn mul(self, other: f64) -> Self::Output {
        *self * other
    }
}

impl ops::Mul<&f64> for Colour {
    type Output = <Colour as ops::Mul<f64>>::Output;
    fn mul(self, other: &f64) -> Self::Output {
        self * *other
    }
}

impl ops::Mul<&f64> for &Colour {
    type Output = <Colour as ops::Mul<f64>>::Output;
    fn mul(self, other: &f64) -> Self::Output {
        *self * *other
    }
}

impl ops::Mul<Colour> for &f64 {
    type Output = <f64 as ops::Mul<Colour>>::Output;
    fn mul(self, other: Colour) -> Self::Output {
        *self * other
    }
}

impl ops::Mul<&Colour> for f64 {
    type Output = <f64 as ops::Mul<Colour>>::Output;
    fn mul(self, other: &Colour) -> Self::Output {
        self * *other
    }
}

impl ops::Mul<&Colour> for &f64 {
    type Output = <f64 as ops::Mul<Colour>>::Output;
    fn mul(self, other: &Colour) -> Self::Output {
        *self * *other
    }
}

impl ops::Div<f64> for &Colour {
    type Output = <Colour as ops::Div<f64>>::Output;
    fn div(self, other: f64) -> Self::Output {
        *self / other
    }
}

impl ops::Div<&f64> for Colour {
    type Output = <Colour as ops::Div<f64>>::Output;
    fn div(self, other: &f64) -> Self::Output {
        self / *other
    }
}

impl ops::Div<&f64> for &Colour {
    type Output = <Colour as ops::Div<f64>>::Output;
    fn div(self, other: &f64) -> Self::Output {
        *self / *other
    }
}

impl ops::Div<Colour> for &f64 {
    type Output = <f64 as ops::Div<Colour>>::Output;
    fn div(self, other: Colour) -> Self::Output {
        *self / other
    }
}

impl ops::Div<&Colour> for f64 {
    type Output = <f64 as ops::Div<Colour>>::Output;
    fn div(self, other: &Colour) -> Self::Output {
        self / *other
    }
}

impl ops::Div<&Colour> for &f64 {
    type Output = <f64 as ops::Div<Colour>>::Output;
    fn div(self, other: &Colour) -> Self::Output {
        *self / *other
    }
}

impl ops::AddAssign<&Colour> for Colour {
    fn add_assign(&mut self, other: &Colour) {
        *self += *other;
    }
}

impl ops::SubAssign<&Colour> for Colour {
    fn sub_assign(&mut self, other: &Colour) {
        *self -= *other;
    }
}

impl ops::MulAssign<&Colour> for Colour {
    fn mul_assign(&mut self, other: &Colour) {
        *self *= *other;
    }
}

impl ops::DivAssign<&Colour> for Colour {
    fn div_assign(&mut self, other: &Colour) {
        *self /= *other;
    }
}

impl ops::AddAssign<&f64> for Colour {
    fn add_assign(&mut self, other: &f64) {
        *self += *other;
    }
}

impl ops::SubAssign<&f64> for Colour {
    fn sub_assign(&mut self, other: &f64) {
        *self -= *other;
    }
}

impl ops::MulAssign<&f64> for Colour {
    fn mul_assign(&mut self, other: &f64) {
        *self *= *other;
    }
}

impl ops::DivAssign<&f64> for Colour {
    fn div_assign(&mut self, other: &f64) {
        *self /= *other;
    }
}
