use crate::gfx::ui::layout::Layout;
use crate::UIElement;

pub mod v_box;

#[derive(Debug, Clone)]
pub struct ContainerData {
    min_width: u32,
    max_width: Option<u32>,

    min_height: u32,
    max_height: Option<u32>,
    debug_flag: bool,

    horizontal: Layout,
    vertical: Layout,
}

pub trait Container: UIElement {
    fn container_data(&self) -> &ContainerData;
    fn container_data_mut(&mut self) -> &mut ContainerData;

    fn min_width(&self) -> u32 {
        self.container_data().min_width
    }
    fn min_width_mut(&mut self) -> &mut u32 {
        &mut self.container_data_mut().min_width
    }
    fn max_width(&self) -> Option<u32> {
        self.container_data().max_width
    }
    fn max_width_mut(&mut self) -> &mut Option<u32> {
        &mut self.container_data_mut().max_width
    }

    fn min_height(&self) -> u32 {
        self.container_data().min_height
    }
    fn min_height_mut(&mut self) -> &mut u32 {
        &mut self.container_data_mut().min_height
    }
    fn max_height(&self) -> Option<u32> {
        self.container_data().max_height
    }
    fn max_height_mut(&mut self) -> &mut Option<u32> {
        &mut self.container_data_mut().max_height
    }

    fn debug_flag(&self) -> bool {
        self.container_data().debug_flag
    }
    fn debug_flag_mut(&mut self) -> &mut bool {
        &mut self.container_data_mut().debug_flag
    }

    fn horizontal(&self) -> Layout { self.container_data().horizontal }
    fn horizontal_mut(&mut self) -> &mut Layout { &mut self.container_data_mut().horizontal }
    fn vertical(&self) -> Layout { self.container_data().vertical }
    fn vertical_mut(&mut self) -> &mut Layout { &mut self.container_data_mut().vertical }
}