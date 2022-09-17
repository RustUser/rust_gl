
use vecmath::*;
use crate::math::linear_algebra::vector_ext::VectorExt;

impl MatrixExt for Matrix4<f32> {
    fn rotate_around(pole: Vector3<f32>, angle: f32) -> Self {
        let u = pole.normalized();

        let u_x = u[0];
        let u_y = u[1];
        let u_z = u[2];

        let angle = angle.to_radians();
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        [
            [cos_theta + u_x.powf(2.0) * (1f32 - cos_theta), u_x * u_y * (1f32 - cos_theta) - u_z * sin_theta, u_x * u_z * (1f32 - cos_theta) + u_y * sin_theta, 0f32],
            [u_y * u_x * (1f32 - cos_theta) + u_z * sin_theta, cos_theta + u_y.powf(2.0) * (1f32 - cos_theta), u_y * u_z * (1f32 - cos_theta) - u_x * sin_theta, 0f32],
            [u_z * u_x * (1f32 - cos_theta) - u_y * sin_theta, u_z * u_y * (1f32 - cos_theta) + u_x * sin_theta, cos_theta + u_z.powf(2.0) * (1f32 - cos_theta), 0f32],
            [0f32, 0f32, 0f32, 1f32]
        ]
    }
}

pub trait MatrixExt {
    fn rotation(rotate: Vector3<f32>) -> Matrix4<f32> {
        let angle = rotate.magnitude();
        let normalized = rotate.normalized();
        Self::rotate_around(normalized, angle)
    }
    fn x_rotation(theta: f32) -> Matrix4<f32> {
        let cos = theta.to_radians().cos();
        let sin = theta.to_radians().sin();

        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos, 0.0, -sin],
            [0.0, sin, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }

    fn y_rotation(theta: f32) -> Matrix4<f32> {
        let cos = theta.to_radians().cos();
        let sin = theta.to_radians().sin();

        [
            [cos, 0.0, sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }

    fn z_rotation(theta: f32) -> Matrix4<f32> {
        let cos = theta.to_radians().cos();
        let sin = theta.to_radians().sin();

        [
            [cos, -sin, 0.0, 0.0],
            [sin, cos, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }

    fn rotate_around(pole: Vector3<f32>, angle: f32) -> Matrix4<f32>;
}