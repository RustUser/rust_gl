use crate::api::LuaObject;
use crate::gfx::bindings::{BufferDataType, BufferType, ColorFlags, DrawMode, DrawType, Face, GLConsts, Ordering};
use crate::gfx::bindings::graphics::{clear, clear_color, cull_face, depth_func, depth_mask, disable, draw_arrays, enable, front_face, shaded_wireframe, viewport};
use crate::gfx::bindings::texturing::active_texture;
use crate::gfx::bindings::uniforms::{uniform_1i};
use crate::gfx::mesh_builder::MeshBuilder;
use crate::gfx::objects::vertex_array_object::VertexArrayObject;
use crate::gfx::objects::vertex_array_object_type::VertexArrayObjectType;
use crate::gfx::objects::vertex_attrib_pointer::{LocalAttribPointer};
use crate::gfx::objects::vertex_buffer_object::VertexBufferObject;
use crate::gfx::program::Program;
use crate::gfx::shader::fragment_shader::FragmentShader;
use crate::gfx::shader::vertex_shader::VertexShader;
use crate::gfx::texture::Texture;
use crate::gfx::texture::texture_library::{ TextureLibraryFlag};
use crate::gfx::ui::custom_ui_property::CustomUIProperty;
use crate::gfx::ui::{UI, UIElement};
use crate::gfx::ui::interactable::slider::Slider;
use crate::gfx::ui::layout::Layout;
use crate::gfx::ui::rectangle::{Rectangle};
use crate::input::Input;
use crate::loader::models::obj::OBJ;
use crate::math::camera::Camera;
use crate::math::linear_algebra::{translation};
use crate::math::linear_algebra::matrix_ext::MatrixExt;
use crate::math::linear_algebra::matrix_wrapper::MatrixWrapper;
use crate::utils::constructor::Constructor;

pub mod gfx;
pub mod utils;
pub mod math;
pub mod wrappers;
pub mod input;
pub mod loader;
pub mod voxel;
pub mod api;
pub mod ecs;
pub mod scene;
pub mod app;
pub mod scene_instance;
pub mod imgui_utils;

#[cfg(feature = "steam")]
pub mod steam_manager;

pub static mut WINDOW_SIZE: [u32; 2] = [512; 2];

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::gfx::consts::color::Color;
    use crate::math::linear_algebra::{mat3, mat4};

    #[test]
    fn test_color() {

        //Red in HSLA
        let hsla = hsva!([0.0, 1.0, 0.5, 1.0]);
        let rgba = hsla.to_rgba();
        let hsla = rgba.to_hsla();
        println!("{:?}", hsla);
    }

    #[test]
    fn matrix() {
        let m4 = [
            [3.0, 2.0, 1.0, 4.0],
            [5.0, 3., 2., 1.],
            [12., 13., 9., 3.],
            [12., 0., 2., 1.]
        ];
        let m3 = mat3(m4);
        let m4 = mat4(m3);
        println!("{:?}", m4);
    }
}