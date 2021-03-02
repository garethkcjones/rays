use crate::{Ray, Vec3};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    #[must_use]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    #[must_use]
    pub fn new(r: &Ray, p: Vec3, normal: Vec3, t: f64) -> Self {
        let front_face = r.direction.dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
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
