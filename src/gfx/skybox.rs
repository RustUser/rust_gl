use std::error::Error;
use crate::{BufferType, Camera, depth_mask, draw_arrays, DrawMode, frag, LocalAttribPointer, Program,vert, VertexArrayObject, VertexArrayObjectType, VertexBufferObject};
use crate::BufferDataType::Float;
use crate::DrawType::StaticDraw;
use crate::Face::Back;
use crate::gfx::bindings::buffers::bind_vertex_array;
use crate::gfx::bindings::graphics::polygon_mode;
use crate::gfx::bindings::PolygonMode;


use crate::math::linear_algebra::{mat3, mat4};

pub const VERTICES: &'static [f32] = {
    &[
        -1.0, -1.0, -1.0,
        -1.0, -1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0,
        -1.0, 1.0, 1.0,
        -1.0, 1.0, -1.0,
        1.0, -1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, 1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, -1.0,
        -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0, -1.0, 1.0
    ]
};

pub const SKYBOX_VERT: &'static str = {
    r#"
#version 330 core
layout (location = 0) in vec3 aPos;

out vec3 TexCoords;

uniform mat4 projection;
uniform mat4 view;

void main()
{
   TexCoords = aPos;
   gl_Position = projection * view * vec4(aPos, 1.0);
}
"#
};

pub const SKYBOX_FRAG: &'static str = {
    r#"
#version 330 core
out vec4 FragColor;

in vec3 TexCoords;

uniform samplerCube skybox;

void main()
{
    FragColor = texture(skybox, TexCoords);
    //FragColor = vec4(1.0, 0.0, 1.0, 1.0);
}
"#
};

#[derive(Debug)]
pub struct Skybox {
    skybox: VertexArrayObject,
    program: Program,
    layout: Option<i32>,
}

impl Skybox {
    pub fn _new(program: Program) -> Skybox {
        use crate::utils::constructor::Constructor;
        let skybox = VertexArrayObject::new(Some(VertexArrayObjectType::Arrays(36)))
            .with_buffer(VertexBufferObject::array(BufferType::ArrayBuffer, StaticDraw, &VERTICES))
            .with_local_attrib_pointer(LocalAttribPointer::new(3, Float, false))
            .build();
        Self {
            skybox,
            program,
            layout: None,
        }
    }


    pub fn new(skybox: VertexArrayObject, program: Program) -> Skybox {
        Self {
            skybox,
            program,
            layout: None,
        }
    }

    pub fn init(&mut self) {
        self.program.enable();
        self.skybox.bind();
        let id = self.program.uniform("skybox");
        self.layout = Some(
            id
        );
        self.skybox.unbind();
        self.program.disable();
    }

    pub fn skybox_program() -> Result<Program, Box<dyn Error>> {
        use crate::{FragmentShader, VertexShader};
        use crate::utils::constructor::Constructor;
        Program::new(frag!(SKYBOX_FRAG), vert!(SKYBOX_VERT))
    }

    pub fn draw(&self, camera: &Camera, wireframe: bool) {
        depth_mask(false);
        self.program.enable();

        let view = mat4(mat3(camera.view().clone()));

        self.program.set_uniform_mat4("projection", camera.last_perspective());
        self.program.set_uniform_mat4("view", &view);

        bind_vertex_array(&self.skybox);

        let texture = self.skybox.textures()[0];
        texture.prepare("skybox", &0, &self.program);

        if let Some(vao_type) = self.skybox.vao_type() {
            if let crate::VertexArrayObjectType::Arrays(tris) = vao_type {
                //draw_arrays(DrawMode::Triangles, 0, *tris);
                if wireframe {
                    polygon_mode(Back, PolygonMode::Line);
                    draw_arrays(DrawMode::Triangles, 0, *tris);
                }
            }
        }
        depth_mask(true);
    }

    pub fn skybox(&self) -> &VertexArrayObject {
        &self.skybox
    }
    pub fn skybox_mut(&mut self) -> &mut VertexArrayObject {
        &mut self.skybox
    }
    pub fn program(&self) -> Program {
        self.program
    }
    pub fn layout(&self) -> Option<i32> {
        self.layout
    }
}