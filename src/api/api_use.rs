
use std::error::Error;
use rlua::prelude::{LuaContext};
use rlua::UserData;
use crate::api::collections::Collections;
use crate::api::LuaObject;
use crate::api::var_args::{Var};

pub struct Use {
    registry: Vec<String>,
}

impl Use {}

impl LuaObject for Use {
    fn load_constructor(context: &LuaContext) -> Result<(), Box<dyn Error>> {
        let _use = context.create_function(|context, registry: Vec<String>| {
            for value in &registry {
                match value.as_str() {
                    "xml" => {}
                    "collections" => {
                        Collections::load_constructor(&context).unwrap();
                    }
                    "var" => {
                        Var::load_constructor(&context).unwrap();
                    }
                    &_ => {
                        eprintln!("Use not supported: {}", value);
                    }
                }
            }
            Ok(Use { registry })
        })?;
        context.globals().set("use", _use)?;
        Ok(())
    }
}

impl UserData for Use {}