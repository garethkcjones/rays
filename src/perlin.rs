use crate::{random_f64, random_usize_in, Vector};

const POINT_COUNT: usize = 256;

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
        let mut ranfloat = [0.0; POINT_COUNT];
        for r in &mut ranfloat {
            *r = random_f64();
        }

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

    #[must_use]
    pub fn noise(&self, p: Vector) -> f64 {
        let p = 4.0 * p;

        let x = p.x() as usize & 255;
        let y = p.y() as usize & 255;
        let z = p.z() as usize & 255;

        let i = self.perm_x[x];
        let j = self.perm_y[y];
        let k = self.perm_z[z];

        self.ranfloat[i ^ j ^ k]
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
fn perlin_generate_perm() -> [usize; POINT_COUNT] {
    let mut p = [0; POINT_COUNT];
    p.iter_mut().enumerate().for_each(|(i, p)| *p = i);
    permute(&mut p);
    p
}

fn permute(p: &mut [usize; POINT_COUNT]) {
    for i in (1..p.len()).rev() {
        let target = random_usize_in(0, i);
        p.swap(i, target);
    }
}
