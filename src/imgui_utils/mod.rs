use imgui_glfw_rs::imgui::InputFloat;

pub trait Widget {
    fn build_widget(&mut self);
}

pub fn grid(width: f32, widgets: &[Box<dyn Widget>]) {

}

impl <L>  Widget for InputFloat<'_, '_, L> {
    fn build_widget(&mut self) {
        //self.build();
    }
}