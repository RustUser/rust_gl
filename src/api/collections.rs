use std::collections::HashMap;
use std::error::Error;
use rlua::prelude::LuaContext;
use rlua::Table;
use crate::api::var_args::Var;
use crate::LuaObject;

pub struct Collections;

impl LuaObject for Collections {
    fn load_constructor(context: &LuaContext) -> Result<(), Box<dyn Error>> {
        let globals = context.globals();
        {
            let f = context.create_function(|_, (map, fallback): (HashMap<String, Var>, Option<String>)| {
                let mut v = map.iter().map(|(k, v)| {
                    format!("{}={}", k, v.to_string())
                }).collect::<Vec<String>>().join(",").trim().to_string();
                if v.is_empty() {
                    v = fallback.unwrap_or(String::new());
                } else {
                    v = format!("[{}]", v);
                }
                Ok(v)
            }).unwrap();
            globals.set("joinMap", f).unwrap();
        }
        {
            let f = context.create_function(|_, (values, fallback): (Vec<Var>, Option<String>)| {
                let mut v = values.iter().map(|v| {
                    v.to_string()
                }).collect::<Vec<String>>().join(",").trim().to_string();
                if v.is_empty() {
                    v = fallback.unwrap_or(String::new());
                } else {
                    v = format!("[{}]", v);
                }
                Ok(v)
            }).unwrap();
            globals.set("join", f).unwrap();
        }
        {
            let f = context.create_function(|_, (table, _fallback): (Table, Option<String>)| {
                let _table = table.sequence_values::<Var>();

                Ok(())
            }).unwrap();
            globals.set("join", f).unwrap();
        }
        Ok(())
    }
}