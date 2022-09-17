use std::path::Path;
use image::{DynamicImage, open};
use crate::gfx::bindings::{ColorFormat, MipMaps, TextureParamName, TextureTarget, TextureWrapping};
use crate::gfx::bindings::ColorFormat::RGB;
use crate::gfx::bindings::TextureParamName::{TextureMagFilter, TextureMinFilter, TextureWrapR, TextureWrapS, TextureWrapT};
use crate::gfx::bindings::TextureTarget::TextureCubeMap;
use crate::gfx::bindings::texturing::{active_texture, bind_texture, bind_texture_raw, gen_textures, generate_mipmap, tex_image_2d_u_bytes, tex_image_2d_u_bytes_raw_target, tex_parameteri};
use crate::{GLConsts, Program, uniform_1i};
use crate::gfx::bindings::uniforms::uniform_location;
use crate::GLConsts::{ClampToEdge, Linear};

pub mod texture_library;

#[derive(Debug, Clone, Copy)]
pub struct Wrapping {
    wrap_s: TextureWrapping,
    wrap_t: TextureWrapping,
    wrap_r: Option<TextureWrapping>,
}

impl Default for Wrapping {
    fn default() -> Self {
        Self {
            wrap_s: TextureWrapping::Repeat,
            wrap_t: TextureWrapping::Repeat,
            wrap_r: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MipMapping {
    min: MipMaps,
    mag: MipMaps,
    level: i32,
}

impl Default for MipMapping {
    fn default() -> Self {
        Self {
            min: MipMaps::LinearMipmapLinear,
            mag: MipMaps::Linear,
            level: 0,
        }
    }
}

#[macro_export]
macro_rules! texture {
    ($t:expr) => {
        Texture::from_image($t)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    id: u32,
    target: TextureTarget,
    wrapping: Wrapping,
    mip_mapping: MipMapping,
    color_format: ColorFormat,
    width: i32,
    height: i32,
}

#[derive(Debug, Clone)]
pub struct ImageBuffer {
    bytes: Vec<u8>,
    width: i32,
    height: i32,
    color_format: ColorFormat,
}

impl<'a> ImageBuffer {
    pub fn new(bytes: &'a [u8], width: i32, height: i32, color_format: ColorFormat) -> Self {
        Self { bytes: bytes.to_vec(), width, height, color_format }
    }

    pub fn from_image(image: &'a DynamicImage) -> ImageBuffer {
        let color_format = ColorFormat::RGB;
        let width = image.width() as i32;
        let height = image.height() as i32;
        Self {
            bytes: image.as_bytes().to_vec(),
            width,
            height,
            color_format,
        }
    }

    pub fn from_image_owned(image: DynamicImage) -> ImageBuffer {
        let color_format = ColorFormat::RGB;
        let width = image.width() as i32;
        let height = image.height() as i32;
        Self {
            bytes: image.as_bytes().to_vec(),
            width,
            height,
            color_format,
        }
    }

    pub fn generate(&self, texture: &Texture) {
        let target = texture.target;
        tex_image_2d_u_bytes(target, texture.mip_mapping.level, texture.color_format, self.width, self.height, self.color_format, &self.bytes[..]);
        generate_mipmap(texture.target);
    }
}

impl Texture {
    pub fn new(target: TextureTarget, wrapping: Wrapping, mip_mapping: MipMapping, color_format: ColorFormat, image_buffer: &'_ ImageBuffer) -> Self {
        let id = gen_textures(1);
        let me = Self { id, target, wrapping, mip_mapping, color_format, width: image_buffer.width, height: image_buffer.height };
        me.generate(Some(image_buffer));
        me
    }
    pub fn from_image(image: &DynamicImage) -> Result<Texture, Box<dyn std::error::Error>> {
        let target = TextureTarget::Texture2D;
        let texture_wrapping = Wrapping::default();
        let mip_mapping = MipMapping::default();
        let color_format = ColorFormat::RGB;
        let image_buffer = ImageBuffer::from_image(&image);
        Ok(Self::new(target, texture_wrapping, mip_mapping, color_format, &image_buffer))
    }

    pub fn cube_map(
        pos_x: &'_ ImageBuffer,
        neg_x: &'_ ImageBuffer,
        pos_y: &'_ ImageBuffer,
        neg_y: &'_ ImageBuffer,
        pos_z: &'_ ImageBuffer,
        neg_z: &'_ ImageBuffer,
    ) -> Texture {
        let id = gen_textures(1);
        bind_texture_raw(TextureCubeMap, &id);
        let mut width = 0;
        let mut height = 0;
        let n = [&pos_x, &neg_x, &pos_y, &neg_y, &pos_z, &neg_z];
        {
            tex_image_2d_u_bytes_raw_target(GLConsts::TextureCubeMapPositiveX as u32, 0, RGB, pos_x.width, pos_x.height, RGB, &pos_x.bytes[..]);
            tex_image_2d_u_bytes_raw_target(GLConsts::TextureCubeMapNegativeX as u32, 0, RGB, neg_x.width, neg_x.height, RGB, &neg_x.bytes[..]);

            tex_image_2d_u_bytes_raw_target(GLConsts::TextureCubeMapPositiveY as u32, 0, RGB, pos_y.width, pos_y.height, RGB, &pos_y.bytes[..]);
            tex_image_2d_u_bytes_raw_target(GLConsts::TextureCubeMapNegativeY as u32, 0, RGB, neg_y.width, neg_y.height, RGB, &neg_y.bytes[..]);

            tex_image_2d_u_bytes_raw_target(GLConsts::TextureCubeMapPositiveZ as u32, 0, RGB, pos_z.width, pos_z.height, RGB, &pos_z.bytes[..]);
            tex_image_2d_u_bytes_raw_target(GLConsts::TextureCubeMapNegativeZ as u32, 0, RGB, neg_z.width, neg_z.height, RGB, &neg_z.bytes[..]);
        }//POS_X
        tex_parameteri(TextureCubeMap, TextureMinFilter, Linear);
        tex_parameteri(TextureCubeMap, TextureMagFilter, Linear);

        tex_parameteri(TextureCubeMap, TextureWrapS, ClampToEdge);
        tex_parameteri(TextureCubeMap, TextureWrapT, ClampToEdge);
        tex_parameteri(TextureCubeMap, TextureWrapR, ClampToEdge);

        let t = Texture {
            id,
            target: TextureTarget::TextureCubeMap,
            wrapping: Wrapping {
                wrap_s: TextureWrapping::ClampToEdge,
                wrap_t: TextureWrapping::ClampToEdge,
                wrap_r: Some(TextureWrapping::ClampToEdge),
            },
            mip_mapping: MipMapping {
                min: MipMaps::Linear,
                mag: MipMaps::Linear,
                level: 0,
            },
            color_format: ColorFormat::RGB,
            width,
            height,
        };
        bind_texture_raw(TextureCubeMap, &0);
        t.generate(None);
        t
    }

    pub fn from_images(images: &[&DynamicImage]) -> Vec<Texture> {
        let mut imgs = vec![];
        for image in images {
            imgs.push(Self::from_image(image).unwrap());
        }
        imgs
    }

    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Texture, Box<dyn std::error::Error>> {
        match open(path) {
            Ok(image) => {
                Self::from_image(&image)
            }
            Err(_e) => {
                Err(Box::new(_e))
            }
        }
    }

    pub fn generate(&self, buffer: Option<&'_ ImageBuffer>) {
        self.bind();
        tex_parameteri(self.target, TextureParamName::TextureWrapS, self.wrapping.wrap_s.into());
        tex_parameteri(self.target, TextureParamName::TextureWrapT, self.wrapping.wrap_t.into());
        if let Some(t) = self.wrapping.wrap_r {
            tex_parameteri(self.target, TextureParamName::TextureWrapR, t.into());
        }
        tex_parameteri(self.target, TextureParamName::TextureMinFilter, self.mip_mapping.min.into());
        tex_parameteri(self.target, TextureParamName::TextureMagFilter, self.mip_mapping.mag.into());
        if let Some(buffer) = buffer {
            buffer.generate(self);
        }
    }

    pub fn active(&self, id: u32) {
        active_texture(id);
    }

    pub fn bind(&self) {
        bind_texture(self.target, self);
    }

    pub fn prepare(&self, name: &str, location: &i32, program: &Program) {
        active_texture(0);
        self.bind();
        uniform_1i(uniform_location(program, name), location);
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn target(&self) -> TextureTarget {
        self.target
    }
    pub fn wrapping(&self) -> &Wrapping {
        &self.wrapping
    }
    pub fn mip_mapping(&self) -> &MipMapping {
        &self.mip_mapping
    }
    pub fn color_format(&self) -> ColorFormat {
        self.color_format
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
}