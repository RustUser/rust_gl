//Fill methods

use crate::Program;

#[derive(Debug, Clone, Copy)]
pub enum FillMethod {
    Solid,
    ///isLeftToRight, ratio
    Horizontal(bool, f32),
    ///isTopToBottom, ratio
    Vertical(bool, f32),
}

impl Default for FillMethod {
    fn default() -> Self {
        Self::Solid
    }
}

impl FillMethod {
    pub fn bind<T: ToString>(&self, program: &Program, name: T) {
        let name = name.to_string();
        let fill_method: u8 = (*self).into();

        program.set_uniform_int(format!("{}.fillMethod", name), &(fill_method as i32));
        let ratio = match self {
            FillMethod::Solid => {
                1f32
            }
            FillMethod::Horizontal(l_to_r, ratio) => {
                program.set_uniform_bool(format!("{}.direction", name), l_to_r);
                *ratio
            }
            FillMethod::Vertical(top_to_bottom, ratio) => {
                program.set_uniform_bool(format!("{}.direction", name), top_to_bottom);
                *ratio
            }
        };
        program.set_uniform_float(format!("{}.ratio", name), &ratio);
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        match self {
            FillMethod::Horizontal(_, r) => *r = ratio,
            FillMethod::Vertical(_, r) => *r = ratio,
            _ => {
                //Omit ratio setting of a solid.
            }
        }
    }
}

impl Into<u8> for FillMethod {
    fn into(self) -> u8 {
        match self {
            FillMethod::Solid => 0,
            FillMethod::Horizontal(_, _) => 1,
            FillMethod::Vertical(_, _) => 2
        }
    }
}