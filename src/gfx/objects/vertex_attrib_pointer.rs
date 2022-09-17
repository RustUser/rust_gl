use crate::gfx::bindings::attrib_pointer::{disable_vertex_attrib_array, enable_vertex_attrib_array};
use crate::gfx::bindings::BufferDataType;

#[derive(Debug, Clone)]
pub struct LocalAttribPointer {
    data_len: u32,
    buffer_data_type: BufferDataType,
    normalized: bool,
}

impl LocalAttribPointer {
    pub fn into_vap(self, id: u32, stride: isize) -> VertexAttribPointer {
        VertexAttribPointer::new(id, self.data_len, self.buffer_data_type, self.normalized, stride)
    }
    pub fn new(data_len: u32, buffer_data_type: BufferDataType, normalized: bool) -> Self { Self { data_len, buffer_data_type, normalized } }
    pub fn data_len(&self) -> u32 {
        self.data_len
    }
    pub fn buffer_data_type(&self) -> BufferDataType {
        self.buffer_data_type
    }
    pub fn normalized(&self) -> bool {
        self.normalized
    }
}

#[derive(Debug, Clone)]
pub struct VertexAttribPointer {
    id: u32,
    data_len: u32,
    buffer_data_type: BufferDataType,
    normalized: bool,
    ///This is the full size of the data. For example, a Vector3 = 4 * 3
    size: isize,
}

impl VertexAttribPointer {
    pub fn new(id: u32, data_len: u32, buffer_data_type: BufferDataType, normalized: bool, size: isize) -> Self { Self { id, data_len, buffer_data_type, normalized, size } }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn data_len(&self) -> u32 { self.data_len }
    pub fn buffer_data_type(&self) -> BufferDataType {
        self.buffer_data_type
    }
    pub fn normalized(&self) -> bool {
        self.normalized
    }
    pub fn size(&self) -> isize {
        self.size
    }
    pub fn enable(&self) { enable_vertex_attrib_array(self) }
    pub fn disable(&self) { disable_vertex_attrib_array(self) }
}