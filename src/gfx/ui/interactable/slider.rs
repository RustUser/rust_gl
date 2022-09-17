use std::fmt::{Debug, Formatter};
use imgui_glfw_rs::glfw::Key;

use crate::gfx::ui::{Callbacks, ui_counter, UIElement, UIElementData};
use crate::{CustomUIProperty, Input, Layout, Rectangle, UI};
use crate::gfx::ui::callbacks::OnSliderValueChanged;
use crate::math::{clamp, inverse_lerp_f64};

pub struct Slider {
    value: f32,
    data: UIElementData,
    on_value_changed: Vec<Box<OnSliderValueChanged>>,
}

impl Debug for Slider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Slider [value: {}, data: {:?}, callbacks: {}]", self.value, self.data, self.on_value_changed.len()))
    }
}

impl Slider {
    pub fn new(handle: Rectangle, slider: Rectangle, value: f32, data: UIElementData) -> Self {
        Self { value, data, on_value_changed: vec![] }
            .with_child(slider)
            .with_child(handle)
    }

    pub fn register_on_value_changed(&mut self, callback: Box<OnSliderValueChanged>) {
        self.on_value_changed.push(callback);
    }

    pub fn with_on_value_changed(mut self, callback: Box<OnSliderValueChanged>) -> Slider {
        self.register_on_value_changed(callback);
        self
    }

    pub fn with_child(mut self, child: Rectangle) -> Slider {
        self.add_child(Box::new(child));
        self
    }

    pub fn calculate_slider_position(area: [f32; 4], v: f32, handle_size: [f32; 2]) -> [u32; 2] {
        let (area_x, area_y) = (area[0], area[1]);
        let (area_w, area_h) = (area[2] + area_x, area[3] + area_y);

        let h_x = area_w - (area[2] - area[2] * v) - (handle_size[0] / 2f32);
        let h_y = area_h - (area[3] / 2f32) - (handle_size[1] / 2f32);
        [h_x as u32, h_y as u32]
    }

    pub fn set_value(&mut self, value: f32) {
        if self.value == value || (value > 1f32 && self.value == 1f32) || (value < 0f32 && self.value == 0f32) {
            return;
        }
        let old = self.value;
        self.value = clamp(value, 0f32, 1f32);
        let value = self.value.clone();

        let area = {
            let slider = self.area();
            [
                slider.position()[0] as f32, slider.position()[1] as f32,
                slider.size()[0], slider.size()[1]
            ]
        };
        let handle = self.handle_mut();

        handle.set_position(Slider::calculate_slider_position(area, value, handle.size()));
        let me: &mut Slider = self.self_mut().unwrap().downcast_mut().unwrap();
        let nv = self.value;
        for callback in &mut self.on_value_changed {
            (callback)(me, old, nv);
        }
    }

    pub fn increment(&mut self, v: f32) {
        self.set_value(self.value + v);
    }

    pub fn decrement(&mut self, v: f32) {
        self.increment(-v);
    }

    pub fn with_corner_radius(mut self, corner_radius: f32) -> Slider {
        self.set_corner_radius(corner_radius);
        self
    }

    pub fn set_corner_radius(&mut self, corner_radius: f32) {
        self.data.corner_radius = corner_radius;
        //self.area_mut().element_data_mut().corner_radius = corner_radius;
        if !self.element_data().tmp_children.is_empty() {
            self.element_data_mut().tmp_children[0].element_data_mut().corner_radius = corner_radius;
        } else {
            self.area_mut().element_data_mut().corner_radius = corner_radius;
        }
    }

    fn area(&self) -> &Rectangle {
        UI::get_element_by_id(&self.children()[0]).unwrap().downcast_ref::<Rectangle>().unwrap()
    }

    fn area_mut(&mut self) -> &mut Rectangle {
        self.area_mut_checked().unwrap()
    }

    fn handle_checked(&self) -> Option<&Rectangle> {
        if self.children().len() == 0 {
            return None;
        }
        UI::get_element_by_id(&self.children()[1]).unwrap().downcast_ref::<Rectangle>()
    }

    fn area_checked(&self) -> Option<&Rectangle> {
        if self.children().len() == 0 {
            return None;
        }
        UI::get_element_by_id(&self.children()[0]).unwrap().downcast_ref::<Rectangle>()
    }

    fn area_mut_checked(&mut self) -> Option<&mut Rectangle> {
        if self.children().len() == 0 {
            return None;
        }
        UI::get_element_by_id_mut(&self.children()[0]).unwrap().downcast_mut::<Rectangle>()
    }

    #[allow(dead_code)]
    fn handle(&self) -> &Rectangle {
        UI::get_element_by_id(&self.children()[1]).unwrap().downcast_ref::<Rectangle>().unwrap()
    }

    fn handle_mut(&mut self) -> &mut Rectangle {
        UI::get_element_by_id_mut(&self.children()[1]).unwrap().downcast_mut::<Rectangle>().unwrap()
    }
}

impl Default for Slider {
    fn default() -> Self {
        let id = ui_counter();
        let area = [
            14f32, 350f32,
            250f32, 30f32
        ];
        let area_position = [area[0] as u32, area[1] as u32];
        let handle_size = [10f32, 40f32];
        let handle_position = Slider::calculate_slider_position(area, 0f32, handle_size);

        let slider = Rectangle::new(
            area_position,
            [area[2], area[3]],
        ).with_custom_property(&"color", CustomUIProperty::Vec4([0.6, 0.6, 0.6, 1.0]));

        let mut handle = Rectangle::new(
            handle_position,
            handle_size,
        ).with_custom_property(&"color", CustomUIProperty::Vec4([0.3, 0.3, 0.3, 1.0]));
        handle.register_on_drag(Box::new(move |handle, _, mouse, _relative_mouse| {
            let slider = handle.get_parent_mut().unwrap().downcast_mut::<Slider>().unwrap();
            let area = slider.area();
            let area = [
                (area.position()[0] - handle.element_data().width / 2) as f64,
                area_position[1] as f64,
                ((area.position()[0] + area.size()[0] as u32) - handle.element_data().width / 2) as f64,
                (area.position()[1] + area.size()[1] as u32) as f64
            ];

            let pos = handle.position();
            let ui_mouse = Input::ui_cursor();

            let mut x = ui_mouse[0] - mouse[0];

            if x <= area[0] {
                x = area[0];
            } else if x >= area[2] {
                x = area[2];
            }

            let a_x = area[0];
            let a_w = area[2] - (handle.element_data().width) as f64;

            let a_xw = a_x + a_w;

            let i_lerp = inverse_lerp_f64(a_x, a_xw, x);

            let ui_mouse = [x as u32, pos[1]];
            handle.set_position(ui_mouse);
            slider.set_value(i_lerp as f32);
        }));

        Self {
            value: 0f32,
            data: UIElementData {
                id,
                name: "".to_string(),
                parent: None,
                children: vec![],
                position: area_position,
                width: area[2] as u32,
                height: area[3] as u32,
                tmp_children: vec![
                    Box::new(slider),
                    Box::new(handle),
                ],
                render_data: None,
                hover_flag: false,
                custom_properties: Default::default(),
                corner_radius: 0.0,
                drag_offset: None,
                callbacks: Callbacks { on_drag: vec![] },
                child_horizontal: Layout::MatchParent(0, 0),
                child_vertical: Layout::MatchParent(0, 0),
                spacing: 0
            },
            on_value_changed: vec![],
        }
    }
}

impl UIElement for Slider {
    fn element_data(&self) -> &UIElementData {
        &self.data
    }

    fn element_data_mut(&mut self) -> &mut UIElementData {
        &mut self.data
    }

    fn resize(&mut self, _size: [i32; 2]) {}

    fn contains_point(&self, _point: [f64; 2]) -> bool {
        false
    }

/*    fn draw(&self, camera: &Camera) {
        self.draw_children(camera);
        /*if let Some(area) = self.area_checked() {
            area.draw(camera);
        }
        if let Some(handle) = self.handle_checked() {
            handle.draw(camera);
        }*/
        //self.area().draw(camera);
        //self.handle().draw(camera);
    }*/

    fn update(&mut self, delta: f32) {
        let value = match Input::is_key_held(Key::Left) {
            true => -delta,
            false => {
                match Input::is_key_held(Key::Right) {
                    true => delta,
                    false => 0f32
                }
            }
        };
        self.increment(value);
    }

    fn tag(&self) -> &'static str {
        "Slider"
    }
}