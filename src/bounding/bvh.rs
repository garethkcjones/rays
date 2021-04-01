use crate::{random, Aabb, HitRecord, Hittable, Ray};
use std::{cmp::Ordering, mem, sync::Arc};

#[derive(Clone, Debug)]
pub struct BvhNode {
    bounding_box: Option<Aabb>,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl BvhNode {
    #[must_use]
    pub fn new(objects: &[Arc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        if objects.is_empty() {
            panic!("Empty object list");
        }
        Self::from_range(&mut objects.to_owned(), time0, time1)
    }

    #[must_use]
    fn from_range(objects: &mut [Arc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        let axis = random::i32_in(0, 2);
        let comparator = match axis {
            0 => box_x_cmp,
            1 => box_y_cmp,
            2 => box_z_cmp,
            _ => unreachable!(),
        };

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match objects.len() {
            0 => unreachable!(),
            1 => {
                let left = Arc::clone(objects.first().unwrap());
                let right = Arc::clone(objects.last().unwrap());
                (left, right)
            }
            2 => {
                let mut left = Arc::clone(objects.first().unwrap());
                let mut right = Arc::clone(objects.last().unwrap());
                if let Ordering::Greater = comparator(&left, &right) {
                    mem::swap(&mut left, &mut right);
                }
                (left, right)
            }
            len => {
                objects.sort_unstable_by(comparator);
                let mid = len / 2;
                let left = Arc::new(BvhNode::from_range(&mut objects[0..mid], time0, time1));
                let right = Arc::new(BvhNode::from_range(&mut objects[mid..len], time0, time1));
                (left, right)
            }
        };

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);
        let bounding_box = Aabb::surrounding_box(box_left, box_right);

        Self {
            bounding_box,
            left,
            right,
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.bounding_box {
            Some(bounding_box) => {
                if bounding_box.hit(r, t_min, t_max) {
                    let left_hit = self.left.hit(r, t_min, t_max);
                    let t_max = left_hit.as_ref().map_or(t_max, |rec| rec.t());
                    let right_hit = self.right.hit(r, t_min, t_max);
                    right_hit.or(left_hit)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bounding_box
    }
}

fn box_cmp(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let a = a.bounding_box(0.0, 0.0);
    let b = b.bounding_box(0.0, 0.0);
    match (a, b) {
        (Some(a), Some(b)) => a.minimum[axis]
            .partial_cmp(&b.minimum[axis])
            .expect("Unexpected NaN in bounding box"),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

fn box_x_cmp(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_cmp(a, b, 0)
}

fn box_y_cmp(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_cmp(a, b, 1)
}

fn box_z_cmp(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_cmp(a, b, 2)
}
