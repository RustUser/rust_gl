use std::collections::HashMap;
use maplit::hashmap;
use crate::{active_texture, BufferType, Camera, Constructor, DrawType, FragmentShader, LocalAttribPointer, Program, Texture, VertexArrayObject, VertexArrayObjectType, VertexBufferObject, VertexShader};
use crate::BufferDataType::Float;
use crate::gfx::render::{RenderObject, RenderProgramObject};
use crate::math::linear_algebra::types::Mat4;

const BLEND_VERTEX: &'static str = include_str!("../../res/shaders/blend_map/vertex.glsl");
const BLEND_FRAGMENT: &'static str = include_str!("../../res/shaders/blend_map/fragment.glsl");

#[derive(Debug)]
pub struct BlendMap {
    vao: VertexArrayObject,
    locations: HashMap<String, i32>,
    program: Program,
}

impl BlendMap {
    pub fn new(
        background: Texture,
        r: Texture,
        g: Texture,
        b: Texture,
        blend: Texture,
        buffer: &[f32],
        triangles: i32,
        draw_type: DrawType,
    ) -> BlendMap {
        let vao = VertexArrayObject::new(Some(VertexArrayObjectType::Arrays(triangles)))
            .with_buffer(VertexBufferObject::array(BufferType::ArrayBuffer, draw_type, buffer))
            .with_local_attrib_pointers(vec![
                LocalAttribPointer::new(3, Float, false),
                LocalAttribPointer::new(3, Float, false),
                LocalAttribPointer::new(2, Float, false),
            ])
            .with_texture(Some("backgroundTexture"), background)
            .with_texture(Some("rTexture"), r)
            .with_texture(Some("gTexture"), g)
            .with_texture(Some("bTexture"), b)
            .with_texture(Some("blendMap"), blend);
        let program = Program::new(FragmentShader::new(BLEND_FRAGMENT), VertexShader::new(BLEND_VERTEX)).unwrap();
        let locations = hashmap! {
            "backgroundTexture".to_string() => program.uniform("backgroundTexture"),
            "rTexture".to_string() => program.uniform("rTexture"),
            "gTexture".to_string() => program.uniform("gTexture"),
            "bTexture".to_string() => program.uniform("bTexture"),
            "blendMap".to_string() => program.uniform("blendMap")
        };
        Self {
            vao,
            locations,
            program,
        }
    }
}

impl RenderObject for BlendMap {
    fn vao(&self) -> &VertexArrayObject {
        &self.vao
    }

    fn draw(&self, camera: &Camera, model: &Mat4) {
        self.program().enable();
        self.vao.bind();
        camera.prepare_render(&self.program);
        self.program.set_uniform_mat4("model", model);
        for i in 0..self.vao.textures().len() {
            active_texture(i as u32);
            self.bind_texture2d(&self.vao.textures()[i]);
        }


        self.program.disable();
    }
}

impl RenderProgramObject for BlendMap {
    fn program(&self) -> &Program {
        &self.program
    }

    fn locations(&self) -> &HashMap<String, i32> {
        &self.locations
    }
}