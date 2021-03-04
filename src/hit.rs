use crate::{Material, Ray, Vec3};
use std::{fmt::Debug, rc::Rc};

#[derive(Clone, Debug)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    material: Rc<dyn Material>,
    t: f64,
    front_face: bool,
}

pub trait Hittable: Debug {
    #[must_use]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl HitRecord {
    #[must_use]
    pub fn new(r: &Ray, p: Vec3, normal: Vec3, material: Rc<dyn Material>, t: f64) -> Self {
        let front_face = r.direction.dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        Self {
            p,
            normal,
            material,
            t,
            front_face,
        }
    }

    #[must_use]
    pub fn p(&self) -> Vec3 {
        self.p
    }

    #[must_use]
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    #[must_use]
    pub fn material(&self) -> &(dyn Material + 'static) {
        self.material.as_ref()
    }

    #[must_use]
    pub fn t(&self) -> f64 {
        self.t
    }

    #[must_use]
    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec.replace(rec);
            }
        }

        temp_rec
    }
}
