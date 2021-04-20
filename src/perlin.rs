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

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            let iterm = (i + di) as isize as usize & MASK;
            let xterm = self.perm_x[iterm];

            for dj in 0..2 {
                let jterm = (j + dj) as isize as usize & MASK;
                let yterm = self.perm_y[jterm];

                for dk in 0..2 {
                    let kterm = (k + dk) as isize as usize & MASK;
                    let zterm = self.perm_z[kterm];

                    let ind = xterm ^ yterm ^ zterm;
                    c[di as usize][dj as usize][dk as usize] = self.ranfloat[ind];
                }
            }
        }

        trilinear_interp(&c, u, v, w)
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

fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;

    #[allow(clippy::needless_range_loop)]
    for i in 0..2 {
        let fi = i as f64;
        let iterm = fi * u + (1.0 - fi) * (1.0 - u);
        for j in 0..2 {
            let fj = j as f64;
            let jterm = fj * v + (1.0 - fj) * (1.0 - v);
            for k in 0..2 {
                let fk = k as f64;
                let kterm = fk * w + (1.0 - fk) * (1.0 - w);
                accum += iterm * jterm * kterm * c[i][j][k];
            }
        }
    }
    accum
}
