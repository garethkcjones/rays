use crate::Vec3;
use rand::prelude::*;

const POINT_COUNT: usize = 1 << 8;

/**
 * Type for generating Perlin noise.
 */
#[derive(Clone, Debug)]
pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
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
        #![allow(clippy::many_single_char_names)]

        const MASK: usize = POINT_COUNT - 1;

        let Vec3(px, py, pz) = p;

        let fpx = px.floor();
        let fpy = py.floor();
        let fpz = pz.floor();

        let u = px - fpx;
        let v = py - fpy;
        let w = pz - fpz;

        let i = fpx as i32;
        let j = fpy as i32;
        let k = fpz as i32;

        let mut c = [[[Default::default(); 2]; 2]; 2];

        for di in 0..2 {
            let iterm = (i + di) as usize & MASK;
            let xterm = self.perm_x[iterm];
            let di = di as usize;

            for dj in 0..2 {
                let jterm = (j + dj) as usize & MASK;
                let yterm = self.perm_y[jterm];
                let dj = dj as usize;

                for dk in 0..2 {
                    let kterm = (k + dk) as usize & MASK;
                    let zterm = self.perm_z[kterm];
                    let dk = dk as usize;

                    let ind = xterm ^ yterm ^ zterm;
                    c[di][dj][dk] = self.ranvec[ind];
                }
            }
        }

        perlin_interp(&c, u, v, w)
    }

    #[must_use]
    pub fn turb(&self, p: Vec3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let ranvec = rand_fill();
        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();

        Self {
            ranvec,
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

fn rand_fill() -> [Vec3; POINT_COUNT] {
    let mut ranvec = [Default::default(); POINT_COUNT];
    for i in &mut ranvec {
        *i = Vec3::new_random(-1.0..1.0).unit();
    }
    ranvec
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    #![allow(clippy::many_single_char_names)]

    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;

    let mut i = 0.0;
    for c in c {
        let iterm = i * uu + (1.0 - i) * (1.0 - uu);
        let iweight = u - i;

        let mut j = 0.0;
        for c in c {
            let jterm = j * vv + (1.0 - j) * (1.0 - vv);
            let jweight = v - j;

            let mut k = 0.0;
            for c in c {
                let kterm = k * ww + (1.0 - k) * (1.0 - ww);
                let kweight = w - k;

                let weight_v = Vec3(iweight, jweight, kweight);
                accum += iterm * jterm * kterm * c.dot(weight_v);

                k += 1.0;
            }
            j += 1.0;
        }
        i += 1.0;
    }
    accum
}
