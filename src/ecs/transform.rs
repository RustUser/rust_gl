use imgui_glfw_rs::imgui::{InputFloat, InputTextFlags, Ui};
use crate::ecs::{Component, ComponentItems, ComponentState, draw_vec3, ECSResult};
use crate::math::linear_algebra::types::Vec3;
use serde::*;
use crate::ecs::lua_component::Value;

pub const TRANSFORM_NAME: &'static str = "Transform";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    name: String,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    items: ComponentItems,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            name: TRANSFORM_NAME.to_string(),
            position: [0.0; 3],
            rotation: [0.0; 3],
            scale: [1.0; 3],
            items: ComponentItems {
                state: ComponentState::Awake,
                enabled: true,
                parent: 0,
                id: 0,
            },
        }
    }
}

impl Component for Transform {
    fn name(&self) -> &String {
        &self.name
    }

    fn items(&self) -> &ComponentItems {
        &self.items
    }

    fn items_mut(&mut self) -> &mut ComponentItems {
        &mut self.items
    }

    fn set(&mut self, key: &String, value: &Value) -> ECSResult {
        match &*key.to_lowercase() {
            "position" => {
                if let Value::Vec3(vec3) = value {
                    self.position = *vec3;
                    return None;
                } else {
                    return Some(format!("Value missmatch in component '{}'. Expecting typeof 'Vec3', found '{:?}'.", self.name, value));
                }
            }
            "rotation" => {
                if let Value::Vec3(vec3) = value {
                    self.rotation = *vec3;
                    return None;
                } else {
                    return Some(format!("Value missmatch in component '{}'. Expecting typeof 'Vec3', found '{:?}'.", self.name, value));
                }
            }
            "scale" => {
                if let Value::Vec3(vec3) = value {
                    self.scale = *vec3;
                    return None;
                } else {
                    return Some(format!("Value missmatch in component '{}'. Expecting typeof 'Vec3', found '{:?}'.", self.name, value));
                }
            }
            _ => {}
        }
        Some(format!("Variable '{}' with value: '{:?}' could not be assigned to component '{}'", key, value, self.name))
    }

    fn imgui_context(&mut self, ui: &Ui) {
        {
            draw_vec3(&mut self.position, ui, "Position", self.items.id);
            draw_vec3(&mut self.rotation, ui, "Rotation", self.items.id);
            draw_vec3(&mut self.scale, ui, "Scale", self.items.id);
        }
    }

    fn awake(&mut self) {
        println!("Aye welcome");
    }
}