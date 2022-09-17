use std::ops::Index;
use crate::math::color::Color;
use crate::math::color::rgba::RGBA;

#[derive(Debug, Clone)]
pub struct RGB(pub [u8; 3]);

impl Index<usize> for RGB {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Color for RGB {
    fn to_rgb(self) -> RGB {
        self
    }

    fn to_rgba(self) -> RGBA {
        RGBA([self.0[0], self.0[1], self.0[2], 255])
    }
}