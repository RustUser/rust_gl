use std::ops::Index;
use vecmath::Vector3;

impl VectorExt for Vector3<f32> {
    fn normalized(&self) -> Self {
        let m = self.magnitude();
        [self[0] / m, self[1] / m, self[2] / m]
    }
}

pub trait VectorExt where Self: Index<usize, Output=f32> {
    fn magnitude(&self) -> f32 {
        (self[0].powf(2.0) + self[1].powf(2.0) + self[2].powf(2.0)).sqrt()
    }

    fn normalized(&self) -> Self;
}