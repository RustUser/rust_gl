use std::any::Any;
use rlua::prelude::LuaContext;
use serde::*;
use crate::ecs::{Component, get_by_id, register_component};
use crate::ecs::lua_component::LuaComponent;
use crate::ecs::transform::Transform;
use crate::scene::COMPONENTS;

static mut ID: usize = 0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameObject {
    pub(crate) name: String,
    pub(crate) id: usize,
    pub(crate) tag: String,
    pub(crate) components: Vec<usize>,
}

impl GameObject {
    pub fn attach<T: Component + Default + 'static>(&mut self) {
        let mut comp = T::default();
        comp.items_mut().parent = self.id;
        comp.items_mut().id = comp.inc_id();
        self.components.push(comp.items().id);
        register_component(comp);
    }

    pub fn remove<T: Component + Default + 'static>(&mut self) -> Option<&'static mut Box<dyn Component>> {
        let _t = T::default();
        for comp in &self.components {
            if let Some(comp) = get_by_id(comp) {
                if comp.type_id() == _t.type_id() {
                    return Some(comp);
                }
            }
        }
        None
    }
    pub fn incremented_id() -> usize {
        unsafe {
            let id = ID;
            ID += 1;
            id
        }
    }

    pub fn empty() -> Self {
        let id = Self::incremented_id();
        Self {
            name: format!("GameObject_{}", id),
            id,
            tag: "".to_string(),
            components: vec![

            ]
        }
    }

    pub fn get_component<T: ToString>(&self, name: T) -> Option<&'static Box<dyn Component>> {
        unsafe {
            let components = &COMPONENTS;
            match components {
                Some(components) => {
                    if let Some(components) = components.get(&self.id) {
                        for component in components {
                            if component.name().eq_ignore_ascii_case(name.to_string().as_str()) {
                                return Some(component);
                            }
                        }
                    }
                    return None;
                }
                None => {
                    panic!("Unable to get component. Components not initialized.");
                }
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn tag_mut(&mut self) -> &mut String {
        &mut self.tag
    }
    pub fn components(&self) -> &Vec<usize> {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut Vec<usize> {
        &mut self.components
    }
}