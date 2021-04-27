use super::{Aabb, HitRecord, Hittable};
use crate::{Isotropic, Material, Ray, Texture, Vec3};
use rand::prelude::*;
use std::{ops::Range, sync::Arc};

/**
 * Type for an isotropic medium.
 */
#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    #[must_use]
    pub fn new(
        boundary: Arc<dyn Hittable>,
        density: f64,
        albedo: impl Into<Arc<dyn Texture>>,
    ) -> Self {
        let phase_function = Isotropic::new_material(albedo);
        let neg_inv_density = -density.recip();
        Self {
            boundary,
            phase_function,
            neg_inv_density,
        }
    }

    #[must_use]
    pub fn new_hittable(
        boundary: Arc<dyn Hittable>,
        density: f64,
        albedo: impl Into<Arc<dyn Texture>>,
    ) -> Arc<dyn Hittable> {
        Arc::new(Self::new(boundary, density, albedo))
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, tr: Range<f64>) -> Option<HitRecord> {
        #![allow(clippy::many_single_char_names)]

        // Print occasional samples when debugging. To enable, set to `true`.
        const ENABLE_DEBUG: bool = false;
        let debugging = ENABLE_DEBUG && thread_rng().gen::<f64>() < 0.00001;

        if let Some(rec1) = self.boundary.hit(r, -f64::INFINITY..f64::INFINITY) {
            if let Some(rec2) = self.boundary.hit(r, (rec1.t() + 0.0001)..f64::INFINITY) {
                if debugging {
                    eprint!("\nt_min = {:?}, t_max = {:?}\n", rec1.t(), rec2.t());
                }

                let t1 = rec1.t().max(tr.start);
                let t2 = rec2.t().min(tr.end);

                if t1 < t2 {
                    let t1 = t1.max(0.0);

                    let ray_length = r.direction().length();
                    let distance_inside_boundary = (t2 - t1) * ray_length;
                    let hit_distance = self.neg_inv_density * thread_rng().gen::<f64>().ln();

                    if hit_distance <= distance_inside_boundary {
                        let t = t1 + hit_distance / ray_length;
                        let p = r.at(t);

                        if debugging {
                            eprintln!("hit_distance = {:?}", hit_distance);
                            eprintln!("rec.t = {:?}", t);
                            eprintln!("rec.p = {:?}", p);
                        }

                        let normal = Vec3(1.0, 0.0, 0.0); // Arbitrary.
                        let u = 0.0; // Arbitrary.
                        let v = 0.0; // Arbitrary.

                        return Some(HitRecord::new(
                            r,
                            p,
                            normal,
                            t,
                            u,
                            v,
                            Arc::clone(&self.phase_function),
                        ));
                    }
                }
            }
        }

        None
    }

    fn bounding_box(&self, tr: Range<f64>) -> Aabb {
        self.boundary.bounding_box(tr)
    }
}
