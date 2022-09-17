use rlua::{UserData, UserDataMethods};
use vecmath::Matrix4;
use crate::math::linear_algebra::{orthographic, perspective};
use crate::math::linear_algebra::types::Vec3;
use crate::{MatrixWrapper, Program, v3};
use crate::math::linear_algebra::vector_ext::VectorExt;

#[derive(Debug, Clone)]
pub struct Camera {
    pub(crate) fov: f32,
    pub(crate) far: f32,
    pub(crate) near: f32,
    pub(crate) aspect_ratio: f32,

    ///Also called the 'eye'.
    pub(crate) position: Vec3,
    pub(crate) rotation: Vec3,
    ///Also called the 'center'; or the 'front'.
    pub(crate) look: Vec3,
    pub(crate) up: Vec3,
    pub(crate) right: Vec3,
    pub(crate) worldUp: Vec3,


    view: Matrix4<f32>,
    last_perspective: Matrix4<f32>,
    last_orthographic: Matrix4<f32>,
}

impl UserData for Camera {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(_methods: &mut T) {
        _methods.add_method_mut("set_position", |_, me, pos: [f32; 3]| {
            me.set_position(pos);
            Ok(())
        });
    }
}

impl Camera {
    pub fn new(fov: f32, far: f32, near: f32, draw_size: [u32; 2], position: Vec3, look: Vec3, up: Vec3) -> Camera {
        let aspect_ratio = draw_size[0] as f32 / draw_size[1] as f32;
        let mut m = Self {
            fov,
            far,
            near,
            aspect_ratio,
            position,
            rotation: [0.0; 3],
            look,
            up,
            right: [0.0; 3],
            worldUp: up,
            //view: IDENTITY_MAT4,
            view: MatrixWrapper::look_at(position, look, up).0,
            last_perspective: perspective(fov, aspect_ratio, near, far),
            last_orthographic: orthographic(draw_size[0] as i32, draw_size[1] as i32, -1f32, 1f32),
        };
        m.update_view();
        m
    }

    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.update_perspective();
    }

    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.update_perspective();
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.update_perspective();
    }

    pub fn update_view(&mut self) {
        self.view = MatrixWrapper::look_at(self.position, self.look, self.worldUp).0
    }

    pub fn update_aspect_ratio(&mut self, draw_size: [u32; 2]) {
        let aspect_ratio = draw_size[0] as f32 / draw_size[1] as f32;
        self.aspect_ratio = aspect_ratio;
        self.update_perspective();
        self.update_orthographic(draw_size);
    }

    pub fn update_perspective(&mut self) {
        self.last_perspective = perspective(self.fov, self.aspect_ratio, self.near, self.far);
    }

    pub fn update_orthographic(&mut self, draw_size: [u32; 2]) {
        self.last_orthographic = orthographic(draw_size[0] as i32, draw_size[1] as i32, -1f32, 1f32);
    }

    pub fn view(&self) -> &Matrix4<f32> {
        &self.view
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }
    pub fn far(&self) -> f32 {
        self.far
    }
    pub fn near(&self) -> f32 {
        self.near
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
    pub fn last_perspective(&self) -> &Matrix4<f32> {
        &self.last_perspective
    }
    pub fn last_orthographic(&self) -> &Matrix4<f32> {
        &self.last_orthographic
    }

    pub fn position_mut(&mut self) -> &mut Vec3 {
        &mut self.position
    }

    pub fn rotation_mut(&mut self) -> &mut Vec3 {
        &mut self.rotation
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.update_view();
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn prepare_render(&self, program: &Program) {
        program.set_uniform_mat4("projection", &self.last_perspective);
        program.set_uniform_mat4("view", &self.view);
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        use crate::math::linear_algebra::vector_wrapper::Vec3Wrapper;
        let mut pitch = rotation[0];
        let mut yaw = rotation[1] - 90.0;
        let roll = rotation[2];

        let mut front = [0.0; 3];
        front[0] = yaw.to_radians().cos() * pitch.to_radians().cos();
        front[1] = pitch.to_radians().sin();
        front[2] = yaw.to_radians().sin() * pitch.to_radians().cos();
        self.look = front.normalized();

        self.right = v3!(self.look).cross(&v3!(self.worldUp)).normalized().0;
        self.up = v3!(self.right).cross(&v3!(self.look)).normalized().0;
        self.rotation = rotation;
        self.update_view();
    }
}