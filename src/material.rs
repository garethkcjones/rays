use crate::{Colour, HitRecord, Ray};
use std::fmt::Debug;

pub trait Material: Debug {
    #[must_use]
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>;
}
