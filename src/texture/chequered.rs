use super::{SolidColour, Texture};
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
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>, scale: Vector) -> Arc<Self> {
        Arc::new(Self { odd, even, scale })
    }

    #[must_use]
    pub fn with_colours(odd: Colour, even: Colour, scale: Vector) -> Arc<Self> {
        Self::new(SolidColour::new(odd), SolidColour::new(even), scale)
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
