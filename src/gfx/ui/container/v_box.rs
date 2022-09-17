use crate::gfx::ui::container::{Container, ContainerData};
use crate::gfx::ui::{Callbacks, ui_counter, UIElementData, UIRenderData};
use crate::math::{clamp_u32};
use crate::{BufferDataType, BufferType, Camera, Constructor, CustomUIProperty, DrawType, LocalAttribPointer, UI, UIElement, VertexArrayObject, VertexArrayObjectType, VertexBufferObject, WINDOW_SIZE};
use crate::gfx::ui::layout::Layout;
use crate::gfx::ui::rectangle::BUFFER;

#[derive(Debug)]
pub struct VBox {
    element_data: UIElementData,
    container_data: ContainerData,
}

impl VBox {
    pub fn new() -> VBox {
        let id = ui_counter();

        let vao = VertexArrayObject::new(Some(VertexArrayObjectType::ArrayStrips(4)))
            .with_buffer(VertexBufferObject::array(BufferType::ArrayBuffer, DrawType::StaticDraw, &BUFFER))
            .with_local_attrib_pointer(LocalAttribPointer::new(2, BufferDataType::Float, false))
            .with_local_attrib_pointer(LocalAttribPointer::new(2, BufferDataType::Float, false))
            .build();
        Self {
            element_data: UIElementData {
                id,
                name: "".to_string(),
                parent: None,
                children: vec![],
                position: [0; 2],
                width: 0,
                height: 0,
                tmp_children: vec![],
                render_data: Some(UIRenderData(UI::default_program(), vao, Default::default())),
                hover_flag: false,
                custom_properties: Default::default(),
                corner_radius: 0.0,
                drag_offset: None,
                callbacks: Callbacks { on_drag: vec![] },
                child_horizontal: Layout::MatchParent(0, 0),
                child_vertical: Layout::MatchParent(0, 0),
                spacing: 0
            },
            container_data: ContainerData {
                min_width: 0,
                max_width: None,
                min_height: 0,
                max_height: None,
                debug_flag: false,
                horizontal: Layout::Absolute,
                vertical: Layout::Absolute,
            },
        }
    }

    pub fn with_position(mut self, position: [u32; 2]) -> VBox {
        self.element_data.position = position;
        self
    }

    pub fn with_horizontal(mut self, horizontal: Layout) -> VBox {
        *self.horizontal_mut() = horizontal;
        self
    }

    pub fn with_vertical(mut self, vertical: Layout) -> VBox {
        *self.vertical_mut() = vertical;
        self
    }

    pub fn with_child_vertical(mut self, vertical: Layout) -> VBox {
        *self.child_vertical_mut() = vertical;
        self
    }

    pub fn with_child_horizontal(mut self, horizontal: Layout) -> VBox {
        *self.child_horizontal_mut() = horizontal;
        self
    }

    pub fn with_child<E: UIElement>(mut self, element: E) -> VBox {
        self.add_child(Box::new(element));
        self
    }

    pub fn with_children<E: UIElement>(mut self, element: Vec<E>) -> VBox {
        for child in element {
            self.add_child(Box::new(child));
        }
        self
    }

    pub fn with_min_width(mut self, width: u32) -> VBox {
        *self.min_width_mut() = width;
        self
    }
    pub fn with_max_width(mut self, width: u32) -> VBox {
        *self.max_width_mut() = Some(width);
        self
    }

    pub fn with_min_height(mut self, height: u32) -> VBox {
        *self.min_height_mut() = height;
        self
    }
    pub fn with_max_height(mut self, height: u32) {
        *self.max_height_mut() = Some(height);
    }

    pub fn with_custom_property<T: ToString>(mut self, key: T, value: CustomUIProperty) -> VBox {
        self.set_custom_property(&key.to_string(), value);
        self
    }

    pub fn with_spacing(mut self, spacing: u32) -> VBox {
        *self.spacing_mut() = spacing;
        self
    }
}

impl Default for VBox {
    fn default() -> Self {
        VBox::new()
    }
}

impl Container for VBox {
    fn container_data(&self) -> &ContainerData {
        &self.container_data
    }

    fn container_data_mut(&mut self) -> &mut ContainerData {
        &mut self.container_data
    }
}

impl UIElement for VBox {
    fn tag(&self) -> &'static str {
        "VBox"
    }

    fn element_data(&self) -> &UIElementData {
        &self.element_data
    }

    fn element_data_mut(&mut self) -> &mut UIElementData {
        &mut self.element_data
    }

    fn resize(&mut self, _size: [i32; 2]) {}

    fn contains_point(&self, _point: [f64; 2]) -> bool {
        false
    }

    fn update(&mut self, _delta: f32) {
        let height = match self.vertical() {
            Layout::MatchParent(top, _bottom) => {
                let height = match self.get_parent() {
                    None => unsafe {
                        //No parent found. Get height of window.
                        let window_height = WINDOW_SIZE[1] as u32 - top;
                        window_height
                    }
                    Some(parent) => {
                        parent.element_data().height
                    }
                };
                height
            }
            Layout::Absolute => self.element_data.height
        };


        self.element_data.width = clamp_u32(self.element_data.width, self.container_data.min_width, self.container_data().max_width.unwrap_or(u32::MAX));
        self.element_data.height = clamp_u32(height, self.container_data.min_height, self.container_data.max_height.unwrap_or(u32::MAX));

        let mut y_offset = 0;
        for child in self.element_data.get_children_mut() {
            let child = child.unwrap();
            child.set_position([self.position()[0], y_offset]);

            match self.child_horizontal() {
                Layout::MatchParent(_left, _right) => {
                    child.element_data_mut().width = self.element_data.width;
                }
                Layout::Absolute => {}
            }

            y_offset += child.element_data().height() + self.element_data.spacing;
        }
    }
    fn draw(&self, camera: &Camera) {
        for child in &self.element_data.get_children() {
            let child = child.unwrap();
            //println!("Child size: {:?}", [child.element_data().width, child.element_data().height]);
            child.draw(camera);
        }
    }
}