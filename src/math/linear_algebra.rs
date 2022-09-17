use vecmath::{Matrix4, Vector3};
use crate::math::linear_algebra::types::{Mat3, Mat4};

pub mod matrix_wrapper;
pub mod matrix_ext;

pub mod vector_wrapper;
pub mod vector_ext;
pub mod types;

pub const IDENTITY_MAT4: Matrix4<f32> = [
    [1f32, 0f32, 0f32, 0f32],
    [0f32, 1f32, 0f32, 0f32],
    [0f32, 0f32, 1f32, 0f32],
    [0f32, 0f32, 0f32, 1f32]
];

pub fn translation(
    v3: Vector3<f32>
) -> Matrix4<f32> {
    let mut im4 = IDENTITY_MAT4;
    for i in 0..3 {
        im4[3][i] = v3[i];
    }
    im4
}

pub fn orthographic(
    width: i32,
    height: i32,
    far: f32,
    near: f32,
) -> Matrix4<f32> {
    let left = 0f32;
    let right = width as f32;
    let bottom = 0f32;
    let top = height as f32;
    [
        [2.0 / (right - left), 0f32, 0f32, 0f32],
        [0f32, 2f32 / (top - bottom), 0f32, 0f32],
        [0f32, 0f32, -2f32 / (far - near), 0f32],
        [-(right + left) / (right - left), -(top + bottom) / (top - bottom), -(far + near) / (far - near), 1f32]
    ]
}

pub fn perspective(
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
) -> [[f32; 4]; 4] {
    let fov = fov.to_radians();
    let zero = 0f32;
    let one = 1f32;
    let two = 2f32;
    let q = one / (fov / two).tan();
    let a = q / aspect_ratio;
    let b = (near + far) / (near - far);
    let c = (two * near * far) / (near - far);

    Matrix4::from([
        [a, zero, zero, zero],
        [zero, q, zero, zero],
        [zero, zero, b, zero - one],
        [zero, zero, c, zero]
    ])
}

pub fn mat3(mat4: Mat4) -> Mat3 {
    let mut out = [[0.0; 3]; 3];
    for y in 0..3 {
        for x in 0..3 {
            out[x][y] = mat4[x][y];
        }
    }
    out
}

pub fn mat4(mat3: Mat3) -> Mat4 {
    let mut out = [[0.0; 4]; 4];
    for y in 0..3 {
        for x in 0..3 {
            out[x][y] = mat3[x][y];
        }
    }
    out[3][3] = 1.0;
    out
}