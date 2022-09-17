use std::collections::HashMap;
use std::fmt::Debug;
use downcast_rs::{Downcast, impl_downcast};
use imgui_glfw_rs::imgui::{TreeNodeFlags, Ui};
use maplit::hashmap;
use serde::*;
use crate::Camera;
use crate::ecs::game_object::GameObject;
use crate::ecs::lua_component::Value;
use crate::math::linear_algebra::types::Vec3;
use crate::scene::Scene;

pub mod game_object;
pub mod transform;

pub mod colliders;
pub mod lua_component;

static mut ID: usize = 0;
static mut COMPONENTS: Option<HashMap<usize, Box<dyn Component + 'static>>> = None;

pub fn draw_vec3(v: &mut Vec3, ui: &Ui, name: &str, id: usize) {
    ui.group(|| {
        let width = ui.window_size()[0] - 15.0;
        let width = width / 3.0;
        ui.text(name);
        ui.push_item_width(width);
        ui.input_float(format!("##_{}_{}_x", id, name), &mut v[0]).build();
        ui.push_item_width(width);
        ui.same_line();
        ui.input_float(format!("##_{}_{}_y", id, name), &mut v[1]).build();
        ui.push_item_width(width);
        ui.same_line();
        ui.input_float(format!("##_{}_{}_z", id, name), &mut v[2]).build();
    });
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentItems {
    #[serde(skip)]
    state: ComponentState,
    enabled: bool,
    parent: usize,
    id: usize,
}

impl ComponentItems {
    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn state(&self) -> &ComponentState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.state
    }
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn parent(&self) -> usize {
        self.parent
    }
}

#[derive(Debug, Clone)]
pub enum ComponentState {
    Awake,
    Start,
    Ready,
}

impl Default for ComponentState {
    fn default() -> Self {
        Self::Awake
    }
}

pub fn init_components() {
    unsafe { COMPONENTS = Some(hashmap! {}); }
}

pub fn register_component<C: Component + 'static>(component: C) {
    let component = Box::new(component);
    unsafe {
        if let Some(components) = COMPONENTS.as_mut() {
            components.insert(component.items().id, component);
        }
    }
}

fn get_by_id(id: &usize) -> Option<&'static mut Box<dyn Component>> {
    unsafe {
        if let Some(components) = &mut COMPONENTS {
            return components.get_mut(id);
        }
    }
    None
}

impl_downcast!(Component);

pub trait Component: Debug + Downcast {
    fn name(&self) -> &String;
    fn parent_id(&self) -> &usize {
        &self.items().parent
    }
    fn items(&self) -> &ComponentItems;
    fn items_mut(&mut self) -> &mut ComponentItems;
    fn state(&self) -> &ComponentState {
        &self.items().state
    }
    fn state_mut(&mut self) -> &mut ComponentState {
        &mut self.items_mut().state
    }
    fn enabled(&self) -> &bool {
        &self.items().enabled
    }
    fn set_enabled(&mut self, flag: bool) {
        self.items_mut().enabled = flag;
    }
    fn parent(&self) -> Option<&'static mut GameObject> {
        Scene::get_object(self.parent_id())
    }
    fn awake(&mut self) {}
    fn start(&mut self) {}
    fn update(&mut self) {}
    fn late_update(&mut self) {}
    fn on_enable(&self) {}
    fn on_disable(&self) {}
    fn inc_id(&self) -> usize {
        unsafe {
            let id = ID;
            ID += 1;
            id
        }
    }
    fn parent_mut(&mut self) -> &mut usize {
        &mut self.items_mut().parent
    }
    fn imgui(&mut self, ui: &Ui) {
        if ui.collapsing_header(self.name(), TreeNodeFlags::DEFAULT_OPEN) {
            self.imgui_context(ui);
        }
    }
    fn imgui_context(&mut self, ui: &Ui) {}
    fn set(&mut self, key: &String, value: &Value)  -> ECSResult;
    fn render(&mut self, camera: &Camera) {}
}

pub type ECSResult = Option<String>;