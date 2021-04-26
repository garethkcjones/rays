use super::{Aabb, HitRecord, Hittable, XyRect, XzRect, YzRect};
use crate::{Material, Ray, Vec3};
use std::{ops::Range, sync::Arc};

/**
 * Type for hittable cuboid blocks.
 */
#[derive(Debug)]
pub struct Block {
    box_min: Vec3,
    box_max: Vec3,
    sides: Vec<Arc<dyn Hittable>>,
}

impl Block {
    #[must_use]
    pub fn new(box_min: Vec3, box_max: Vec3, material: Arc<dyn Material>) -> Self {
        let Vec3(p0x, p0y, p0z) = box_min;
        let Vec3(p1x, p1y, p1z) = box_max;

        let sides = vec![
            XyRect::new_hittable(p0x..p1x, p0y..p1y, p1z, Arc::clone(&material)),
            XyRect::new_hittable(p0x..p1x, p0y..p1y, p0z, Arc::clone(&material)),
            XzRect::new_hittable(p0x..p1x, p0z..p1z, p1y, Arc::clone(&material)),
            XzRect::new_hittable(p0x..p1x, p0z..p1z, p0y, Arc::clone(&material)),
            YzRect::new_hittable(p0y..p1y, p0z..p1z, p1x, Arc::clone(&material)),
            YzRect::new_hittable(p0y..p1y, p0z..p1z, p0x, material),
        ];

        assert_eq!(sides.len(), 6);

        Self {
            box_min,
            box_max,
            sides,
        }
    }

    #[must_use]
    pub fn new_hittable(
        box_min: Vec3,
        box_max: Vec3,
        material: Arc<dyn Material>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self::new(box_min, box_max, material))
    }
}

impl Hittable for Block {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        self.sides.hit(r, tr)
    }

    fn bounding_box(&self, _tr: Range<f64>) -> Aabb {
        Aabb::new(self.box_min, self.box_max)
    }
}
