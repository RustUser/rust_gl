use std::ops::Index;
use crate::math::color::Color;
use crate::math::color::rgb::RGB;

#[derive(Debug, Clone)]
pub struct RGBA(pub [u8; 4]);

impl Index<usize> for RGBA {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Color for RGBA {
    fn to_rgb(self) -> RGB {
        RGB([self.0[0], self.0[1], self.0[2]])
    }

    fn to_rgba(self) -> RGBA {
        self
    }
}