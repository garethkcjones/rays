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
        let fp = p.apply(f64::floor);
        let dp = p - fp;

        let u = dp.x();
        let v = dp.y();
        let w = dp.z();

        let i = fp.x() as i32;
        let j = fp.y() as i32;
        let k = fp.z() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            let idi = (i + di) & 255;
            let di = di as usize;
            let x = self.perm_x[idi as usize];

            for dj in 0..2 {
                let jdj = (j + dj) & 255;
                let dj = dj as usize;
                let y = self.perm_y[jdj as usize];

                for dk in 0..2 {
                    let kdk = (k + dk) & 255;
                    let dk = dk as usize;
                    let z = self.perm_z[kdk as usize];

                    c[di][dj][dk] = self.ranfloat[x ^ y ^ z];
                }
            }
        }

        trilinear_interp(&c, u, v, w)
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

#[must_use]
fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    let mut i = 0.0;
    for c in c {
        let mut j = 0.0;
        for c in c {
            let mut k = 0.0;
            for c in c {
                accum += (i * u + (1.0 - i) * (1.0 - u))
                    * (j * v + (1.0 - j) * (1.0 - v))
                    * (k * w + (1.0 - k) * (1.0 - w))
                    * c;
                k += 1.0;
            }
            j += 1.0;
        }
        i += 1.0;
    }
    accum
}
