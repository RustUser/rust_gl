use super::{*, super::bindings::*, super::bindings::shader::*};
use crate::utils::constructor::Constructor;

#[derive(Debug, Clone)]
pub struct FragmentShader(u32);

impl<T: ToString> Constructor<T> for FragmentShader {
    fn new(input: T) -> Self {
        let id = gl_create_shader(ShaderType::FragmentShader);
        let vs = Self(id);
        shader_source(input, &vs);
        compile_shader(&vs);
        vs
    }
}

impl Drop for FragmentShader {
    fn drop(&mut self) {
        delete_shader(self);
    }
}

impl Shader for FragmentShader {
    fn id(&self) -> u32 {
        self.0
    }

    fn shader_type(&self) -> ShaderType {
        ShaderType::FragmentShader
    }
}