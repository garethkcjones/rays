use std::ops;

/**
 * Type for representing vectors in 3-D space.
 */
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    #[must_use]
    pub const fn x(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn y(self) -> f64 {
        self.1
    }

    #[must_use]
    pub const fn z(self) -> f64 {
        self.2
    }

    #[must_use]
    pub fn cross(self, v: Self) -> Self {
        let Vec3(x1, y1, z1) = self;
        let Vec3(x2, y2, z2) = v;
        let x = y1 * z2 - z1 * y2;
        let y = z1 * x2 - x1 * z2;
        let z = x1 * y2 - y1 * x2;
        Self(x, y, z)
    }

    #[must_use]
    pub fn dot(self, v: Self) -> f64 {
        let Vec3(x, y, z) = self * v;
        x + y + z
    }

    #[must_use]
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    #[must_use]
    pub fn unit(self) -> Vec3 {
        self / self.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let Vec3(x, y, z) = self;
        let x = -x;
        let y = -y;
        let z = -z;
        Self(x, y, z)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, v: Self) -> Self::Output {
        let Vec3(x1, y1, z1) = self;
        let Vec3(x2, y2, z2) = v;
        let x = x1 + x2;
        let y = y1 + y2;
        let z = z1 + z2;
        Self(x, y, z)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, v: Self) -> Self::Output {
        let Vec3(x1, y1, z1) = self;
        let Vec3(x2, y2, z2) = v;
        let x = x1 - x2;
        let y = y1 - y2;
        let z = z1 - z2;
        Self(x, y, z)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, v: Self) -> Self::Output {
        let Vec3(x1, y1, z1) = self;
        let Vec3(x2, y2, z2) = v;
        let x = x1 * x2;
        let y = y1 * y2;
        let z = z1 * z2;
        Self(x, y, z)
    }
}

impl ops::Div for Vec3 {
    type Output = Self;
    fn div(self, v: Self) -> Self::Output {
        let Vec3(x1, y1, z1) = self;
        let Vec3(x2, y2, z2) = v;
        let x = x1 / x2;
        let y = y1 / y2;
        let z = z1 / z2;
        Self(x, y, z)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, s: f64) -> Self::Output {
        let Vec3(x, y, z) = self;
        let x = x + s;
        let y = y + s;
        let z = z + s;
        Self(x, y, z)
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, s: f64) -> Self::Output {
        let Vec3(x, y, z) = self;
        let x = x - s;
        let y = y - s;
        let z = z - s;
        Self(x, y, z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, s: f64) -> Self::Output {
        let Vec3(x, y, z) = self;
        let x = x * s;
        let y = y * s;
        let z = z * s;
        Self(x, y, z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, s: f64) -> Self::Output {
        let Vec3(x, y, z) = self;
        let x = x / s;
        let y = y / s;
        let z = z / s;
        Self(x, y, z)
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Self::Output {
        let Vec3(x, y, z) = v;
        let x = self + x;
        let y = self + y;
        let z = self + z;
        Vec3(x, y, z)
    }
}

impl ops::Sub<Vec3> for f64 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Self::Output {
        let Vec3(x, y, z) = v;
        let x = self - x;
        let y = self - y;
        let z = self - z;
        Vec3(x, y, z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        let Vec3(x, y, z) = v;
        let x = self * x;
        let y = self * y;
        let z = self * z;
        Vec3(x, y, z)
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, v: Vec3) -> Self::Output {
        let Vec3(x, y, z) = v;
        let x = self / x;
        let y = self / y;
        let z = self / z;
        Vec3(x, y, z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, v: Self) {
        *self = *self + v;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, v: Self) {
        *self = *self - v;
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, v: Self) {
        *self = *self * v;
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, v: Self) {
        *self = *self / v;
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, s: f64) {
        *self = *self + s;
    }
}

impl ops::SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, s: f64) {
        *self = *self - s;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, s: f64) {
        *self = *self * s;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, s: f64) {
        *self = *self / s;
    }
}