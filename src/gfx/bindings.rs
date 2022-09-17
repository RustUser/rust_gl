use std::ffi::CString;
use std::mem::size_of;
use gl::*;
use std::ptr::{null_mut};
use rlua::{Context, MetaMethod, UserData, UserDataMethods};
use crate::gfx::shader::Shader;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum BufferDataType {
    Float = gl::FLOAT
}

impl BufferDataType {
    pub fn size(&self) -> usize {
        match self {
            BufferDataType::Float => size_of::<f32>()
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DrawType {
    StreamDraw = gl::STREAM_DRAW,
    StaticDraw = gl::STATIC_DRAW,
    DynamicDraw = gl::DYNAMIC_DRAW,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum BufferType {
    ArrayBuffer = gl::ARRAY_BUFFER
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ColorFlags {
    ColorBufferBit = GLConsts::ColorBufferBit as u32,
    DepthBufferBit = GLConsts::DepthBufferBit as u32,
}

impl std::ops::BitOr for ColorFlags {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER,
    FragmentShader = gl::FRAGMENT_SHADER,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum IV {
    CompileStatus = gl::COMPILE_STATUS,

    LinkStatus = gl::LINK_STATUS,

    InfoLogLength = gl::INFO_LOG_LENGTH,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DrawMode {
    Triangles = gl::TRIANGLES,
    TriangleStrip = gl::TRIANGLE_STRIP,
}

use enum_iterator::{all, Sequence};

#[derive(Debug, Clone, Copy, Sequence)]
#[repr(u32)]
pub enum GLConsts {
    DepthTest = gl::DEPTH_TEST,
    ColorBufferBit = gl::COLOR_BUFFER_BIT,
    DepthBufferBit = gl::DEPTH_BUFFER_BIT,

    Always = gl::ALWAYS,
    Never = gl::NEVER,
    Less = gl::LESS,
    Equal = gl::EQUAL,
    LEqual = gl::LEQUAL,
    Greater = gl::GREATER,
    NotEqual = gl::NOTEQUAL,
    GEqual = gl::GEQUAL,

    Line = gl::LINE,
    Fill = gl::FILL,

    Back = gl::BACK,
    Front = gl::FRONT,
    FrontAndBack = gl::FRONT_AND_BACK,

    CullFace = gl::CULL_FACE,

    CounterClockwise = gl::CCW,
    Clockwise = gl::CW,
    Blend = gl::BLEND,

    Texture1D = gl::TEXTURE_1D,
    Texture1DArray = gl::TEXTURE_1D_ARRAY,
    Texture2D = gl::TEXTURE_2D,
    Texture2DArray = gl::TEXTURE_2D_ARRAY,
    Texture2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
    Texture2DMultisampleArray = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
    Texture3D = gl::TEXTURE_3D,
    TextureCubeMap = gl::TEXTURE_CUBE_MAP,
    TextureCubeMapArray = gl::TEXTURE_CUBE_MAP_ARRAY,
    TextureRectangle = gl::TEXTURE_RECTANGLE,

    DepthStencilTextureMode = gl::DEPTH_STENCIL_TEXTURE_MODE,
    TextureBaseLevel = gl::TEXTURE_BASE_LEVEL,
    TextureCompareFunc = gl::TEXTURE_COMPARE_FUNC,
    TextureCompareMode = gl::TEXTURE_COMPARE_MODE,
    TextureLodBias = gl::TEXTURE_LOD_BIAS,
    TextureMinFilter = gl::TEXTURE_MIN_FILTER,
    TextureMagFilter = gl::TEXTURE_MAG_FILTER,
    TextureMinLod = gl::TEXTURE_MIN_LOD,
    TextureMaxLod = gl::TEXTURE_MAX_LOD,
    TextureMaxLevel = gl::TEXTURE_MAX_LEVEL,
    TextureSwizzleR = gl::TEXTURE_SWIZZLE_R,
    TextureSwizzleG = gl::TEXTURE_SWIZZLE_G,
    TextureSwizzleB = gl::TEXTURE_SWIZZLE_B,
    TextureSwizzleA = gl::TEXTURE_SWIZZLE_A,
    TextureWrapS = gl::TEXTURE_WRAP_S,
    TextureWrapT = gl::TEXTURE_WRAP_T,
    TextureWrapR = gl::TEXTURE_WRAP_R,

    Repeat = gl::REPEAT,
    MirroredRepeat = gl::MIRRORED_REPEAT,
    ClampToEdge = gl::CLAMP_TO_EDGE,
    ClampToBorder = gl::CLAMP_TO_BORDER,

    Nearest = gl::NEAREST,
    Linear = gl::LINEAR,

    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,

    RGB = gl::RGB,

    UnsignedByte = gl::UNSIGNED_BYTE,

    TextureCubeMapPositiveX = gl::TEXTURE_CUBE_MAP_POSITIVE_X,
    TextureCubeMapNegativeX = gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
    TextureCubeMapPositiveY = gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
    TextureCubeMapNegativeY = gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
    TextureCubeMapPositiveZ = gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
    TextureCubeMapNegativeZ = gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
}

impl GLConsts {
    pub fn load_lua<'a>(lua: &Context) {
        let globals = lua.globals();
        let cons = all::<GLConsts>();
        for con in cons {
            let name = format!("gl_{:?}", con);
            let func = lua.create_function(move |_, _: ()| {
                Ok(con)
            });
            globals.set(name, func.unwrap()).unwrap();
        }
        let cons = all::<Face>();
        for con in cons {
            let name = format!("gl_{:?}", con);
            let func = lua.create_function(move |_, _: ()| {
                Ok(con)
            });
            globals.set(name, func.unwrap()).unwrap();
        }

        let cons = all::<Ordering>();
        for con in cons {
            let name = format!("gl_{:?}", con);
            let func = lua.create_function(move |_, _: ()| {
                Ok(con)
            });
            globals.set(name, func.unwrap()).unwrap();
        }
    }
}

impl UserData for GLConsts {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(_methods: &mut T) {
        _methods.add_meta_method(MetaMethod::ToString, |_, me, _: ()| {
            Ok(format!("{:?}", me))
        });
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum MipMaps {
    NearestMipmapNearest = GLConsts::NearestMipmapNearest as u32,
    LinearMipmapNearest = GLConsts::LinearMipmapNearest as u32,
    NearestMipmapLinear = GLConsts::NearestMipmapLinear as u32,
    LinearMipmapLinear = GLConsts::LinearMipmapLinear as u32,
    Linear = GLConsts::Linear as u32,
}

impl Into<GLConsts> for MipMaps {
    fn into(self) -> GLConsts {
        match self {
            MipMaps::NearestMipmapNearest => GLConsts::NearestMipmapNearest,
            MipMaps::LinearMipmapNearest => GLConsts::LinearMipmapNearest,
            MipMaps::NearestMipmapLinear => GLConsts::NearestMipmapLinear,
            MipMaps::LinearMipmapLinear => GLConsts::LinearMipmapLinear,
            MipMaps::Linear => GLConsts::Linear
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TextureFiltering {
    Nearest = GLConsts::Nearest as u32,
    Linear = GLConsts::Linear as u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TextureWrapping {
    Repeat = GLConsts::Repeat as u32,
    MirroredRepeat = GLConsts::MirroredRepeat as u32,
    ClampToEdge = GLConsts::ClampToEdge as u32,
    ClampToBorder = GLConsts::ClampToBorder as u32,
}

impl Into<GLConsts> for TextureWrapping {
    fn into(self) -> GLConsts {
        match self {
            TextureWrapping::Repeat => GLConsts::Repeat,
            TextureWrapping::MirroredRepeat => GLConsts::MirroredRepeat,
            TextureWrapping::ClampToEdge => GLConsts::ClampToEdge,
            TextureWrapping::ClampToBorder => GLConsts::ClampToBorder
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TextureParamName {
    DepthStencilTextureMode = GLConsts::DepthStencilTextureMode as u32,
    TextureBaseLevel = GLConsts::TextureBaseLevel as u32,
    TextureCompareFunc = GLConsts::TextureCompareFunc as u32,
    TextureCompareMode = GLConsts::TextureCompareMode as u32,
    TextureLodBias = GLConsts::TextureLodBias as u32,
    TextureMinFilter = GLConsts::TextureMinFilter as u32,
    TextureMagFilter = GLConsts::TextureMagFilter as u32,
    TextureMinLod = GLConsts::TextureMinLod as u32,
    TextureMaxLod = GLConsts::TextureMaxLod as u32,
    TextureMaxLevel = GLConsts::TextureMaxLevel as u32,
    TextureSwizzleR = GLConsts::TextureSwizzleR as u32,
    TextureSwizzleG = GLConsts::TextureSwizzleG as u32,
    TextureSwizzleB = GLConsts::TextureSwizzleB as u32,
    TextureSwizzleA = GLConsts::TextureSwizzleA as u32,
    TextureWrapS = GLConsts::TextureWrapS as u32,
    TextureWrapT = GLConsts::TextureWrapT as u32,
    TextureWrapR = GLConsts::TextureWrapR as u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TextureTarget {
    Texture1D = GLConsts::Texture1D as u32,
    Texture1DArray = GLConsts::Texture1DArray as u32,
    Texture2D = GLConsts::Texture2D as u32,
    Texture2DArray = GLConsts::Texture2DArray as u32,
    Texture2DMultisample = GLConsts::Texture2DMultisample as u32,
    Texture2DMultisampleArray = GLConsts::Texture2DMultisampleArray as u32,
    Texture3D = GLConsts::Texture3D as u32,
    TextureCubeMap = GLConsts::TextureCubeMap as u32,
    TextureCubeMapArray = GLConsts::TextureCubeMapArray as u32,
    TextureRectangle = GLConsts::TextureRectangle as u32,
    DepthStencilTextureMode = GLConsts::DepthStencilTextureMode as u32,
    TextureBaseLevel = GLConsts::TextureBaseLevel as u32,
    TextureCompareFunc = GLConsts::TextureCompareFunc as u32,
    TextureCompareMode = GLConsts::TextureCompareMode as u32,
    TextureLodBias = GLConsts::TextureLodBias as u32,
    TextureMinFilter = GLConsts::TextureMinFilter as u32,
    TextureMagFilter = GLConsts::TextureMagFilter as u32,
    TextureMinLod = GLConsts::TextureMinLod as u32,
    TextureMaxLod = GLConsts::TextureMaxLod as u32,
    TextureMaxLevel = GLConsts::TextureMaxLevel as u32,
    TextureSwizzleR = GLConsts::TextureSwizzleR as u32,
    TextureSwizzleG = GLConsts::TextureSwizzleG as u32,
    TextureSwizzleB = GLConsts::TextureSwizzleB as u32,
    TextureSwizzleA = GLConsts::TextureSwizzleA as u32,
    TextureWrapS = GLConsts::TextureWrapS as u32,
    TextureWrapT = GLConsts::TextureWrapT as u32,
    TextureWrapR,
    TextureCubeMapPositiveX = GLConsts::TextureCubeMapPositiveX as u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ColorFormat {
    RGB = GLConsts::RGB as u32
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum DataType {
    UnsignedByte = GLConsts::UnsignedByte as u32
}

#[derive(Debug, Clone, Copy, Sequence)]
#[repr(u32)]
pub enum PolygonMode {
    Line = GLConsts::Line as u32,
    Fill = GLConsts::Fill as u32,
}

#[derive(Debug, Clone, Copy, Sequence)]
#[repr(u32)]
pub enum Face {
    Back = GLConsts::Back as u32,
    Front = GLConsts::Front as u32,
    FrontAndBack = GLConsts::FrontAndBack as u32,
}

#[derive(Debug, Clone, Copy, Sequence)]
#[repr(u32)]
pub enum Ordering {
    CounterClockWise = GLConsts::CounterClockwise as u32,
    ClockWise = GLConsts::Clockwise as u32,
}

impl UserData for PolygonMode {}

impl UserData for Ordering {}

impl UserData for Face {}

pub mod program {
    use std::ptr::null_mut;
    use crate::gfx::bindings::{create_whitespace_cstring_with_len, IV};
    use crate::gfx::program::Program;
    use crate::gfx::shader::Shader;

    pub fn use_program(program: &Program) {
        unsafe {
            gl::UseProgram(program.id());
        }
    }

    pub fn disable_program() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn create_program() -> u32 {
        unsafe {
            gl::CreateProgram()
        }
    }

    pub fn attach_shader<S: Shader>(program: &Program, shader: &S) {
        unsafe {
            gl::AttachShader(program.id(), shader.id());
        }
    }

    pub fn link_program(program: &Program) {
        unsafe {
            gl::LinkProgram(program.id());
        }
    }

    pub fn get_program_iv(program: &Program, iv: IV) -> i32 {
        unsafe {
            let mut success: i32 = 0;
            gl::GetProgramiv(program.id(), iv as u32, &mut success);
            success
        }
    }

    pub fn program_iv(program: &Program, iv: IV) -> bool {
        get_program_iv(program, iv) == 1
    }

    pub fn program_log_len(program: &Program) -> i32 {
        get_program_iv(program, IV::InfoLogLength)
    }

    pub fn program_info_log(program: &Program) -> String {
        unsafe {
            let log_size = program_log_len(program);
            let buffer = create_whitespace_cstring_with_len(log_size as usize);

            gl::GetProgramInfoLog(
                program.id(),
                log_size,
                null_mut(),
                buffer.as_ptr() as *mut gl::types::GLchar,
            );
            buffer.to_string_lossy().to_string()
        }
    }
}

pub mod shader {
    use std::ffi::CString;
    use std::ptr::null;
    use super::*;

    pub fn gl_create_shader(shader_type: ShaderType) -> u32 {
        unsafe {
            CreateShader(shader_type as u32)
        }
    }

    pub fn shader_source<T: ToString>(source: T, shader: &dyn Shader) {
        unsafe {
            let source = source.to_string();
            let source = &CString::new(source).unwrap();

            ShaderSource(shader.id(), 1, &source.as_ptr(), null());
        }
    }

    pub fn compile_shader(shader: &dyn Shader) {
        unsafe {
            CompileShader(shader.id());
        }
    }

    pub fn get_shader_iv<S: Shader + ?Sized>(shader: &S, iv: IV) -> i32 {
        unsafe {
            let mut success: i32 = 0;
            GetShaderiv(shader.id(), iv as u32, &mut success);
            success
        }
    }

    pub fn info_log_len<S: Shader + ?Sized>(shader: &S) -> i32 {
        get_shader_iv(shader, IV::InfoLogLength)
    }

    pub fn shader_info_log<S: Shader + ?Sized>(shader: &S) -> String {
        unsafe {
            let log_size = info_log_len(shader);
            let buffer = create_whitespace_cstring_with_len(log_size as usize);

            gl::GetShaderInfoLog(
                shader.id(),
                log_size,
                null_mut(),
                buffer.as_ptr() as *mut gl::types::GLchar,
            );
            buffer.to_string_lossy().to_string()
        }
    }

    pub fn delete_shader<S: Shader>(shader: &S) {
        unsafe {
            gl::DeleteShader(shader.id());
        }
    }
}

pub mod buffers {
    use std::ffi::c_void;
    use crate::DrawType;
    use crate::gfx::bindings::BufferType;
    use crate::gfx::objects::Buffer;
    use crate::gfx::objects::vertex_array_object::VertexArrayObject;

    pub fn gen_buffers(count: i32) -> u32 {
        unsafe {
            let mut id: u32 = 0;
            gl::GenBuffers(count, &mut id);
            id
        }
    }

    pub fn bind_buffer<B: Buffer + ?Sized>(buffer_type: BufferType, buffer: &B) {
        unsafe {
            gl::BindBuffer(buffer_type as u32, buffer.id());
        }
    }

    pub fn buffer_data_array(buffer_type: BufferType, size: isize, data: *const c_void, draw_type: DrawType) {
        unsafe {
            gl::BufferData(buffer_type as u32, size, data, draw_type as u32);
        }
        println!("glBufferData({:?}, {}, {:?}, {:?})", buffer_type, size, data, draw_type);
    }

    pub fn gen_vertex_arrays(size: i32) -> u32 {
        unsafe {
            let mut vao: u32 = 0;
            gl::GenVertexArrays(size, &mut vao);
            vao
        }
    }

    pub fn bind_vertex_array(vao: &VertexArrayObject) {
        unsafe {
            gl::BindVertexArray(vao.id());
        }
    }

    pub fn unbind_vertex_array() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub mod graphics {
    use crate::gfx::bindings::{DrawMode, Face, GLConsts, Ordering, PolygonMode};
    use crate::Program;

    pub fn clear(flags: u32) {
        unsafe {
            gl::Clear(flags)
        }
    }

    pub fn clear_color(color: [f32; 4]) {
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
        }
    }

    pub fn draw_arrays(mode: DrawMode, first: i32, count: i32) {
        unsafe {
            gl::DrawArrays(mode as u32, first, count);
        }
    }

    pub fn viewport(position: [i32; 2], size: [i32; 2]) {
        unsafe {
            gl::Viewport(position[0], position[1], size[0], size[1])
        }
    }

    pub fn enable(constant: GLConsts) {
        unsafe {
            gl::Enable(constant as u32);
        }
    }

    pub fn cull_face(face: Face) {
        unsafe {
            gl::CullFace(face as u32);
        }
    }

    pub fn front_face(ordering: Ordering) {
        unsafe {
            gl::FrontFace(ordering as u32);
        }
    }

    pub fn disable(constant: GLConsts) {
        unsafe {
            gl::Disable(constant as u32);
        }
    }

    pub fn depth_mask(depth_mask: bool) {
        unsafe {
            let mask = match depth_mask {
                true => gl::TRUE,
                false => gl::FALSE
            };
            gl::DepthMask(mask)
        }
    }

    pub fn depth_func(constant: GLConsts) {
        unsafe {
            gl::DepthFunc(constant as u32);
        }
    }

    pub fn polygon_mode(face: Face, mode: PolygonMode) {
        unsafe {
            gl::PolygonMode(face as u32, mode as u32);
        }
    }

    pub fn shaded_wireframe<F>(face: Face, wireframe_color: &[f32; 4], draw: F) where F: Fn() {
        polygon_mode(face, PolygonMode::Fill);
        Program::current_program(|program| program.set_uniform_vec4("color", &[1.0; 4]));
        //draw();

        polygon_mode(face, PolygonMode::Line);
        Program::current_program(|program| program.set_uniform_vec4("color", wireframe_color));
        draw();

        polygon_mode(face, PolygonMode::Fill);
    }
}

pub mod attrib_pointer {
    use std::ffi::c_void;
    use std::mem::transmute;
    use crate::BufferDataType;
    use crate::gfx::objects::vertex_attrib_pointer::VertexAttribPointer;

    pub fn vertex_attrib_pointer(id: u32, size: i32, data_type: BufferDataType, normalized: bool, stride: i32, offset: *const c_void) {
        unsafe {
            let normalized = match normalized {
                true => gl::TRUE,
                false => gl::FALSE
            };
            gl::VertexAttribPointer(id, size, data_type as u32, normalized, stride, offset);
            let offset: isize = transmute(offset);
            println!("VertexAttribPointer({}, {}, {:?}, {}, {}, {})", id, size, data_type, normalized, stride, offset);
        }
    }

    pub fn enable_vertex_attrib_array(attrib_pointer: &VertexAttribPointer) {
        unsafe {
            gl::EnableVertexAttribArray(attrib_pointer.id());
        }
    }

    pub fn disable_vertex_attrib_array(attrib_pointer: &VertexAttribPointer) {
        unsafe {
            gl::DisableVertexAttribArray(attrib_pointer.id());
        }
    }
}

pub mod uniforms {
    use std::ffi::CString;
    use vecmath::{Matrix4, Vector2, Vector3, Vector4};
    use crate::Program;

    pub fn uniform_location<T: ToString>(program: &Program, name: T) -> i32 {
        unsafe {
            let name = name.to_string();
            let name = CString::new(name).unwrap();

            gl::GetUniformLocation(program.id(), name.as_ptr())
        }
    }

    pub fn uniform_1ui(location: i32, value: &u32) {
        unsafe {
            gl::Uniform1ui(location, *value);
        }
    }

    pub fn uniform_1i(location: i32, value: &i32) {
        unsafe {
            gl::Uniform1i(location, *value);
        }
    }

    pub fn uniform_1f(location: i32, value: &f32) {
        unsafe {
            gl::Uniform1f(location, *value);
        }
    }

    pub fn uniform_3f(location: i32, value: Vector3<f32>) {
        unsafe {
            gl::Uniform3f(location, value[0], value[1], value[2]);
        }
    }

    pub fn uniform_4f(location: i32, value: Vector4<f32>) {
        unsafe {
            gl::Uniform4f(location, value[0], value[1], value[2], value[3]);
        }
    }

    pub fn uniform_1fv(location: i32, count: i32, value: &f32) {
        unsafe {
            gl::Uniform1fv(location, count, value as *const f32);
        }
    }

    pub fn uniform_2fv(location: i32, count: i32, value: &Vector2<f32>) {
        unsafe {
            gl::Uniform2fv(location, count, value.as_ptr());
        }
    }

    pub fn uniform_3fv(location: i32, count: i32, value: &Vector3<f32>) {
        unsafe {
            gl::Uniform3fv(location, count, value.as_ptr());
        }
    }

    pub fn uniform_4fv(location: i32, count: i32, value: &Vector4<f32>) {
        unsafe {
            gl::Uniform4fv(location, count, value.as_ptr());
        }
    }

    pub fn uniform_matrix4fv(location: i32, count: i32, transpose: bool, value: &Matrix4<f32>) {
        unsafe {
            let transpose = match transpose {
                true => gl::TRUE,
                false => gl::FALSE
            };
            gl::UniformMatrix4fv(location, count, transpose, value[0].as_ptr())
        }
    }
}

pub mod texturing {
    use crate::gfx::bindings::{ColorFormat, DataType, TextureParamName, TextureTarget};
    use crate::gfx::texture::{Texture};
    use crate::GLConsts;

    pub fn gen_textures(count: i32) -> u32 {
        let mut texture = 0;
        unsafe {
            gl::GenTextures(count, &mut texture);
        }
        texture
    }

    pub fn active_texture(texture_index: u32) {
        unsafe {
            let index = gl::TEXTURE0 + texture_index;
            gl::ActiveTexture(index);
        }
    }

    pub fn bind_texture(texture_target: TextureTarget, texture: &Texture) {
        unsafe {
            gl::BindTexture(texture_target as u32, texture.id());
        }
    }

    pub fn bind_texture_raw(texture_target: TextureTarget, texture: &u32) {
        unsafe {
            gl::BindTexture(texture_target as u32, *texture);
        }
    }

    pub fn tex_image_2d_u_bytes(texture_target: TextureTarget, level: i32, color_format: ColorFormat, width: i32, height: i32, source_color_format: ColorFormat, data: &[u8]) {
        unsafe {
            gl::TexImage2D(texture_target as u32, level, (color_format as u32) as i32, width, height, 0, source_color_format as u32, DataType::UnsignedByte as u32, data.as_ptr() as *const _);
        }
    }

    pub fn tex_image_2d_u_bytes_raw_target(texture_target: u32, level: i32, color_format: ColorFormat, width: i32, height: i32, source_color_format: ColorFormat, data: &[u8]) {
        unsafe {
            gl::TexImage2D(texture_target, level, (color_format as u32) as i32, width, height, 0, source_color_format as u32, DataType::UnsignedByte as u32, data.as_ptr() as *const _);
        }
    }


    pub fn generate_mipmap(texture_target: TextureTarget) {
        unsafe {
            gl::GenerateMipmap(texture_target as u32);
        }
    }

    pub fn tex_parameteri(target: TextureTarget, param_name: TextureParamName, param: GLConsts) {
        unsafe {
            gl::TexParameteri(target as u32, param_name as u32, (param as u32) as i32);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}