use crate::{Material, Ray, Vec3};
use std::sync::Arc;

/**
 * Type for recording a ray hit.
 */
#[derive(Clone, Debug)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    u: f64,
    v: f64,
    material: Arc<dyn Material>,
    front_face: bool,
}

impl HitRecord {
    #[allow(clippy::many_single_char_names)]
    #[must_use]
    pub fn new(
        r: &Ray,
        p: Vec3,
        outward_normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            u,
            v,
            material,
            front_face,
        }
    }

    #[must_use]
    pub const fn p(&self) -> Vec3 {
        self.p
    }

    #[must_use]
    pub const fn normal(&self) -> Vec3 {
        self.normal
    }

    #[must_use]
    pub const fn t(&self) -> f64 {
        self.t
    }

    #[must_use]
    pub const fn u(&self) -> f64 {
        self.u
    }

    #[must_use]
    pub const fn v(&self) -> f64 {
        self.v
    }

    #[must_use]
    pub const fn front_face(&self) -> bool {
        self.front_face
    }

    #[must_use]
    pub fn material(&self) -> Arc<dyn Material> {
        Arc::clone(&self.material)
    }

    #[must_use]
    pub fn material_ref(&self) -> &dyn Material {
        self.material.as_ref()
    }
}
