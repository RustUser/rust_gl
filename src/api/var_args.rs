use std::collections::HashMap;
use std::error::Error;
use rlua::{Context,  MetaMethod, UserData, UserDataMethods};
use crate::LuaObject;

#[derive(Debug, Clone)]
pub enum Var {
    Bool(bool),
    String(String),
    Integer(i32),
}

#[derive(Debug, Clone)]
pub struct VarArgs {
    args: HashMap<String, Var>,
}

impl Into<VarArgs> for HashMap<String, Var> {
    fn into(self) -> VarArgs {
        VarArgs {
            args: self
        }
    }
}

impl Var {
    pub fn bool(&self) -> Option<&bool> {
        match self {
            Var::Bool(b) => {
                Some(b)
            }
            _ => {
                None
            }
        }
    }
    pub fn int(&self) -> Option<&i32> {
        match self {
            Var::Integer(int) => {
                Some(int)
            }
            _ => {
                None
            }
        }
    }
}

impl From<bool> for Var {
    fn from(b: bool) -> Self {
        Var::Bool(b)
    }
}

impl Into<Var> for i32 {
    fn into(self) -> Var {
        Var::Integer(self)
    }
}

impl ToString for Var {
    fn to_string(&self) -> String {
        match self {
            Var::Bool(b) => {
                b.to_string()
            }
            Var::String(s) => {
                s.clone()
            }
            Var::Integer(i) => {
                i.to_string()
            }
        }
    }
}

impl UserData for Var {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(_methods: &mut T) {
        _methods.add_meta_method(MetaMethod::ToString, |_, v, _: ()| {
            Ok(v.to_string())
        });
    }
}

impl LuaObject for Var {
    fn load_constructor(context: &Context) -> Result<(), Box<dyn Error>> {
        context.globals().set("bool", context.create_function(|_, value: bool| {
            Ok(Var::Bool(value))
        })?)?;

        context.globals().set("int", context.create_function(|_, value: i32| {
            Ok(Var::Integer(value))
        })?)?;

        Ok(())
    }
}

impl UserData for VarArgs {}