use std::error::Error;
use std::fmt::{Display, Formatter};
use vecmath::{Matrix4, Vector2, Vector4};
use crate::gfx::bindings::IV;
use crate::gfx::bindings::program::{attach_shader, create_program, disable_program, link_program, program_iv, use_program};
use crate::gfx::bindings::uniforms::{uniform_1f, uniform_1i, uniform_1ui, uniform_2fv, uniform_4fv, uniform_location, uniform_matrix4fv};
use crate::gfx::shader::Shader;
use super::shader::{fragment_shader::FragmentShader, vertex_shader::VertexShader};

static mut CURRENT_PROGRAM: Option<Program> = None;

#[derive(Debug, Clone, Copy)]
pub struct Program(u32);

#[derive(Debug, Clone)]
pub enum ProgramError {
    ProgramLinkError(String)
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramError::ProgramLinkError(link) => {
                f.write_fmt(format_args!("Link error: {}", link))
            }
        }
    }
}

impl Error for ProgramError {}

impl Program {
    pub fn new(fragment_shader: FragmentShader, vertex_shader: VertexShader) -> Result<Program, Box<dyn Error>> {
        let id = create_program();
        let program = Program(id);

        if let Err(error) = fragment_shader.compilation_status() {
            return Err(Box::new(error));
        }
        if let Err(error) = vertex_shader.compilation_status() {
            return Err(Box::new(error));
        }

        attach_shader(&program, &vertex_shader);
        attach_shader(&program, &fragment_shader);
        link_program(&program);

        let status = program.link_status();
        if !status {
            let status = program.status();
            return Err(Box::new(ProgramError::ProgramLinkError(status.to_string())));
        }
        Ok(program)
    }

    pub fn link_status(&self) -> bool {
        program_iv(self, IV::LinkStatus)
    }
    pub fn status(&self) -> String {
        crate::gfx::bindings::program::program_info_log(self)
    }
    pub fn id(&self) -> u32 {
        self.0
    }
    pub fn enable(&self) {
        use_program(self);
        unsafe { CURRENT_PROGRAM = Some(self.clone()); }
    }
    pub fn disable(&self) {
        unsafe { CURRENT_PROGRAM = None };
        disable_program()
    }

    pub fn uniform<T: ToString>(&self, uniform: T) -> i32 {
        uniform_location(self, uniform)
    }

    pub fn set_uniform_bool<T: ToString>(&self, uniform: T, value: &bool) {
        let value = match *value {
            true => 1,
            false => 0
        };

        self.set_uniform_int(uniform, &value);
    }

    pub fn set_uniform_u_byte<T: ToString>(&self, uniform: T, value: &u8) {
        let location = self.uniform(uniform);
        uniform_1ui(location, &(*value as u32));
    }

    pub fn set_uniform_int<T: ToString>(&self, uniform: T, value: &i32) {
        let location = self.uniform(uniform);
        uniform_1i(location, value);
    }

    pub fn set_uniform_float<T: ToString>(&self, uniform: T, value: &f32) {
        let location = self.uniform(uniform);
        uniform_1f(location, value);
    }

    pub fn set_uniform_vec2<T: ToString>(&self, uniform: T, value: &Vector2<f32>) {
        let location = self.uniform(uniform);
        uniform_2fv(location, 1, value);
    }

    pub fn set_uniform_vec4<T: ToString>(&self, uniform: T, value: &Vector4<f32>) {
        let location = self.uniform(uniform);
        uniform_4fv(location, 1, value);
    }

    pub fn set_uniform_mat4<T: ToString>(&self, uniform: T, value: &Matrix4<f32>) {
        let location = self.uniform(uniform);
        uniform_matrix4fv(location, 1, false, value);
    }

    pub fn current_program<F>(f: F) where F: Fn(&'static Program) {
        unsafe {
            if let Some(program) = &CURRENT_PROGRAM {
                f(program);
            }
        }
    }
}