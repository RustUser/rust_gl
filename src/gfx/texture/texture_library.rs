use std::ops::BitOr;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use crate::{Texture, texture};
use enum_iterator::{all, Sequence};
use image::{load_from_memory};

static mut TEXTURE_LIBRARY: Option<Arc<Mutex<TextureLibrary>>> = None;

#[derive(Debug, Clone, Copy, Sequence)]
#[repr(u16)]
pub enum TextureLibraryFlag {
    Metal = 0x01,
    Wood = 0x02,
    Water = 0x04,
    Dirt = 0x08,
}

impl Into<usize> for TextureLibraryFlag {
    fn into(self) -> usize {
        match self {
            TextureLibraryFlag::Metal => 0,
            TextureLibraryFlag::Wood => 1,
            TextureLibraryFlag::Water => 2,
            TextureLibraryFlag::Dirt => 3
        }
    }
}

impl BitOr<u16> for TextureLibraryFlag {
    type Output = u16;

    fn bitor(self, rhs: u16) -> Self::Output {
        self as u16 | rhs
    }
}

impl BitOr<TextureLibraryFlag> for TextureLibraryFlag {
    type Output = u16;

    fn bitor(self, rhs: TextureLibraryFlag) -> Self::Output {
        self as u16 | rhs as u16
    }
}

pub struct TextureLibrary {
    library: Vec<Vec<Texture>>,
}

impl TextureLibrary {
    pub fn get_textures(texture: TextureLibraryFlag) -> Vec<Texture> {
        unsafe {
            let id: usize = texture.into();
            match &TEXTURE_LIBRARY {
                None => panic!("Texture library not initialized."),
                Some(texture_library) => {
                    match texture_library.lock() {
                        Ok(library) => {
                            library.library.get(id).unwrap().clone()
                        }
                        Err(e) => {
                            panic!("{}", e)
                        }
                    }
                }
            }
        }
    }
    pub fn is_loaded() -> bool {
        unsafe { TEXTURE_LIBRARY.is_some() }
    }
    pub unsafe fn init(flags: u16) {
        TEXTURE_LIBRARY = Some(Arc::new(Mutex::new(TextureLibrary { library: vec![] })));

        spawn(move || {
            let tex_lib = TEXTURE_LIBRARY.as_mut().unwrap();
            let libs = all::<TextureLibraryFlag>();
            let mut library = vec![];

            let mut index = 0;
            for tex_lib in libs {
                let mut lib = vec![];

                if (flags >> index & 1) != 0 {
                    match tex_lib {
                        TextureLibraryFlag::Metal => {
                            lib = vec![
                                texture!(&load_from_memory(include_bytes!("../../../res/textures/metal/Metal_006_ambientOcclusion.jpg")).unwrap()).unwrap(),
                                texture!(&load_from_memory(include_bytes!("../../../res/textures/metal/Metal_006_basecolor.jpg")).unwrap()).unwrap(),
                                texture!(&load_from_memory(include_bytes!("../../../res/textures/metal/Metal_006_height.png")).unwrap()).unwrap(),
                                texture!(&load_from_memory(include_bytes!("../../../res/textures/metal/Metal_006_metallic.jpg")).unwrap()).unwrap(),
                                texture!(&load_from_memory(include_bytes!("../../../res/textures/metal/Metal_006_normal.jpg")).unwrap()).unwrap(),
                                texture!(&load_from_memory(include_bytes!("../../../res/textures/metal/Metal_006_roughness.jpg")).unwrap()).unwrap(),
                            ];
                        }
                        TextureLibraryFlag::Wood => {
                            panic!("Wood not yet implemented.")
                        }
                        TextureLibraryFlag::Water => {
                            //panic!("Water not yet implemented.")
                        }
                        TextureLibraryFlag::Dirt => {
                            panic!("Dirt not yet implemented.")
                        }
                    }
                }

                library.push(lib);
                index += 1;
            }
            let tex_l = TextureLibrary {
                library
            };
            let mut lib = tex_lib.lock().unwrap();
            *lib = tex_l;
        });
    }
}