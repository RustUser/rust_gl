use crate::gfx::ui::{Callbacks, ui_counter, UIElementData, UIRenderData};
use crate::{Camera, CustomUIProperty, Layout, Texture, UI, UIElement};
use crate::gfx::ui::rectangle::BUFFER;

#[derive(Debug)]
pub struct Image {
    data: UIElementData
}

impl Image {
    pub fn new(texture: Option<Texture>) -> Image {
        let mut vao = UI::generic_vao(&BUFFER);
        let (width, height) = match texture {
            None => {
                (0, 0)
            }
            Some(texture) => {
                let size = (texture.width(), texture.height());
                vao.put_texture::<&str>(None, texture);
                size
            }
        };

        Self {
            data: UIElementData {
                id: ui_counter(),
                name: "".to_string(),
                parent: None,
                children: vec![],
                position: [0, height as u32],
                width: width as u32,
                height: height as u32,
                tmp_children: vec![],
                render_data: Some(UIRenderData(UI::default_program(), vao, Default::default())),
                hover_flag: false,
                custom_properties: maplit::hashmap! {
                    "color".to_string() => CustomUIProperty::Vec4([0.0, 1.0, 0.0, 1.0])
                },
                corner_radius: 0.0,
                drag_offset: None,
                callbacks: Callbacks { on_drag: vec![] },
                child_horizontal: Layout::Absolute,
                child_vertical: Layout::Absolute,
                spacing: 0
            },
        }
    }
}

impl UIElement for Image {
    fn tag(&self) -> &'static str {
        "Image"
    }

    fn element_data(&self) -> &UIElementData {
        &self.data
    }

    fn element_data_mut(&mut self) -> &mut UIElementData {
        &mut self.data
    }

    fn resize(&mut self, _: [i32; 2]) {

    }

    fn draw(&self, camera: &Camera) {
        self.default_draw(camera);
    }
}