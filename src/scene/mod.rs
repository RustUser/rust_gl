use std::borrow::BorrowMut;
use std::collections::HashMap;
use maplit::hashmap;
use serde::*;
use crate::ecs::Component;
use crate::ecs::game_object::GameObject;
use crate::OBJ;

static mut SCENE: Option<Scene> = None;
static mut OBJECTS: Option<HashMap<usize, GameObject>> = None;
pub static mut COMPONENTS: Option<HashMap<usize, Vec<Box<dyn Component + 'static>>>> = None;

#[derive(Debug)]
pub struct Scene {
    pub(crate) name: String,
    pub(crate) id: usize,
    //pub(crate) objects: HashMap<usize, GameObject>,
}

impl Scene {
    pub fn new<T: ToString>(name: T) -> Scene {
        Self {
            name: name.to_string(),
            id: 0,
            //objects: hashmap! {},
        }
    }

    pub fn clear_objects() {
        unsafe { OBJECTS = Some(HashMap::new()); }
    }

    pub fn load_scene(scene: Scene) {
        unsafe { SCENE = Some(scene); }
    }

    pub fn get_object(id: &usize) -> Option<&'static mut GameObject> {
        unsafe {
            if let Some(objects) = &mut OBJECTS {
                return objects.get_mut(id);
            }
        }
        None
    }

    pub fn objects_mut() -> &'static mut HashMap<usize, GameObject> {
        unsafe {
            match &mut OBJECTS {
                Some(objects) => {
                    objects
                }
                None => {
                    panic!("Not initialized.")
                }
            }
        }
    }
}