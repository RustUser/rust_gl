
use std::mem::{size_of};
use vecmath::Matrix4;
use crate::{BufferDataType, Constructor, draw_arrays, DrawMode, Face, LocalAttribPointer, MatrixExt, MatrixWrapper, shaded_wireframe, VertexArrayObject, VertexArrayObjectType, VertexBufferObject};
use crate::BufferType::ArrayBuffer;
use crate::DrawType::{DynamicDraw};
use crate::math::linear_algebra::types::Vec3;
use crate::utils::data_structure::DataStructure;

pub const FACE: &'static [f32] = &[
    -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0,// A
    0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0,// B
    -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0,// C

    0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0,// B
    0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0,// D
    -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0// C
];

#[derive(Debug)]
pub struct MeshBuilder {
    faces: Vec<f32>,
    vao: VertexArrayObject,
}

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        let faces = FACE.to_vec();
        let vbo = VertexBufferObject::array(ArrayBuffer, DynamicDraw, &faces);
        Self {
            faces,
            vao: VertexArrayObject::new(Some(VertexArrayObjectType::Arrays(6)))
                .with_buffer(vbo)
                .with_local_attrib_pointers(vec![
                    LocalAttribPointer::new(3, BufferDataType::Float, false),
                    LocalAttribPointer::new(3, BufferDataType::Float, false),
                    LocalAttribPointer::new(2, BufferDataType::Float, false),
                ])
                .build(),
        }
    }

    pub fn push_face(&mut self, pos_rot: [Vec3; 2]) {
        let position = pos_rot[0];
        let mut face = FACE.to_vec();
        for n in 0..face.len() / 8 {
            let i = n * 8;
            for x in 0..3 {
                let f_x = &mut face[i + x];
                *f_x += position[x];
            }
            let m = Matrix4::y_rotation(pos_rot[1][1]);
            let v = [face[i], face[i + 1], face[i + 2]];
            let mw = MatrixWrapper(m);
            let mv = mw * v;
            for t in 0..3 {
                face[i + t] = mv[t];
            }
        }
        self.push_vertices(face);
    }

    pub fn push_vertices(&mut self, vertices: Vec<f32>) {
        if let Some(vao_type) = self.vao.vao_type_mut() {
            match vao_type {
                VertexArrayObjectType::Arrays(tris) => {
                    *tris += vertices.len() as i32 / 8;
                }
                VertexArrayObjectType::ArrayStrips(_) => {}
            }
        }
        let add_size = size_of::<f32>() * vertices.len();
        self.faces.push_vec(vertices);
        let buffer = &mut self.vao.buffers_mut()[0];
        *buffer.size_mut() += add_size as isize;
        buffer.set_data(self.faces.as_ptr() as *const _);
        buffer.buffer_data();
    }

    pub fn bind(&self) {
        self.vao.bind();
    }

    pub fn with_face(mut self, position: [Vec3; 2]) -> MeshBuilder {
        self.push_face(position);
        self
    }

    pub fn with_faces(mut self, positions: Vec<[Vec3; 2]>) -> MeshBuilder {
        for position in positions {
            self.push_face(position);
        }
        self
    }

    pub fn draw(&self) {
        shaded_wireframe(Face::FrontAndBack, &[0.0, 1.0, 0.0, 1.0], || {
            if let Some(_type) = self.vao.vao_type() {
                match _type {
                    VertexArrayObjectType::Arrays(tris) => {
                        draw_arrays(DrawMode::Triangles, 0, *tris);
                    }
                    VertexArrayObjectType::ArrayStrips(_) => {}
                }
            }
        });
    }
}