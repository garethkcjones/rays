use super::Material;
use crate::{Colour, HitRecord, Ray, Texture, Vector};
use std::sync::Arc;

#[derive(Debug)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    #[must_use]
    pub fn new(emit: impl Into<Arc<dyn Texture>>) -> Arc<dyn Material> {
        Arc::new(Self { emit: emit.into() })
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<(Ray, Colour)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Vector) -> Colour {
        self.emit.value(u, v, p)
    }
}
