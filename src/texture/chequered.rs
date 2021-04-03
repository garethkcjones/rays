#![allow(clippy::new_ret_no_self)]

use super::Texture;
use crate::{Colour, Vector};
use std::sync::Arc;

#[derive(Debug)]
pub struct Chequered {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
    scale: Vector,
}

impl Chequered {
    #[must_use]
    pub fn new(
        odd: impl Into<Arc<dyn Texture>>,
        even: impl Into<Arc<dyn Texture>>,
        scale: Vector,
    ) -> Arc<dyn Texture> {
        Arc::new(Self {
            odd: odd.into(),
            even: even.into(),
            scale,
        })
    }
}

impl Texture for Chequered {
    fn value(&self, u: f64, v: f64, p: Vector) -> Colour {
        let sines = (self.scale * p).apply(f64::sin).product();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
