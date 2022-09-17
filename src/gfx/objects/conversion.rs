use obj::Obj;
use crate::{BufferDataType, BufferType, Constructor, DrawType, LocalAttribPointer, VertexArrayObject, VertexArrayObjectType, VertexBufferObject};

impl ToVAO for Obj {
    fn to_vao(&self) -> VertexArrayObject {
        let vertices = &self.vertices;
        let indices = &self.indices;
        let mut positions = vec![];

        for index in indices {
            let index = *index;
            let vertex = vertices[index as usize];
            let position = vertex.position;
            let normal = vertex.normal;
            let tc = vertex.tex_coords;
            positions.push(position[0]);
            positions.push(position[1]);
            positions.push(position[2]);

            positions.push(normal[0]);
            positions.push(normal[1]);
            positions.push(normal[2]);

            positions.push(tc[0]);
            positions.push(tc[1]);
        }

        VertexArrayObject::new(Some(VertexArrayObjectType::Arrays(indices.len() as i32)))
            .with_buffer(VertexBufferObject::array(BufferType::ArrayBuffer, DrawType::StaticDraw, &positions))
            .with_local_attrib_pointer(LocalAttribPointer::new(3, BufferDataType::Float, false))
            .with_local_attrib_pointer(LocalAttribPointer::new(3, BufferDataType::Float, false))
            .with_local_attrib_pointer(LocalAttribPointer::new(2, BufferDataType::Float, false))
            .build()
    }
}

pub trait ToVAO {
    fn to_vao(&self) -> VertexArrayObject;
}