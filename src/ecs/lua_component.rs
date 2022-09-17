use std::collections::HashMap;
use imgui_glfw_rs::imgui::Ui;
use rlua::{Lua, UserData, UserDataMethods};
use rlua::prelude::{LuaContext, LuaFunction};
use crate::ecs::{Component, ComponentItems, draw_vec3, ECSResult};
use crate::math::linear_algebra::types::{Vec3, Vec4};

#[derive(Debug, Clone)]
pub enum Value {
    Float(f32),
    String(String),
    Vec3(Vec3),
}

#[derive(Debug, Clone)]
pub struct LuaComponent {
    name: String,
    values: HashMap<String, Option<Value>>,
    component_items: ComponentItems,
}

impl LuaComponent {
    pub fn new(name: String, parent: usize, source_code: &String) -> LuaComponent {
        let mut component = Self {
            name,
            values: Default::default(),
            component_items: ComponentItems {
                state: Default::default(),
                enabled: false,
                parent,
                id: 0,
            },
        };
        let id = component.inc_id();
        let lua = Lua::new();
        let src = source_code.clone();
        component.component_items.id = id;
        let component = lua.context(|ctx| {
            LuaComponent::load_lua(&ctx, component);
            let globals = ctx.globals();
            ctx.load(&src).exec().unwrap();
            globals.get("self").unwrap()
        });

        component
    }

    fn load_lua(ctx: &LuaContext, component: LuaComponent) {
        let globals = ctx.globals();
        globals.set("self", component).unwrap();
        Self::init(ctx);
    }

    pub fn init(ctx: &LuaContext) {
        let globals = ctx.globals();
        let func = ctx.create_function(|_, input: String| {
            Ok(Value::String(input))
        }).unwrap();
        globals.set("string", func).unwrap();

        let func = ctx.create_function(|_, v: Vec3| {
            Ok(Value::Vec3(v))
        }).unwrap();
        globals.set("vec3", func);
    }
}

impl UserData for Value {}

impl UserData for LuaComponent {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(_methods: &mut T) {
        _methods.add_method_mut("field", |_, me, (field, value): (String, Option<Value>)| {
            Ok(me.values.insert(field, value))
        });
    }
}

impl Component for LuaComponent {
    fn name(&self) -> &String {
        &self.name
    }

    fn set(&mut self, key: &String, value: &Value) -> ECSResult {
        if !self.values.contains_key(key) {
            return Some(format!("Variable '{}' with value: '{:?}' could not be assigned to component '{}'", key, value, self.name));
        }
        self.values.insert(key.clone(), Some(value.clone()));
        None
    }

    fn items(&self) -> &ComponentItems {
        &self.component_items
    }

    fn items_mut(&mut self) -> &mut ComponentItems {
        &mut self.component_items
    }

    fn imgui_context(&mut self, ui: &Ui) {
        println!("{:?}", self.values);
        for (k, v) in &mut self.values {
            if let Some(v) = v {
                ui.group(|| {
                    //let width = ui.window_size()[0] - 15.0;
                    ui.text(self.name.clone());
                    match v {
                        Value::Float(f) => {
                            ui.input_float(k, f).build();
                        }
                        Value::String(s) => {
                            ui.input_text(k, s).build();
                        }
                        Value::Vec3(v3) => {
                            draw_vec3(v3, ui, k.as_str(), 0);
                        }
                    }
                });
            }
        }
    }

    fn update(&mut self) {
        let src = r#"
        function update() {
            print("Hello!")
        }

        "#;
        let lua = Lua::new();
        lua.context(|ctx|{
            let globals = ctx.globals();
            ctx.load(src).exec();
            let update: LuaFunction = globals.get("update").unwrap();
            update.call::<_, ()>(()).unwrap();
        });
    }
}