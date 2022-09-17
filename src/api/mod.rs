use std::error::Error;

use rlua::prelude::LuaContext;

pub mod api_use;
pub mod web;
pub mod collections;
pub mod var_args;

pub trait LuaObject {
    fn load_constructor(context: &LuaContext) -> Result<(), Box<dyn Error>>;
}