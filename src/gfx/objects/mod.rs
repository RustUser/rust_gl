use std::ffi::c_void;
use std::fmt::Debug;
use crate::{BufferType, DrawType};
use crate::gfx::bindings::buffers::*;

pub mod vertex_array_object_type;
pub mod vertex_array_object;
pub mod vertex_buffer_object;
pub mod vertex_attrib_pointer;
pub mod conversion;

pub trait Buffer: Debug {
    fn size(&self) -> isize;
    fn size_mut(&mut self) -> &mut isize;
    fn data(&self) -> *const c_void;
    fn set_data(&mut self, data: *const c_void);
    fn id(&self) -> u32;
    fn buffer_type(&self) -> &BufferType;
    fn draw_type(&self) -> &DrawType;
    fn bind(&self) {
        bind_buffer(*self.buffer_type(), self);
    }
    ///Updates the data in the buffer.
    fn buffer_data(&self) {
        self.bind();
        buffer_data_array(
            *self.buffer_type(),
            self.size(),
            self.data(),
            *self.draw_type()
        );
    }
}