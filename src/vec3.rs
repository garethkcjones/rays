use crate::{random_f64, random_f64_in};
use std::{fmt, ops};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub fn random() -> Self {
        let x = random_f64();
        let y = random_f64();
        let z = random_f64();
        Self { x, y, z }
    }

    #[must_use]
    pub fn random_in(min: f64, max: f64) -> Self {
        let x = random_f64_in(min, max);
        let y = random_f64_in(min, max);
        let z = random_f64_in(min, max);
        Self { x, y, z }
    }

    #[must_use]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_in(-1.0, 1.0);
            if p.dot(p) < 1.0 {
                return p;
            }
        }
    }

    #[must_use]
    pub fn random_unit() -> Self {
        Vec3::random_in_unit_sphere().unit()
    }

    #[must_use]
    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_sphere = Vec3::random_in_unit_sphere();
        if in_sphere.dot(normal) > 0.0 {
            in_sphere
        } else {
            -in_sphere
        }
    }

    #[must_use]
    pub fn near_zero(self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        const EPSILON: f64 = 1e-8;
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    #[must_use]
    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }

    #[must_use]
    pub fn abs(self) -> f64 {
        self.dot(self).sqrt()
    }

    #[must_use]
    pub fn unit(self) -> Self {
        self / self.abs()
    }

    #[must_use]
    pub fn dot(self, other: Self) -> f64 {
        let Self {
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let Self {
            x: x2,
            y: y2,
            z: z2,
        } = other;
        x1 * x2 + y1 * y2 + z1 * z2
    }

    #[must_use]
    pub fn cross(self, other: Self) -> Self {
        let Self {
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let Self {
            x: x2,
            y: y2,
            z: z2,
        } = other;
        let x = y1 * z2 - y2 * z1;
        let y = x2 * z1 - x1 * z2;
        let z = x1 * y2 - x2 - y1;
        Self { x, y, z }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(v: (f64, f64, f64)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

impl From<([f64; 3])> for Vec3 {
    fn from(v: [f64; 3]) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

// Vector operators.

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let Self { x, y, z } = self;
        let x = -x;
        let y = -y;
        let z = -z;
        Self { x, y, z }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Self { x, y, z }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Self { x, y, z }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;
        Self { x, y, z }
    }
}

impl ops::Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        let x = self.x / other.x;
        let y = self.y / other.y;
        let z = self.z / other.z;
        Self { x, y, z }
    }
}

// Scalar operators.

impl ops::Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        let x = self.x + other;
        let y = self.y + other;
        let z = self.z + other;
        Self { x, y, z }
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Self::Output {
        let x = self + other.x;
        let y = self + other.y;
        let z = self + other.z;
        Vec3 { x, y, z }
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, other: f64) -> Self::Output {
        let x = self.x - other;
        let y = self.y - other;
        let z = self.z - other;
        Self { x, y, z }
    }
}

impl ops::Sub<Vec3> for f64 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Self::Output {
        let x = self - other.x;
        let y = self - other.y;
        let z = self - other.z;
        Vec3 { x, y, z }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        let x = self.x * other;
        let y = self.y * other;
        let z = self.z * other;
        Self { x, y, z }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        let x = self * other.x;
        let y = self * other.y;
        let z = self * other.z;
        Vec3 { x, y, z }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        let x = self.x / other;
        let y = self.y / other;
        let z = self.z / other;
        Self { x, y, z }
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Self::Output {
        let x = self / other.x;
        let y = self / other.y;
        let z = self / other.z;
        Vec3 { x, y, z }
    }
}

// Assignment operators.

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl ops::SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

// Indirect operators.

impl ops::Neg for &Vec3 {
    type Output = <Vec3 as ops::Neg>::Output;
    fn neg(self) -> Self::Output {
        -*self
    }
}

impl ops::Add for &Vec3 {
    type Output = <Vec3 as ops::Add>::Output;
    fn add(self, other: Self) -> Self::Output {
        *self + *other
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = <Vec3 as ops::Add>::Output;
    fn add(self, other: &Vec3) -> Self::Output {
        self + *other
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = <Vec3 as ops::Add>::Output;
    fn add(self, other: Vec3) -> Self::Output {
        *self + other
    }
}

impl ops::Sub for &Vec3 {
    type Output = <Vec3 as ops::Sub>::Output;
    fn sub(self, other: Self) -> Self::Output {
        *self - *other
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = <Vec3 as ops::Sub>::Output;
    fn sub(self, other: &Vec3) -> Self::Output {
        self - *other
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = <Vec3 as ops::Sub>::Output;
    fn sub(self, other: Vec3) -> Self::Output {
        *self - other
    }
}

impl ops::Mul for &Vec3 {
    type Output = <Vec3 as ops::Mul>::Output;
    fn mul(self, other: Self) -> Self::Output {
        *self * *other
    }
}

impl ops::Mul<&Vec3> for Vec3 {
    type Output = <Vec3 as ops::Mul>::Output;
    fn mul(self, other: &Vec3) -> Self::Output {
        self * *other
    }
}

impl ops::Mul<Vec3> for &Vec3 {
    type Output = <Vec3 as ops::Mul>::Output;
    fn mul(self, other: Vec3) -> Self::Output {
        *self * other
    }
}

impl ops::Div for &Vec3 {
    type Output = <Vec3 as ops::Div>::Output;
    fn div(self, other: Self) -> Self::Output {
        *self / *other
    }
}

impl ops::Div<&Vec3> for Vec3 {
    type Output = <Vec3 as ops::Div>::Output;
    fn div(self, other: &Vec3) -> Self::Output {
        self / *other
    }
}

impl ops::Div<Vec3> for &Vec3 {
    type Output = <Vec3 as ops::Div>::Output;
    fn div(self, other: Vec3) -> Self::Output {
        *self / other
    }
}

impl ops::Add<f64> for &Vec3 {
    type Output = <Vec3 as ops::Add<f64>>::Output;
    fn add(self, other: f64) -> Self::Output {
        *self + other
    }
}

impl ops::Add<&f64> for Vec3 {
    type Output = <Vec3 as ops::Add<f64>>::Output;
    fn add(self, other: &f64) -> Self::Output {
        self + *other
    }
}

impl ops::Add<&f64> for &Vec3 {
    type Output = <Vec3 as ops::Add<f64>>::Output;
    fn add(self, other: &f64) -> Self::Output {
        *self + *other
    }
}

impl ops::Add<Vec3> for &f64 {
    type Output = <f64 as ops::Add<Vec3>>::Output;
    fn add(self, other: Vec3) -> Self::Output {
        *self + other
    }
}

impl ops::Add<&Vec3> for f64 {
    type Output = <f64 as ops::Add<Vec3>>::Output;
    fn add(self, other: &Vec3) -> Self::Output {
        self + *other
    }
}

impl ops::Add<&Vec3> for &f64 {
    type Output = <f64 as ops::Add<Vec3>>::Output;
    fn add(self, other: &Vec3) -> Self::Output {
        *self + *other
    }
}

impl ops::Sub<f64> for &Vec3 {
    type Output = <Vec3 as ops::Sub<f64>>::Output;
    fn sub(self, other: f64) -> Self::Output {
        *self - other
    }
}

impl ops::Sub<&f64> for Vec3 {
    type Output = <Vec3 as ops::Sub<f64>>::Output;
    fn sub(self, other: &f64) -> Self::Output {
        self - *other
    }
}

impl ops::Sub<&f64> for &Vec3 {
    type Output = <Vec3 as ops::Sub<f64>>::Output;
    fn sub(self, other: &f64) -> Self::Output {
        *self - *other
    }
}

impl ops::Sub<Vec3> for &f64 {
    type Output = <f64 as ops::Sub<Vec3>>::Output;
    fn sub(self, other: Vec3) -> Self::Output {
        *self - other
    }
}

impl ops::Sub<&Vec3> for f64 {
    type Output = <f64 as ops::Sub<Vec3>>::Output;
    fn sub(self, other: &Vec3) -> Self::Output {
        self - *other
    }
}

impl ops::Sub<&Vec3> for &f64 {
    type Output = <f64 as ops::Sub<Vec3>>::Output;
    fn sub(self, other: &Vec3) -> Self::Output {
        *self - *other
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = <Vec3 as ops::Mul<f64>>::Output;
    fn mul(self, other: f64) -> Self::Output {
        *self * other
    }
}

impl ops::Mul<&f64> for Vec3 {
    type Output = <Vec3 as ops::Mul<f64>>::Output;
    fn mul(self, other: &f64) -> Self::Output {
        self * *other
    }
}

impl ops::Mul<&f64> for &Vec3 {
    type Output = <Vec3 as ops::Mul<f64>>::Output;
    fn mul(self, other: &f64) -> Self::Output {
        *self * *other
    }
}

impl ops::Mul<Vec3> for &f64 {
    type Output = <f64 as ops::Mul<Vec3>>::Output;
    fn mul(self, other: Vec3) -> Self::Output {
        *self * other
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = <f64 as ops::Mul<Vec3>>::Output;
    fn mul(self, other: &Vec3) -> Self::Output {
        self * *other
    }
}

impl ops::Mul<&Vec3> for &f64 {
    type Output = <f64 as ops::Mul<Vec3>>::Output;
    fn mul(self, other: &Vec3) -> Self::Output {
        *self * *other
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = <Vec3 as ops::Div<f64>>::Output;
    fn div(self, other: f64) -> Self::Output {
        *self / other
    }
}

impl ops::Div<&f64> for Vec3 {
    type Output = <Vec3 as ops::Div<f64>>::Output;
    fn div(self, other: &f64) -> Self::Output {
        self / *other
    }
}

impl ops::Div<&f64> for &Vec3 {
    type Output = <Vec3 as ops::Div<f64>>::Output;
    fn div(self, other: &f64) -> Self::Output {
        *self / *other
    }
}

impl ops::Div<Vec3> for &f64 {
    type Output = <f64 as ops::Div<Vec3>>::Output;
    fn div(self, other: Vec3) -> Self::Output {
        *self / other
    }
}

impl ops::Div<&Vec3> for f64 {
    type Output = <f64 as ops::Div<Vec3>>::Output;
    fn div(self, other: &Vec3) -> Self::Output {
        self / *other
    }
}

impl ops::Div<&Vec3> for &f64 {
    type Output = <f64 as ops::Div<Vec3>>::Output;
    fn div(self, other: &Vec3) -> Self::Output {
        *self / *other
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        *self += *other;
    }
}

impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, other: &Vec3) {
        *self -= *other;
    }
}

impl ops::MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, other: &Vec3) {
        *self *= *other;
    }
}

impl ops::DivAssign<&Vec3> for Vec3 {
    fn div_assign(&mut self, other: &Vec3) {
        *self /= *other;
    }
}

impl ops::AddAssign<&f64> for Vec3 {
    fn add_assign(&mut self, other: &f64) {
        *self += *other;
    }
}

impl ops::SubAssign<&f64> for Vec3 {
    fn sub_assign(&mut self, other: &f64) {
        *self -= *other;
    }
}

impl ops::MulAssign<&f64> for Vec3 {
    fn mul_assign(&mut self, other: &f64) {
        *self *= *other;
    }
}

impl ops::DivAssign<&f64> for Vec3 {
    fn div_assign(&mut self, other: &f64) {
        *self /= *other;
    }
}
