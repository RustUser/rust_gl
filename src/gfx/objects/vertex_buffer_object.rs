use std::ffi::c_void;
use std::mem::size_of_val;
use crate::gfx::bindings::{BufferType, DrawType};
use crate::gfx::bindings::buffers::*;
use crate::gfx::objects::Buffer;
use crate::utils::{flatten_3};

#[derive(Debug, Clone)]
pub struct VertexBufferObject(u32, BufferType, DrawType, isize, *const c_void);

impl VertexBufferObject {
    pub fn array(buffer_type: BufferType, draw_type: DrawType, buffer: &[f32]) -> VertexBufferObject {
        let id = gen_buffers(1);
        let size = size_of_val(buffer) as isize;
        let buffer = buffer.as_ptr() as *const _;

        let vbo = VertexBufferObject(id, buffer_type, draw_type, size, buffer);
        bind_buffer(buffer_type, &vbo);
        buffer_data_array(buffer_type, size, buffer, draw_type);

        vbo
    }

    pub fn segmented_3(buffer_type: BufferType, draw_type: DrawType, buffer: &Vec<[f32; 3]>) -> VertexBufferObject {
        let buffer = flatten_3(buffer.clone());
        Self::array(buffer_type, draw_type, &buffer[..])
    }
}

impl Buffer for VertexBufferObject {
    fn size(&self) -> isize {
        self.3
    }

    fn size_mut(&mut self) -> &mut isize {
        &mut self.3
    }

    fn data(&self) -> *const c_void {
        self.4
    }

    fn set_data(&mut self, data: *const c_void) {
        self.4 = data;
    }

    fn id(&self) -> u32 {
        self.0
    }

    fn buffer_type(&self) -> &BufferType {
        &self.1
    }

    fn draw_type(&self) -> &DrawType {
        &self.2
    }
}