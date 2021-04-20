use crate::Vec3;
use rand::prelude::*;

const POINT_COUNT: usize = 1 << 8;

/**
 * Type for generating Perlin noise.
 */
#[derive(Clone, Debug)]
pub struct Perlin {
    ranfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    #[must_use]
    pub fn noise(&self, p: Vec3) -> f64 {
        const MASK: usize = POINT_COUNT - 1;

        let i = (4.0 * p.x()) as isize as usize & MASK;
        let j = (4.0 * p.y()) as isize as usize & MASK;
        let k = (4.0 * p.z()) as isize as usize & MASK;

        let ind = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        self.ranfloat[ind]
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranfloat = [0.0; POINT_COUNT];
        thread_rng().fill(&mut ranfloat);

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

fn permute(p: &mut [usize; POINT_COUNT]) {
    for i in (0..p.len()).rev() {
        let target = thread_rng().gen_range(0..=i);
        p.swap(i, target);
    }
}

fn perlin_generate_perm() -> [usize; POINT_COUNT] {
    let mut p = [0; POINT_COUNT];
    for (i, p) in p.iter_mut().enumerate() {
        *p = i;
    }
    permute(&mut p);
    p
}
