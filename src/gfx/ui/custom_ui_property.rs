use std::fmt::Debug;
use vecmath::Vector4;

#[derive(Debug)]
pub enum CustomUIProperty {
    Vec4(Vector4<f32>)
}