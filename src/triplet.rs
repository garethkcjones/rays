use crate::random;
use std::{fmt, marker::PhantomData, ops};

pub trait Tag: Copy + Default {}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ColourTag;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VectorTag;

impl Tag for ColourTag {}
impl Tag for VectorTag {}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3<T: Tag> {
    data: [f64; 3],
    tag: PhantomData<T>,
}

pub type Colour = Vec3<ColourTag>;
pub type Vector = Vec3<VectorTag>;

impl Colour {
    #[must_use]
    pub const fn r(self) -> f64 {
        self.data[0]
    }

    #[must_use]
    pub const fn g(self) -> f64 {
        self.data[1]
    }

    #[must_use]
    pub const fn b(self) -> f64 {
        self.data[2]
    }
}

impl Vector {
    #[must_use]
    pub const fn x(self) -> f64 {
        self.data[0]
    }

    #[must_use]
    pub const fn y(self) -> f64 {
        self.data[1]
    }

    #[must_use]
    pub const fn z(self) -> f64 {
        self.data[2]
    }
}

impl<T: Tag> Vec3<T> {
    #[must_use]
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self {
            data: [a, b, c],
            tag: PhantomData,
        }
    }

    #[must_use]
    pub fn random() -> Self {
        let mut u = Self::default();
        u.data.iter_mut().for_each(|u| *u = random::f64());
        u
    }

    #[must_use]
    pub fn random_in(min: f64, max: f64) -> Self {
        let mut u = Self::default();
        u.data
            .iter_mut()
            .for_each(|u| *u = random::f64_in(min, max));
        u
    }

    #[must_use]
    pub fn sum(self) -> f64 {
        self.data.iter().sum()
    }

    #[must_use]
    pub fn product(self) -> f64 {
        self.data.iter().product()
    }

    #[must_use]
    pub fn apply<F>(self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let mut u = Self::default();
        u.data
            .iter_mut()
            .zip(self.data.iter())
            .for_each(|(u, v)| *u = f(*v));
        u
    }

    #[must_use]
    pub fn apply_with<F>(self, other: Self, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        let mut u = Self::default();
        u.data
            .iter_mut()
            .zip(self.data.iter().zip(other.data.iter()))
            .for_each(|(u, (v, w))| *u = f(*v, *w));
        u
    }
}

impl Vector {
    #[must_use]
    pub fn random_in_unit_disk() -> Self {
        loop {
            let x = random::f64_in(-1.0, 1.0);
            let y = random::f64_in(-1.0, 1.0);
            if x * x + y * y < 1.0 {
                return Self::new(x, y, 0.0);
            }
        }
    }

    #[must_use]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in(-1.0, 1.0);
            if p.dot(p) < 1.0 {
                return p;
            }
        }
    }

    #[must_use]
    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    #[must_use]
    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_sphere = Self::random_in_unit_sphere();
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
        for u in self.data.iter() {
            if *u >= EPSILON {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }

    #[must_use]
    pub fn refract(self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(normal).clamp(-1.0, 1.0);
        let r_perpendicular = etai_over_etat * (self + cos_theta * normal);
        let r_parallel = -(1.0 - r_perpendicular.dot(r_perpendicular)).abs().sqrt() * normal;
        r_perpendicular + r_parallel
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
        (self * other).sum()
    }

    #[must_use]
    pub fn cross(self, other: Self) -> Self {
        let [ux, uy, uz] = self.data;
        let [vx, vy, vz] = other.data;
        let x = uy * vz - uz * vy;
        let y = uz * vx - ux * vz;
        let z = ux * vy - uy * vx;
        Self::new(x, y, z)
    }
}

impl<T: Tag> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c] = self.data;
        write!(f, "({}, {}, {})", a, b, c)
    }
}

impl<T: Tag> From<[f64; 3]> for Vec3<T> {
    fn from(v: [f64; 3]) -> Self {
        Self {
            data: v,
            tag: PhantomData,
        }
    }
}

impl<T: Tag> From<(f64, f64, f64)> for Vec3<T> {
    fn from(v: (f64, f64, f64)) -> Self {
        let (a, b, c) = v;
        Self::new(a, b, c)
    }
}

// Indexing.

impl<T: Tag> ops::Index<usize> for Vec3<T> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Tag> ops::IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Unary operators.

impl<T: Tag> ops::Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.apply(f64::neg)
    }
}

// Vector–vector operators.

impl<T: Tag> ops::Add for Vec3<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self.apply_with(other, f64::add)
    }
}

impl<T: Tag> ops::Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self.apply_with(other, f64::sub)
    }
}

impl<T: Tag> ops::Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        self.apply_with(other, f64::mul)
    }
}

impl<T: Tag> ops::Div for Vec3<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        self.apply_with(other, f64::div)
    }
}

// Vector–scalar operators.

impl<T: Tag> ops::Add<f64> for Vec3<T> {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        self.apply(|x| x + other)
    }
}

impl<T: Tag> ops::Sub<f64> for Vec3<T> {
    type Output = Self;
    fn sub(self, other: f64) -> Self::Output {
        self.apply(|x| x - other)
    }
}

impl<T: Tag> ops::Mul<f64> for Vec3<T> {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        self.apply(|x| x * other)
    }
}

impl<T: Tag> ops::Div<f64> for Vec3<T> {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        self.apply(|x| x / other)
    }
}

// Scalar–vector operators.

impl<T: Tag> ops::Add<Vec3<T>> for f64 {
    type Output = Vec3<T>;
    fn add(self, other: Vec3<T>) -> Self::Output {
        other.apply(|x| self + x)
    }
}

impl<T: Tag> ops::Sub<Vec3<T>> for f64 {
    type Output = Vec3<T>;
    fn sub(self, other: Vec3<T>) -> Self::Output {
        other.apply(|x| self - x)
    }
}

impl<T: Tag> ops::Mul<Vec3<T>> for f64 {
    type Output = Vec3<T>;
    fn mul(self, other: Vec3<T>) -> Self::Output {
        other.apply(|x| self * x)
    }
}

impl<T: Tag> ops::Div<Vec3<T>> for f64 {
    type Output = Vec3<T>;
    fn div(self, other: Vec3<T>) -> Self::Output {
        other.apply(|x| self / x)
    }
}

// Vector assignment operators.

impl<T: Tag> ops::AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Tag> ops::SubAssign for Vec3<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T: Tag> ops::MulAssign for Vec3<T> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<T: Tag> ops::DivAssign for Vec3<T> {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

// Scalar assignment operators.

impl<T: Tag> ops::AddAssign<f64> for Vec3<T> {
    fn add_assign(&mut self, other: f64) {
        *self = *self + other;
    }
}

impl<T: Tag> ops::SubAssign<f64> for Vec3<T> {
    fn sub_assign(&mut self, other: f64) {
        *self = *self - other;
    }
}

impl<T: Tag> ops::MulAssign<f64> for Vec3<T> {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

impl<T: Tag> ops::DivAssign<f64> for Vec3<T> {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

// Indirect operators.

impl<T: Tag> ops::Neg for &Vec3<T> {
    type Output = <Vec3<T> as ops::Neg>::Output;
    fn neg(self) -> Self::Output {
        -*self
    }
}

impl<T: Tag> ops::Add for &Vec3<T> {
    type Output = <Vec3<T> as ops::Add>::Output;
    fn add(self, other: Self) -> Self::Output {
        *self + *other
    }
}

impl<T: Tag> ops::Add<&Vec3<T>> for Vec3<T> {
    type Output = <Vec3<T> as ops::Add>::Output;
    fn add(self, other: &Vec3<T>) -> Self::Output {
        self + *other
    }
}

impl<T: Tag> ops::Add<Vec3<T>> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Add>::Output;
    fn add(self, other: Vec3<T>) -> Self::Output {
        *self + other
    }
}

impl<T: Tag> ops::Sub for &Vec3<T> {
    type Output = <Vec3<T> as ops::Sub>::Output;
    fn sub(self, other: Self) -> Self::Output {
        *self - *other
    }
}

impl<T: Tag> ops::Sub<&Vec3<T>> for Vec3<T> {
    type Output = <Vec3<T> as ops::Sub>::Output;
    fn sub(self, other: &Vec3<T>) -> Self::Output {
        self - *other
    }
}

impl<T: Tag> ops::Sub<Vec3<T>> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Sub>::Output;
    fn sub(self, other: Vec3<T>) -> Self::Output {
        *self - other
    }
}

impl<T: Tag> ops::Mul for &Vec3<T> {
    type Output = <Vec3<T> as ops::Mul>::Output;
    fn mul(self, other: Self) -> Self::Output {
        *self * *other
    }
}

impl<T: Tag> ops::Mul<&Vec3<T>> for Vec3<T> {
    type Output = <Vec3<T> as ops::Mul>::Output;
    fn mul(self, other: &Vec3<T>) -> Self::Output {
        self * *other
    }
}

impl<T: Tag> ops::Mul<Vec3<T>> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Mul>::Output;
    fn mul(self, other: Vec3<T>) -> Self::Output {
        *self * other
    }
}

impl<T: Tag> ops::Div for &Vec3<T> {
    type Output = <Vec3<T> as ops::Div>::Output;
    fn div(self, other: Self) -> Self::Output {
        *self / *other
    }
}

impl<T: Tag> ops::Div<&Vec3<T>> for Vec3<T> {
    type Output = <Vec3<T> as ops::Div>::Output;
    fn div(self, other: &Vec3<T>) -> Self::Output {
        self / *other
    }
}

impl<T: Tag> ops::Div<Vec3<T>> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Div>::Output;
    fn div(self, other: Vec3<T>) -> Self::Output {
        *self / other
    }
}

impl<T: Tag> ops::Add<f64> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Add<f64>>::Output;
    fn add(self, other: f64) -> Self::Output {
        *self + other
    }
}

impl<T: Tag> ops::Add<&f64> for Vec3<T> {
    type Output = <Vec3<T> as ops::Add<f64>>::Output;
    fn add(self, other: &f64) -> Self::Output {
        self + *other
    }
}

impl<T: Tag> ops::Add<&f64> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Add<f64>>::Output;
    fn add(self, other: &f64) -> Self::Output {
        *self + *other
    }
}

impl<T: Tag> ops::Add<Vec3<T>> for &f64 {
    type Output = <f64 as ops::Add<Vec3<T>>>::Output;
    fn add(self, other: Vec3<T>) -> Self::Output {
        *self + other
    }
}

impl<T: Tag> ops::Add<&Vec3<T>> for f64 {
    type Output = <f64 as ops::Add<Vec3<T>>>::Output;
    fn add(self, other: &Vec3<T>) -> Self::Output {
        self + *other
    }
}

impl<T: Tag> ops::Add<&Vec3<T>> for &f64 {
    type Output = <f64 as ops::Add<Vec3<T>>>::Output;
    fn add(self, other: &Vec3<T>) -> Self::Output {
        *self + *other
    }
}

impl<T: Tag> ops::Sub<f64> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Sub<f64>>::Output;
    fn sub(self, other: f64) -> Self::Output {
        *self - other
    }
}

impl<T: Tag> ops::Sub<&f64> for Vec3<T> {
    type Output = <Vec3<T> as ops::Sub<f64>>::Output;
    fn sub(self, other: &f64) -> Self::Output {
        self - *other
    }
}

impl<T: Tag> ops::Sub<&f64> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Sub<f64>>::Output;
    fn sub(self, other: &f64) -> Self::Output {
        *self - *other
    }
}

impl<T: Tag> ops::Sub<Vec3<T>> for &f64 {
    type Output = <f64 as ops::Sub<Vec3<T>>>::Output;
    fn sub(self, other: Vec3<T>) -> Self::Output {
        *self - other
    }
}

impl<T: Tag> ops::Sub<&Vec3<T>> for f64 {
    type Output = <f64 as ops::Sub<Vec3<T>>>::Output;
    fn sub(self, other: &Vec3<T>) -> Self::Output {
        self - *other
    }
}

impl<T: Tag> ops::Sub<&Vec3<T>> for &f64 {
    type Output = <f64 as ops::Sub<Vec3<T>>>::Output;
    fn sub(self, other: &Vec3<T>) -> Self::Output {
        *self - *other
    }
}

impl<T: Tag> ops::Mul<f64> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Mul<f64>>::Output;
    fn mul(self, other: f64) -> Self::Output {
        *self * other
    }
}

impl<T: Tag> ops::Mul<&f64> for Vec3<T> {
    type Output = <Vec3<T> as ops::Mul<f64>>::Output;
    fn mul(self, other: &f64) -> Self::Output {
        self * *other
    }
}

impl<T: Tag> ops::Mul<&f64> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Mul<f64>>::Output;
    fn mul(self, other: &f64) -> Self::Output {
        *self * *other
    }
}

impl<T: Tag> ops::Mul<Vec3<T>> for &f64 {
    type Output = <f64 as ops::Mul<Vec3<T>>>::Output;
    fn mul(self, other: Vec3<T>) -> Self::Output {
        *self * other
    }
}

impl<T: Tag> ops::Mul<&Vec3<T>> for f64 {
    type Output = <f64 as ops::Mul<Vec3<T>>>::Output;
    fn mul(self, other: &Vec3<T>) -> Self::Output {
        self * *other
    }
}

impl<T: Tag> ops::Mul<&Vec3<T>> for &f64 {
    type Output = <f64 as ops::Mul<Vec3<T>>>::Output;
    fn mul(self, other: &Vec3<T>) -> Self::Output {
        *self * *other
    }
}

impl<T: Tag> ops::Div<f64> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Div<f64>>::Output;
    fn div(self, other: f64) -> Self::Output {
        *self / other
    }
}

impl<T: Tag> ops::Div<&f64> for Vec3<T> {
    type Output = <Vec3<T> as ops::Div<f64>>::Output;
    fn div(self, other: &f64) -> Self::Output {
        self / *other
    }
}

impl<T: Tag> ops::Div<&f64> for &Vec3<T> {
    type Output = <Vec3<T> as ops::Div<f64>>::Output;
    fn div(self, other: &f64) -> Self::Output {
        *self / *other
    }
}

impl<T: Tag> ops::Div<Vec3<T>> for &f64 {
    type Output = <f64 as ops::Div<Vec3<T>>>::Output;
    fn div(self, other: Vec3<T>) -> Self::Output {
        *self / other
    }
}

impl<T: Tag> ops::Div<&Vec3<T>> for f64 {
    type Output = <f64 as ops::Div<Vec3<T>>>::Output;
    fn div(self, other: &Vec3<T>) -> Self::Output {
        self / *other
    }
}

impl<T: Tag> ops::Div<&Vec3<T>> for &f64 {
    type Output = <f64 as ops::Div<Vec3<T>>>::Output;
    fn div(self, other: &Vec3<T>) -> Self::Output {
        *self / *other
    }
}

impl<T: Tag> ops::AddAssign<&Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, other: &Vec3<T>) {
        *self += *other;
    }
}

impl<T: Tag> ops::SubAssign<&Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, other: &Vec3<T>) {
        *self -= *other;
    }
}

impl<T: Tag> ops::MulAssign<&Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, other: &Vec3<T>) {
        *self *= *other;
    }
}

impl<T: Tag> ops::DivAssign<&Vec3<T>> for Vec3<T> {
    fn div_assign(&mut self, other: &Vec3<T>) {
        *self /= *other;
    }
}

impl<T: Tag> ops::AddAssign<&f64> for Vec3<T> {
    fn add_assign(&mut self, other: &f64) {
        *self += *other;
    }
}

impl<T: Tag> ops::SubAssign<&f64> for Vec3<T> {
    fn sub_assign(&mut self, other: &f64) {
        *self -= *other;
    }
}

impl<T: Tag> ops::MulAssign<&f64> for Vec3<T> {
    fn mul_assign(&mut self, other: &f64) {
        *self *= *other;
    }
}

impl<T: Tag> ops::DivAssign<&f64> for Vec3<T> {
    fn div_assign(&mut self, other: &f64) {
        *self /= *other;
    }
}
