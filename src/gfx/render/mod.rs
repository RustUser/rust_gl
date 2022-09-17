use std::collections::HashMap;
use crate::{Camera, Program, Texture, uniform_1i, VertexArrayObject};
use crate::gfx::bindings::TextureTarget;
use crate::gfx::bindings::texturing::bind_texture;
use crate::gfx::bindings::uniforms::{uniform_3fv};
use crate::math::linear_algebra::types::{Mat4, Vec3};

pub trait RenderObject {
    fn vao(&self) -> &VertexArrayObject;
    fn draw(&self, camera: &Camera, model: &Mat4);
}

pub trait RenderProgramObject: RenderObject {
    fn program(&self) -> &Program;
    fn locations(&self) -> &HashMap<String, i32>;

    fn location<T: ToString>(&self, location: T) -> Option<&i32> {
        self.locations().get(&location.to_string())
    }

    fn load_int(&self, location: i32, value: &i32) {
        self.program().enable();
        uniform_1i(location, value);
        self.program().disable();
    }
    fn load_vector3(&self, location: i32, value: &Vec3) {
        self.program().enable();
        uniform_3fv(location, 1, value);
        self.program().disable();
    }

    fn bind_texture2d(&self, texture: &Texture) {
        bind_texture(TextureTarget::Texture2D, texture);
    }
}