use vecmath::Matrix4;
use crate::gfx::ui::{UIElement, UIElementData};

use crate::gfx::ui::custom_ui_property::CustomUIProperty;
use crate::gfx::ui::rectangle::Rectangle;

#[derive(Debug)]
pub struct Square(Rectangle);

impl Square {
    pub fn with_position(self, position: [u32; 2]) -> Square {
        Square(self.0.with_position(position))
    }
    pub fn with_custom_property(mut self, property: &dyn ToString, value: CustomUIProperty) -> Square {
        self.0.set_custom_property(property, value);
        self
    }

    pub fn scale(&self) -> Matrix4<f32> {
        self.0.scale()
    }

    pub fn new(position: [u32; 2], size: f32) -> Self {
        Self(Rectangle::new(position, [size; 2]))
    }
    pub fn data(&self) -> &UIElementData {
        self.0.data()
    }
    pub fn size(&self) -> f32 {
        self.0.size()[0]
    }
}

impl Default for Square {
    fn default() -> Self {
        Square::new([0; 2], 100f32)
    }
}

impl UIElement for Square {
    fn tag(&self) -> &'static str {
        "Square"
    }

    fn element_data(&self) -> &UIElementData {
        self.0.element_data()
    }

    fn element_data_mut(&mut self) -> &mut UIElementData {
        self.0.element_data_mut()
    }

    fn resize(&mut self, _size: [i32; 2]) {
        self.0.resize(_size);
    }

    fn contains_point(&self, point: [f64; 2]) -> bool {
        self.0.contains_point(point)
    }
}