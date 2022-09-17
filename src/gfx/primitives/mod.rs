use crate::{OBJ, VertexArrayObject};

const CUBE_OBJ: &'static str = include_str!("cube/cube.obj");
const CUBE_MAT: &'static str = include_str!("cube/cube.mtl");

pub enum Primitive {
    Cube
}

impl Primitive {
    pub fn cube() -> VertexArrayObject {
        OBJ::from_raw(CUBE_OBJ.to_string()).unwrap().objects()[0].build_vao("Material").unwrap()
    }
}