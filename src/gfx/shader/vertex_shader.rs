use super::{*, super::bindings::*, super::bindings::shader::*};
use crate::utils::constructor::Constructor;

#[derive(Debug, Clone)]
pub struct VertexShader(u32);

impl<T: ToString> Constructor<T> for VertexShader {
    fn new(input: T) -> Self {
        let id = gl_create_shader(ShaderType::VertexShader);
        let vs = Self(id);
        shader_source(input, &vs);
        compile_shader(&vs);
        vs
    }
}

impl Drop for VertexShader {
    fn drop(&mut self) {
        delete_shader(self);
    }
}

impl Shader for VertexShader {
    fn id(&self) -> u32 {
        self.0
    }

    fn shader_type(&self) -> ShaderType {
        ShaderType::VertexShader
    }
}