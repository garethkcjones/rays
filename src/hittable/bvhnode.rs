use super::{Aabb, HitRecord, Hittable};
use crate::Ray;
use rand::prelude::*;
use std::{cmp::Ordering, mem, ops::Range, sync::Arc};

/**
 * Type for a node in a bounding volumn hierarchy tree.
 */
#[derive(Debug)]
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: Aabb,
}

impl BvhNode {
    #[must_use]
    pub fn new(objects: &[Arc<dyn Hittable>], tr: Range<f64>) -> Self {
        Self::from_range(&mut objects.to_owned(), tr)
    }

    #[must_use]
    pub fn new_hittable(objects: &[Arc<dyn Hittable>], tr: Range<f64>) -> Arc<dyn Hittable> {
        Arc::new(Self::new(objects, tr))
    }

    #[must_use]
    fn from_range(objects: &mut [Arc<dyn Hittable>], tr: Range<f64>) -> Self {
        assert!(!objects.is_empty());

        let axis = thread_rng().gen_range(0..=2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => unreachable!(),
        };

        let (left, right) = match objects.len() {
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
                let left = BvhNode::new_hittable_from_range(&mut objects[..mid], tr.clone());
                let right = BvhNode::new_hittable_from_range(&mut objects[mid..], tr.clone());
                (left, right)
            }
        };

        let box_left = left.bounding_box(tr.clone());
        let box_right = right.bounding_box(tr);
        let bounding_box = Aabb::surrounding_box(box_left, box_right);

        Self {
            left,
            right,
            bounding_box,
        }
    }

    #[must_use]
    fn new_hittable_from_range(
        objects: &mut [Arc<dyn Hittable>],
        tr: Range<f64>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self::from_range(objects, tr))
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        if self.bounding_box.hit(r, tr.clone()) {
            let hit_left = self.left.hit(r, tr.clone());
            let tr = hit_left
                .as_ref()
                .map_or(tr.clone(), |rec| tr.start..rec.t());
            let hit_right = self.right.hit(r, tr);
            hit_right.or(hit_left)
        } else {
            None
        }
    }

    fn bounding_box(&self, _tr: Range<f64>) -> Aabb {
        self.bounding_box.clone()
    }
}

#[must_use]
fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    let a = a.bounding_box(0.0..1.0).minimum().x();
    let b = b.bounding_box(0.0..1.0).minimum().x();
    a.partial_cmp(&b).expect("unexpected NaN in bounding box x")
}

#[must_use]
fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    let a = a.bounding_box(0.0..1.0).minimum().y();
    let b = b.bounding_box(0.0..1.0).minimum().y();
    a.partial_cmp(&b).expect("unexpected NaN in bounding box y")
}

#[must_use]
fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    let a = a.bounding_box(0.0..1.0).minimum().z();
    let b = b.bounding_box(0.0..1.0).minimum().z();
    a.partial_cmp(&b).expect("unexpected NaN in bounding box z")
}
