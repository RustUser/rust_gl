use std::ops::{Add, Index, IndexMut, Mul, Sub};
use vecmath::{mat4_transposed, Matrix4, Vector3};
use crate::math::linear_algebra::{IDENTITY_MAT4};
use crate::math::linear_algebra::types::Vec3;
use crate::{translation, v3};
use crate::math::linear_algebra::vector_ext::VectorExt;
use crate::math::linear_algebra::vector_wrapper::Vec3Wrapper;

#[macro_export]
macro_rules! mat4 {
    ($matrix:expr) => {
        MatrixWrapper($matrix)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MatrixWrapper(pub Matrix4<f32>);

pub struct Float(f32);

impl MatrixWrapper {
    pub fn new(matrix: Matrix4<f32>) -> MatrixWrapper {
        Self(matrix)
    }
    pub fn to_owned(self) -> Matrix4<f32> {
        self.0
    }
    pub fn translated(&self, t: Vector3<f32>) -> MatrixWrapper {
        MatrixWrapper(translation(t)) + *self
    }
    pub fn scale(&self, scale: Vector3<f32>) -> MatrixWrapper {
        let s = self.0.clone();
        let c0 = s[0] * Float(scale[0]);
        let c1 = s[1] * Float(scale[1]);
        let c2 = s[2] * Float(scale[2]);
        let c3 = s[3];
        Self([
            c0,
            c1,
            c2,
            c3
        ])
    }
    pub fn translation(t: Vector3<f32>) -> MatrixWrapper {
        MatrixWrapper(translation(t))
    }
    pub fn scale_matrix(scale: Vector3<f32>) -> MatrixWrapper {
        MatrixWrapper(IDENTITY_MAT4)
            .with_value(0, 0, scale[0])
            .with_value(1, 1, scale[1])
            .with_value(2, 2, scale[2])
    }
    pub fn with_value(mut self, x: usize, y: usize, value: f32) -> Self {
        self.0[x][y] = value;
        self
    }
    pub fn transposed(self) -> MatrixWrapper {
        Self(mat4_transposed(self.0))
    }

    pub fn look_at(
        position: Vec3,
        target: Vec3,
        up: Vec3,
    ) -> MatrixWrapper {
        let zero = 0f32;
        let one = 1f32;

        let z_axis = (v3!(position) - v3!(target)).normalized();
        let x_axis = v3!(up.normalized()).cross(&z_axis).normalized();
        let y_axis = z_axis.cross(&x_axis);

        let mut translation = IDENTITY_MAT4;
        translation[3][0] = -position[0];
        translation[3][1] = -position[1];
        translation[3][2] = -position[2];

        let mut rotation = IDENTITY_MAT4;
        rotation[0][0] = x_axis[0];
        rotation[1][0] = x_axis[1];
        rotation[2][0] = x_axis[2];

        rotation[0][1] = y_axis[0];
        rotation[1][1] = y_axis[1];
        rotation[2][1] = y_axis[2];

        rotation[0][2] = z_axis[0];
        rotation[1][2] = z_axis[1];
        rotation[2][2] = z_axis[2];
        mat4!(translation) * mat4!(rotation)
    }
    pub fn row(&self, row: usize) -> [f32; 4] {
        self.0[row]
    }
    pub fn col(&self, col: usize) -> [f32; 4] {
        let mut out = [0.0; 4];
        for row in 0..4 {
            out[row] = self.0[row][col];
        }
        out
    }

    pub fn row1(&self) -> [f32; 4] {
        self.row(0)
    }

    pub fn row2(&self) -> [f32; 4] {
        self.row(1)
    }

    pub fn row3(&self) -> [f32; 4] {
        self.row(2)
    }

    pub fn row4(&self) -> [f32; 4] {
        self.row(3)
    }
}

impl Index<usize> for MatrixWrapper {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for MatrixWrapper {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Matrix4<f32>> for MatrixWrapper {
    fn from(matrix: Matrix4<f32>) -> Self {
        MatrixWrapper::new(matrix)
    }
}

impl Add<MatrixWrapper> for MatrixWrapper {
    type Output = MatrixWrapper;

    fn add(self, rhs: MatrixWrapper) -> Self::Output {
        let mut result = [[0f32; 4]; 4];
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                result[i][j] = self[i][j] + rhs[i][j];
            }
        }
        MatrixWrapper::from(result)
    }
}

impl Sub<MatrixWrapper> for MatrixWrapper {
    type Output = MatrixWrapper;

    fn sub(self, rhs: MatrixWrapper) -> Self::Output {
        let mut result = [[0f32; 4]; 4];
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                result[i][j] = self[i][j] - rhs[i][j];
            }
        }
        MatrixWrapper::from(result)
    }
}

impl Mul<MatrixWrapper> for MatrixWrapper {
    type Output = MatrixWrapper;

    fn mul(self, rhs: MatrixWrapper) -> Self::Output {
        let multiply_matrices_cell = |first: &MatrixWrapper, second: &MatrixWrapper, row: usize, col: usize| {
            let mut cell = 0f32;
            for i in 0..second[0].len() {
                cell += first[row][i] * second[i][col];
            }
            cell
        };
        let mut result = [[0f32; 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                result[row][col] = multiply_matrices_cell(&self, &rhs, row, col);
            }
        }

        MatrixWrapper::new(result)
    }
}

impl Mul<Float> for [f32; 4] {
    type Output = [f32; 4];

    fn mul(self, rhs: Float) -> Self::Output {
        [self[0] * rhs.0, self[1] * rhs.0, self[2] * rhs.0, self[3] * rhs.0]
    }
}

impl<V: Into<Vec3>> Mul<V> for MatrixWrapper {
    type Output = Vec3;

    fn mul(self, rhs: V) -> Self::Output {
        let vec: Vec3 = rhs.into();

        let x = vec[0];
        let y = vec[1];
        let z = vec[2];
        let mut res = [0.0; 3];

        res[0] = x * self[0][0] + y * self[1][0] + z * self[2][0] + self[3][0];
        res[1] = x * self[0][1] + y * self[1][1] + z * self[2][1] + self[3][1];
        res[2] = x * self[0][2] + y * self[1][2] + z * self[2][2] + self[3][2];
        let w = x * self[0][3] + y * self[1][3] + z * self[2][3] + self[3][3];

        if w != 0.0 {
            for r in &mut res {
                *r /= w;
            }
        }

        res
    }
}