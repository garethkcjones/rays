use super::Material;
use crate::{Colour, HitRecord, Ray, Texture, Vec3};
use std::sync::Arc;

/**
 * Type for materials emitting difuse light.
 */
#[derive(Debug)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    #[must_use]
    pub fn new(emit: impl Into<Arc<dyn Texture>>) -> Self {
        Self { emit: emit.into() }
    }

    #[must_use]
    pub fn new_material(emit: impl Into<Arc<dyn Texture>>) -> Arc<dyn Material> {
        Arc::new(Self::new(emit))
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Colour, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Colour {
        self.emit.value(u, v, p)
    }
}
