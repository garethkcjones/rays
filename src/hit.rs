use crate::{Aabb, Material, Ray, Vector};
use std::{fmt::Debug, sync::Arc};

#[derive(Clone, Debug)]
pub struct HitRecord {
    p: Vector,
    normal: Vector,
    material: Arc<dyn Material>,
    t: f64,
    front_face: bool,
}

pub trait Hittable: Debug + Send + Sync {
    #[must_use]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    #[must_use]
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl HitRecord {
    #[must_use]
    pub fn new(r: &Ray, p: Vector, normal: Vector, material: Arc<dyn Material>, t: f64) -> Self {
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
    pub const fn p(&self) -> Vector {
        self.p
    }

    #[must_use]
    pub const fn normal(&self) -> Vector {
        self.normal
    }

    #[must_use]
    pub fn material(&self) -> &(dyn Material + 'static) {
        self.material.as_ref()
    }

    #[must_use]
    pub const fn t(&self) -> f64 {
        self.t
    }

    #[must_use]
    pub const fn front_face(&self) -> bool {
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let mut objects = self.iter();

        let mut bounding;
        match objects.next() {
            Some(object) => bounding = object.bounding_box(time0, time1),
            None => return None,
        }

        for object in objects {
            if bounding.is_none() {
                return None;
            }
            let bb = object.bounding_box(time0, time1);
            bounding = Aabb::surrounding_box(bounding, bb);
        }

        bounding
    }
}
