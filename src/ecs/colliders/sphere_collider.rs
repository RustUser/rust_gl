use imgui_glfw_rs::imgui::Ui;
use vecmath::Matrix4;
use crate::ecs::colliders::Collider;
use crate::ecs::{Component, ComponentItems, draw_vec3, ECSResult};
use crate::ecs::lua_component::Value;
use crate::math::linear_algebra::types::Vec3;
use crate::{Camera, draw_arrays, DrawMode, Face, FragmentShader, mat4, OBJ, Program, shaded_wireframe, VertexArrayObject, VertexArrayObjectType, VertexShader};
use crate::ecs::transform::Transform;
use crate::math::linear_algebra::IDENTITY_MAT4;
use crate::scene::Scene;
use crate::utils::constructor::Constructor;
use crate::MatrixWrapper;
use crate::math::linear_algebra::matrix_ext::MatrixExt;
use crate::math::linear_algebra::vector_wrapper::Vec3Wrapper;

pub const SPHERE_OBJ: &'static str = include_str!("../../../res/models/debug/sphere/sphere.obj");
pub const SPHERE_VERT: &'static str = include_str!("../../../res/shaders/debug/debug_vert.glsl");
pub const SPHERE_FRAG: &'static str = include_str!("../../../res/shaders/debug/debug_frag.glsl");

#[derive(Debug)]
pub struct RenderBundle {
    vao: VertexArrayObject,
    program: Program,
    color: [f32; 4]
}

#[derive(Debug)]
pub struct SphereCollider {
    radius: f32,
    offset: Vec3,
    name: String,
    component_items: ComponentItems,
    show: bool,
    bundle: Option<RenderBundle>,
}

impl Default for SphereCollider {
    fn default() -> Self {
        let mut d = Self {
            radius: 1.0,
            offset: [0.0; 3],
            name: "SphereCollider".to_string(),
            component_items: ComponentItems {
                state: Default::default(),
                enabled: true,
                parent: 0,
                id: 0,
            },
            show: false,
            bundle: None
        };
        d.component_items.id = d.inc_id();
        d
    }
}

impl SphereCollider {}

impl Collider for SphereCollider {}

impl Component for SphereCollider {
    fn awake(&mut self) {
        let o = OBJ::from_raw(SPHERE_OBJ.to_string()).unwrap();
        let vao = o.objects()[0].build_vao("None").unwrap();
        let program = Program::new(FragmentShader::new(SPHERE_FRAG), VertexShader::new(SPHERE_VERT)).unwrap();
        self.bundle = Some(RenderBundle {
            vao,
            program,
            color: [15.0 / 255.0, 1.0, 80.0 / 255.0, 1.0]
        });
    }
    
    fn name(&self) -> &String {
        &self.name
    }

    fn items(&self) -> &ComponentItems {
        &self.component_items
    }

    fn items_mut(&mut self) -> &mut ComponentItems {
        &mut self.component_items
    }

    fn set(&mut self, key: &String, value: &Value) -> ECSResult {
        None
    }
    fn imgui_context(&mut self, ui: &Ui) {
        ui.group(|| {
            ui.text("Radius:");
            ui.input_float(format!("##radius_{}_{}", self.name, self.component_items.id), &mut self.radius).build();
            draw_vec3(&mut self.offset, ui, "Offset", self.component_items.id);
            if ui.checkbox("Draw", &mut self.show) {

            }
        });
    }

    fn render(&mut self, camera: &Camera) {
        if !self.show {
            return;
        }
        if let Some(rb) = &self.bundle {
            rb.program.enable();
            camera.prepare_render(&rb.program);


            let parent = self.parent().unwrap();
            let transform = parent.get_component("Transform".to_string()).unwrap().downcast_ref::<Transform>().unwrap();
            let rotation = transform.rotation;
            let rotation = mat4!(Matrix4::rotation(rotation));
            let v = (Vec3Wrapper(self.offset) + Vec3Wrapper(transform.position)).0;
            let scale = MatrixWrapper::scale_matrix((Vec3Wrapper(transform.scale) * self.radius).0);
            let t = scale * mat4!(IDENTITY_MAT4).translated(v);
            rb.program.set_uniform_mat4("model", &t.0);
            rb.vao.bind();
            shaded_wireframe(Face::FrontAndBack, &rb.color, || {
                if let Some(vao_type) = rb.vao.vao_type() {
                    match vao_type {
                        VertexArrayObjectType::Arrays(tris) => {
                            draw_arrays(DrawMode::Triangles, 0, *tris);
                        }
                        _ => {}
                    }
                }
            });

            rb.program.disable();
        }
    }
}
