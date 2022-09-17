use std::ops::Index;
use crate::math::color::rgb::RGB;
use crate::math::color::rgba::RGBA;

pub mod rgb;
pub mod rgba;

pub trait Color: Index<usize> {
    fn to_rgb(self) -> RGB;
    fn to_rgba(self) -> RGBA;
}