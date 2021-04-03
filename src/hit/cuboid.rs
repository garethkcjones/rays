#![allow(clippy::new_ret_no_self)]

use super::{HitRecord, Hittable, XyRect, XzRect, YzRect};
use crate::{Aabb, Material, Ray, Vector};
use std::{
    mem::{self, MaybeUninit},
    sync::Arc,
};

#[derive(Debug)]
pub struct Cuboid {
    sides: [Arc<dyn Hittable>; 6],
    box_min: Vector,
    box_max: Vector,
}

impl Cuboid {
    #[must_use]
    pub fn new(box_min: Vector, box_max: Vector, material: Arc<dyn Material>) -> Arc<dyn Hittable> {
        let mut sides: [MaybeUninit<Arc<dyn Hittable>>; 6] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let x0 = box_min.x();
        let y0 = box_min.y();
        let z0 = box_min.z();

        let x1 = box_max.x();
        let y1 = box_max.y();
        let z1 = box_max.z();

        sides[0] = MaybeUninit::new(XyRect::new(x0, x1, y0, y1, z0, Arc::clone(&material)));
        sides[1] = MaybeUninit::new(XyRect::new(x0, x1, y0, y1, z1, Arc::clone(&material)));

        sides[2] = MaybeUninit::new(XzRect::new(x0, x1, z0, z1, y0, Arc::clone(&material)));
        sides[3] = MaybeUninit::new(XzRect::new(x0, x1, z0, z1, y1, Arc::clone(&material)));

        sides[4] = MaybeUninit::new(YzRect::new(y0, y1, z0, z1, x0, Arc::clone(&material)));
        sides[5] = MaybeUninit::new(YzRect::new(y0, y1, z0, z1, x1, material));

        let sides: [Arc<dyn Hittable>; 6] = unsafe { mem::transmute(sides) };

        Arc::new(Self {
            sides,
            box_min,
            box_max,
        })
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}
