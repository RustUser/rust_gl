use std::collections::HashMap;
use crate::ecs::game_object::GameObject;

#[derive(Debug, Clone)]
pub struct IComponent {
    pub name: String,
    pub data: HashMap<String, String>
}

#[derive(Debug, Clone)]
pub struct IGameObject {
    pub name: String,
    pub tag: String,
    pub attached: Vec<IComponent>
}

impl IGameObject {
    pub fn load(&self) -> GameObject {
        GameObject {
            name: self.name.clone(),
            id: GameObject::incremented_id(),
            tag: self.tag.clone(),
            components: vec![

            ]
        }
    }
}