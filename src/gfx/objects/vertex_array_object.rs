use crate::Constructor;
use crate::gfx::bindings::attrib_pointer::*;
use crate::gfx::bindings::buffers::{bind_vertex_array, gen_vertex_arrays, unbind_vertex_array};
use crate::gfx::objects::Buffer;
use crate::gfx::objects::vertex_array_object_type::VertexArrayObjectType;
use crate::gfx::objects::vertex_attrib_pointer::{LocalAttribPointer, VertexAttribPointer};
use crate::gfx::texture::Texture;

#[derive(Debug)]
pub struct VertexArrayObject(u32, Vec<Box<dyn Buffer>>, Vec<VertexAttribPointer>, isize, Option<VertexArrayObjectType>, Vec<Texture>, Vec<Option<String>>);

impl VertexArrayObject {
    pub fn bind(&self) {
        bind_vertex_array(self);
    }
    pub fn unbind(&self) {
        unbind_vertex_array();
    }

    pub fn bind_buffer<B: Buffer + 'static>(&mut self, buffer: B) {
        self.bind();
        buffer.bind();
        buffer.buffer_data();
        self.1.push(Box::new(buffer));
        self.unbind();
    }

    pub fn vertex_attrib_pointer(&mut self, pointer: VertexAttribPointer) {
        self.bind();
        self.3 += (pointer.data_len() * pointer.buffer_data_type().size() as u32) as isize;
        pointer.enable();
        self.2.push(pointer);
        self.unbind();
    }

    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn buffers(&self) -> &Vec<Box<dyn Buffer>> {
        &self.1
    }

    pub fn buffers_mut(&mut self) -> &mut Vec<Box<dyn Buffer>> {
        &mut self.1
    }

    pub fn attrib_pointers(&self) -> &Vec<VertexAttribPointer> {
        &self.2
    }

    pub fn stride(&self) -> &isize {
        &self.3
    }

    pub fn with_buffer<B: Buffer + 'static>(mut self, buffer: B) -> VertexArrayObject {
        self.bind_buffer(buffer);
        self
    }

    pub fn with_local_attrib_pointer(self, pointer: LocalAttribPointer) -> VertexArrayObject {
        let id = self.2.len();
        let stride = *self.stride();
        self.with_vertex_attrib_pointer(pointer.into_vap(id as u32, stride))
    }

    pub fn with_local_attrib_pointers(mut self, pointers: Vec<LocalAttribPointer>) -> Self {
        for pointer in pointers {
            let id = self.2.len();
            let stride = *self.stride();
            self.vertex_attrib_pointer(pointer.into_vap(id as u32, stride));
        }
        self
    }

    pub fn with_vertex_attrib_pointer(mut self, pointer: VertexAttribPointer) -> VertexArrayObject {
        self.vertex_attrib_pointer(pointer);
        self
    }

    pub fn build(self) -> Self {
        self.bind();
        let mut pointer = 0;
        for vap in &self.2 {
            vertex_attrib_pointer(vap.id(), vap.data_len() as i32, vap.buffer_data_type(), vap.normalized(), *self.stride() as i32, (pointer) as *const _);
            vap.enable();
            pointer += vap.size();
        }
        self
    }

    pub fn vao_type(&self) -> &Option<VertexArrayObjectType> {
        &self.4
    }

    pub fn vao_type_mut(&mut self) -> &mut Option<VertexArrayObjectType> {
        &mut self.4
    }

    pub fn with_texture<T: ToString>(mut self, name: Option<T>, texture: Texture) -> VertexArrayObject {
        self.put_texture(name, texture);
        self
    }

    pub fn put_texture<T: ToString>(&mut self, name: Option<T>, texture: Texture) {
        self.5.push(texture);
        let name = match name {
            None => {
                None
            }
            Some(name) => {
                Some(name.to_string())
            }
        };
        self.6.push(name);
    }

    pub fn textures(&self) -> &Vec<Texture> {
        &self.5
    }
}

impl Constructor<Option<VertexArrayObjectType>> for VertexArrayObject {
    fn new(_type: Option<VertexArrayObjectType>) -> Self {
        Self(gen_vertex_arrays(1), vec![], vec![], 0, _type, vec![], vec![])
    }
}