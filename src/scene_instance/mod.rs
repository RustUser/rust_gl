use maplit::hashmap;
use crate::scene::Scene;
use crate::scene_instance::game_object_instance::IGameObject;

pub mod game_object_instance;
pub mod entry_point;

#[derive(Debug, Clone)]
pub struct IScene {
    pub name: String,
    pub id: usize,
    pub objects: Vec<IGameObject>
}

impl IScene {
    pub fn load(&self) -> Scene {
        let mut objects = hashmap! {};
        for object in &self.objects {
            let object = object.load();
            objects.insert(object.id, object);
        }
        Scene {
            name: self.name.clone(),
            id: self.id,
        }
    }
}