use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use downcast_rs::{Downcast, impl_downcast};
use imgui_glfw_rs::glfw::*;
use vecmath::{Vector4};
use crate::{BufferDataType, BufferType, Camera, Constructor, cull_face, draw_arrays, DrawMode, DrawType, enable, Face, FragmentShader, GLConsts, Input, Layout, LocalAttribPointer, MatrixWrapper, Program, VertexArrayObject, VertexArrayObjectType, VertexBufferObject, VertexShader};
use crate::gfx::bindings::texturing::active_texture;
use crate::gfx::texture::Texture;
use crate::gfx::ui::callbacks::OnDrag;
use crate::gfx::ui::custom_ui_property::CustomUIProperty;
use crate::gfx::ui::fill_method::FillMethod;
use crate::math::linear_algebra::IDENTITY_MAT4;

pub mod rectangle;
pub mod square;
pub mod image;

pub mod container;
pub mod layout;

pub mod interactable;

pub mod glyph_cache;

pub mod callbacks;
pub mod fill_method;
pub mod custom_ui_property;

pub const DEFAULT_COLOR: Vector4<f32> = [0f32, 0f32, 0f32, 1f32];
static mut COUNTER: usize = 0;
static mut UI: Option<UI> = None;

const DEFAULT_VERTEX_SHADER: &'static str = {
    r#"
#version 330 core

uniform mat4 projection;
uniform mat4 model;

out vec2 currentPos;
out vec2 uv;

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aUV;

void main() {
    gl_Position = projection * model * vec4(aPos, 0, 1);
    currentPos = aPos;
    uv = aUV;
}

"#
};
const DEFAULT_FRAGMENT_SHADER: &'static str = include_str!("../../../res/shaders/ui/frag.glsl");
/*const DEFAULT_FRAGMENT_SHADER: &'static str = {
    r#"
#version 330 core

uniform vec2 bottomLeft;
uniform vec2 size;
uniform float cornerRadius;

uniform vec4 color;

in vec2 currentPos;
out vec4 FragColor;

void main()
{
    vec2 a_uv = currentPos;
    float u_radius = cornerRadius;
    vec2 u_dimensions = size;

    vec2 coords = a_uv * u_dimensions;



    if (
        length(coords - vec2(0) < u_radius ||
        length(coords - vec2(0, u_dimensions.y) < u_radius ||
        length(coords - vec2(u_dimensions.x, 0) < u_radius ||
        length(coords - u_dimensions) < u_radius
        )
    {
        discard;
    }


    FragColor = color;
}
"#
};*/

pub struct UI {
    elements: HashMap<usize, Box<dyn UIElement>>,
    default_program: Program,
}

impl UI {
    pub unsafe fn init() {
        UI = Some(UI {
            elements: Default::default(),
            default_program: Program::new(
                FragmentShader::new(DEFAULT_FRAGMENT_SHADER),
                VertexShader::new(DEFAULT_VERTEX_SHADER),
            ).unwrap(),
        })
    }
    pub fn update(delta: f32) {
        unsafe {
            match &mut UI {
                None => {
                    panic!("Initialize the UI first.")
                }
                Some(ui) => {
                    for (_, elem) in &mut ui.elements {
                        elem.update(delta);
                    }
                }
            }
        }
    }

    pub fn get_element_by_id(id: &usize) -> Option<&'static Box<dyn UIElement>> {
        unsafe {
            match &mut UI {
                None => {
                    panic!("Initialize the UI first.")
                }
                Some(ui) => {
                    for (elem_id, elem) in &ui.elements {
                        if elem_id == id {
                            return Some(elem);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_element_by_id_mut(id: &usize) -> Option<&'static mut Box<dyn UIElement>> {
        unsafe {
            match &mut UI {
                None => {
                    panic!("Initialize the UI first.")
                }
                Some(ui) => {
                    for (elem_id, elem) in &mut ui.elements {
                        if elem_id == id {
                            return Some(elem);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn draw(camera: &Camera) {
        cull_face(Face::Front);
        enable(GLConsts::Blend);
        unsafe {
            match &mut UI {
                None => panic!("Initialize the UI first."),
                Some(ui) => {
                    let mut keys = ui.elements.keys().into_iter().collect::<Vec<&usize>>();
                    keys.sort_by(|a, b| b.cmp(a));

                    for index in keys {
                        match ui.elements.get(index) {
                            None => panic!("Getting a ui element by an invalid id is not possible."),
                            Some(elem) => {
                                if elem.is_root() {
                                    elem.draw(camera);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update_screen_size(screen_size: [u32; 2]) {
        unsafe {
            match &mut UI {
                None => panic!("Initialize the UI first."),
                Some(ui) => {
                    for (_id, elem) in &mut ui.elements {
                        elem.resize([screen_size[0] as i32, screen_size[1] as i32]);
                    }
                }
            }
        }
    }

    pub fn update_key(key: Key, code: Action, delta: f32) {
        unsafe {
            if let Some(ui) = &mut UI {
                for (_, elem) in &mut ui.elements {
                    elem.update_key(key, code, delta);
                }
            }
        }
    }

    pub fn update_mouse(button: MouseButton, action: Action) {
        unsafe {
            if let Some(ui) = &mut UI {
                let mut keys = ui.elements.keys().into_iter().map(|value| *value).collect::<Vec<usize>>();
                keys.sort_by(|a, b| b.cmp(a));


                for key in keys {
                    match ui.elements.get_mut(&key) {
                        None => panic!("Getting a ui element by an invalid id is not possible."),
                        Some(elem) => {
                            if elem.update_mouse(button, action) {}
                        }
                    }
                }
            }
        }
    }

    pub fn update_cursor(cursor: [f64; 2]) {
        unsafe {
            match &mut UI {
                None => panic!("Initialize the UI first."),
                Some(ui) => {
                    let mut keys = ui.elements.keys().into_iter().map(|value| *value).collect::<Vec<usize>>();
                    keys.sort_by(|a, b| b.cmp(a));

                    let force = false;

                    for key in keys {
                        match ui.elements.get_mut(&key) {
                            None => panic!("Getting a ui element by an invalid id is not possible."),
                            Some(elem) => {
                                if elem.update_cursor(cursor, force) {
                                    //force = true;
                                }
                            }
                        }
                    }

                    /*for (_id, elem) in &mut ui.elements {
                        elem.update_cursor(cursor);
                    }*/
                }
            }
        }
    }

    pub fn default_program() -> Program {
        unsafe {
            match &UI {
                None => panic!("Initialize the UI first."),
                Some(ui) => {
                    ui.default_program
                }
            }
        }
    }

    pub fn register<E: UIElement + 'static>(element: E) -> usize {
        unsafe {
            match &mut UI {
                None => {
                    panic!("Initialize the UI first.")
                }
                Some(ui) => {
                    let id = element.id();
                    Self::register_element(element, ui);

                    id
                }
            }
        }
    }

    fn register_boxed_element(mut element: Box<dyn UIElement>, ui: &mut UI) {
        let name = element.name();
        if !name.is_empty() {
            println!("Registering {}", name);
        }
        let id = element.id();
        let mut children = {
            let mut children = vec![];
            while !element.child_buffer().is_empty() {
                let child = element.child_buffer().remove(0);
                children.push(child);
            }
            children
        };

        while !children.is_empty() {
            let mut child = children.remove(0);
            element.register_child(&child);
            child.set_parent(id);
            if ui.elements.contains_key(&child.id()) {
                eprintln!("Cannot have duplicate ids.");
                continue;
            }
            //ui.elements.insert(child.id(), child);
            Self::register_boxed_element(child, ui);
        }
        ui.elements.insert(element.id(), element);
    }

    fn register_element<E: UIElement + 'static>(mut element: E, ui: &mut UI) {
        let id = element.id();
        let mut children = {
            let mut children = vec![];
            while !element.child_buffer().is_empty() {
                let child = element.child_buffer().remove(0);
                children.push(child);
            }
            children
        };

        while !children.is_empty() {
            let mut child = children.remove(0);
            element.register_child(&child);
            child.set_parent(id);
            if ui.elements.contains_key(&child.id()) {
                eprintln!("Cannot have duplicate ids.");
                continue;
            }
            //ui.elements.insert(child.id(), child);
            Self::register_boxed_element(child, ui);
        }
        ui.elements.insert(element.id(), Box::new(element));
    }

    fn generic_vao(buffer: &[f32]) -> VertexArrayObject {
        VertexArrayObject::new(Some(VertexArrayObjectType::ArrayStrips(4)))
            .with_buffer(VertexBufferObject::array(BufferType::ArrayBuffer, DrawType::StaticDraw, buffer))
            .with_local_attrib_pointer(LocalAttribPointer::new(2, BufferDataType::Float, false))
            .with_local_attrib_pointer(LocalAttribPointer::new(2, BufferDataType::Float, false))
            .build()
    }
}

#[derive(Debug)]
pub struct UIRenderData(Program, VertexArrayObject, FillMethod);

pub struct Callbacks {
    on_drag: Vec<Box<OnDrag>>,
}

impl Debug for Callbacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let on_drag = self.on_drag.len();
        f.write_fmt(format_args!("Callbacks: [on_drag: {}]", on_drag))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct UIElementData {
    id: usize,
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
    position: [u32; 2],
    width: u32,
    height: u32,
    tmp_children: Vec<Box<dyn UIElement>>,
    render_data: Option<UIRenderData>,
    hover_flag: bool,
    custom_properties: HashMap<String, CustomUIProperty>,
    corner_radius: f32,
    drag_offset: Option<[f64; 2]>,
    callbacks: Callbacks,
    child_horizontal: Layout,
    child_vertical: Layout,
    spacing: u32,
}

impl UIElementData {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn parent(&self) -> Option<usize> {
        self.parent
    }
    pub fn children(&self) -> &Vec<usize> {
        &self.children
    }
    pub fn get_children(&self) -> Vec<Option<&'static Box<dyn UIElement>>> {
        let mut children = vec![];
        for child in &self.children {
            children.push(UI::get_element_by_id(child));
        }
        children
    }
    pub fn get_children_mut(&self) -> Vec<Option<&'static mut Box<dyn UIElement>>> {
        let mut children = vec![];
        for child in &self.children {
            children.push(UI::get_element_by_id_mut(child));
        }
        children
    }
    pub fn position(&self) -> [u32; 2] {
        self.position
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn tmp_children(&self) -> &Vec<Box<dyn UIElement>> {
        &self.tmp_children
    }
    pub fn render_data(&self) -> &Option<UIRenderData> {
        &self.render_data
    }
    pub fn hover_flag(&self) -> bool {
        self.hover_flag
    }
    pub fn custom_properties(&self) -> &HashMap<String, CustomUIProperty> {
        &self.custom_properties
    }
    pub fn corner_radius(&self) -> f32 {
        self.corner_radius
    }
    pub fn drag_offset(&self) -> Option<[f64; 2]> {
        self.drag_offset
    }
    pub fn callbacks(&self) -> &Callbacks {
        &self.callbacks
    }
}

impl_downcast!(UIElement);

pub trait UIElement where Self: Debug + Downcast {
    fn name(&self) -> &String {
        &self.element_data().name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.element_data_mut().name
    }
    fn tag(&self) -> &'static str;
    fn id(&self) -> usize {
        self.element_data().id
    }
    fn set_parent(&mut self, parent: usize) {
        self.element_data_mut().parent = Some(parent);
    }
    fn parent(&self) -> Option<usize> {
        self.element_data().parent
    }
    fn get_parent(&self) -> Option<&'static Box<dyn UIElement>> {
        return match &self.parent() {
            None => {
                None
            }
            Some(id) => {
                UI::get_element_by_id(id)
            }
        };
    }
    fn get_parent_mut(&self) -> Option<&'static mut Box<dyn UIElement>> {
        return match &self.parent() {
            None => {
                None
            }
            Some(id) => {
                UI::get_element_by_id_mut(id)
            }
        };
    }
    fn children(&self) -> &Vec<usize> {
        &self.element_data().children
    }
    fn register_child(&mut self, child: &Box<dyn UIElement>) {
        self.element_data_mut().children.push(child.id());
    }
    fn is_root(&self) -> bool {
        self.element_data().parent.is_none()
    }
    fn position(&self) -> &[u32; 2] {
        &self.element_data().position
    }
    fn set_position(&mut self, position: [u32; 2]) {
        self.element_data_mut().position = position;
    }
    fn element_data(&self) -> &UIElementData;
    fn element_data_mut(&mut self) -> &mut UIElementData;
    fn resize(&mut self, size: [i32; 2]);
    fn update_cursor(&mut self, cursor: [f64; 2], force: bool) -> bool {
        if force {
            self.element_data_mut().hover_flag = false;
            return true;
        }
        self.element_data_mut().hover_flag = self.contains_point(cursor);

        if let Some(offset) = self.element_data().drag_offset {
            let relative = self.relative_mouse_pos(cursor);
            let me = self.self_mut().unwrap();
            for on_drag in &mut self.element_data_mut().callbacks.on_drag {
                let ui_mouse = Input::ui_cursor();

                on_drag(me, ui_mouse, offset, relative);
            }
        }
        self.element_data().hover_flag
    }
    fn register_on_drag(&mut self, callback: Box<OnDrag>) {
        self.element_data_mut().callbacks.on_drag.push(callback);
    }
    fn update_mouse(&mut self, mouse_button: MouseButton, action: Action) -> bool {
        match action {
            Action::Release => {
                self.element_data_mut().drag_offset = None;
            }
            Action::Press => {
                if self.element_data().hover_flag && mouse_button == MouseButton::Button1 {
                    let mouse = Input::ui_cursor();
                    let rel = self.relative_mouse_pos(mouse);
                    self.element_data_mut().drag_offset = Some(rel);
                    return true;
                }
            }
            _ => {}
        }
        false
    }
    fn relative_mouse_pos(&self, mouse: [f64; 2]) -> [f64; 2] {
        let position = self.element_data().position;
        let position = [position[0] as f64, position[1] as f64];
        [mouse[0] - position[0], mouse[1] - position[1]]
    }
    fn child_buffer(&mut self) -> &mut Vec<Box<dyn UIElement>> {
        &mut self.element_data_mut().tmp_children
    }
    fn add_child(&mut self, child: Box<dyn UIElement>) {
        self.child_buffer().push(child);
    }
    fn contains_point(&self, point: [f64; 2]) -> bool {
        let x = self.position()[0] as f32;
        let y = self.position()[1] as f32;
        let w = self.element_data().width as f32 + x;
        let h = self.element_data().height as f32 + y;
        point[0] as f32 >= x && point[0] as f32 <= w
            &&
            point[1] as f32 >= y && point[1] as f32 <= h
    }
    fn set_custom_property(&mut self, key: &dyn ToString, value: CustomUIProperty) {
        self.element_data_mut().custom_properties.insert(key.to_string(), value);
    }
    fn get_custom_property(&self, key: &dyn ToString) -> Option<&CustomUIProperty> {
        self.element_data().custom_properties.get(&key.to_string())
    }
    fn get_custom_property_mut(&mut self, key: &dyn ToString) -> Option<&mut CustomUIProperty> {
        self.element_data_mut().custom_properties.get_mut(&key.to_string())
    }
    fn self_ref(&self) -> Option<&'static Box<dyn UIElement>> {
        UI::get_element_by_id(&self.id())
    }
    fn self_mut(&self) -> Option<&'static mut Box<dyn UIElement>> {
        UI::get_element_by_id_mut(&self.id())
    }
    fn color(&self) -> Vector4<f32> {
        return match self.element_data().hover_flag {
            true => {
                match self.get_custom_property(&"hover:color") {
                    None => {
                        DEFAULT_COLOR
                    }
                    Some(color) => {
                        match color {
                            CustomUIProperty::Vec4(color) => {
                                *color
                            }
                        }
                    }
                }
            }
            false => {
                match self.get_custom_property(&"color") {
                    None => {
                        DEFAULT_COLOR
                    }
                    Some(color) => {
                        match color {
                            CustomUIProperty::Vec4(color) => *color
                        }
                    }
                }
            }
        };
    }
    fn set_color(&mut self, color: Vector4<f32>) {
        self.set_custom_property(&"color", CustomUIProperty::Vec4(color));
    }
    fn set_fill_method(&mut self, fill_method: FillMethod) {
        match &mut self.element_data_mut().render_data {
            None => eprintln!("No fill method can be set when the render data of a UI element is None."),
            Some(render_data) => render_data.2 = fill_method
        }
    }
    fn get_fill_method_mut(&mut self) -> Option<&mut FillMethod> {
        match &mut self.element_data_mut().render_data {
            None => None,
            Some(render_data) => {
                Some(&mut render_data.2)
            }
        }
    }
    fn update_key(&mut self, _key: Key, _code: Action, _delta: f32) {}
    fn default_update(&mut self, _delta: f32) {
        for child in self.children() {
            let child = UI::get_element_by_id_mut(child).unwrap();
            match self.child_horizontal() {
                Layout::MatchParent(h, v) => {
                    child.set_position([self.position()[0] + h, self.position()[1] + v]);
                }
                Layout::Absolute => {}
            }
            child.update(_delta);
        }
    }
    fn update(&mut self, _delta: f32) {
        self.default_update(_delta);
    }
    fn put_texture(&mut self, texture: Texture) {
        if let Some(render_data) = &mut self.element_data_mut().render_data {
            render_data.1.put_texture::<&str>(None, texture);
        } else {
            eprintln!("Render data not currently set up for this UI element.");
        }
    }
    fn draw_children(&self, camera: &Camera) {
        for child in self.children() {
            let child = UI::get_element_by_id(child).unwrap();
            child.draw(camera);
        }
    }
    fn default_draw(&self, camera: &Camera) {
        if let Some(render_data) = &self.element_data().render_data {
            let projection = camera.last_orthographic();

            let program = render_data.0;
            let vao = &render_data.1;
            program.enable();
            vao.bind();

            let t = self.position();

            let mut model = MatrixWrapper(IDENTITY_MAT4).translated([(t[0] * 2) as f32, (t[1] * 2) as f32, 0f32]);
            let color = self.color();

            model = model.scale([self.element_data().width as f32, self.element_data().height as f32, 1f32]);

            program.set_uniform_mat4("projection", projection);
            program.set_uniform_mat4("model", &model.0);
            program.set_uniform_vec4("color", &color);

            let pos = &self.element_data().position;

            program.set_uniform_vec2("bottomLeft", &[pos[0] as f32, pos[1] as f32]);
            program.set_uniform_vec2("size", &[self.element_data().width as f32, self.element_data().height as f32]);

            let radius_enable = self.element_data().corner_radius.abs() > 0.0;
            program.set_uniform_bool("enableCorner", &radius_enable);
            program.set_uniform_float("cornerRadius", &self.element_data().corner_radius);

            render_data.2.bind(&program, "fillStrategy");

            for i in 0..10 {
                active_texture(i);
                program.set_uniform_bool(format!("texture{}", i), &false);
            }

            for i in 0..vao.textures().len() {
                let texture = vao.textures()[i];
                program.set_uniform_bool(format!("texture{}", i), &true);
                texture.active(i as u32);
                texture.bind();
            }

            if let Some(vao_type) = vao.vao_type() {
                match vao_type {
                    VertexArrayObjectType::Arrays(tris) => {
                        draw_arrays(DrawMode::Triangles, 0, *tris);
                    }
                    VertexArrayObjectType::ArrayStrips(tris) => {
                        draw_arrays(DrawMode::TriangleStrip, 0, *tris);
                    }
                }
            }

            vao.unbind();
            program.disable();
        }
        self.draw_children(camera);
    }
    fn draw(&self, camera: &Camera) {
        self.default_draw(camera);
    }
    fn child_horizontal(&self) -> Layout { self.element_data().child_horizontal }
    fn child_horizontal_mut(&mut self) -> &mut Layout { &mut self.element_data_mut().child_horizontal }
    fn child_vertical(&self) -> Layout { self.element_data().child_vertical }
    fn child_vertical_mut(&mut self) -> &mut Layout { &mut self.element_data_mut().child_vertical }
    fn spacing(&self) -> u32 { self.element_data().spacing }
    fn spacing_mut(&mut self) -> &mut u32 { &mut self.element_data_mut().spacing }
}

pub fn ui_counter() -> usize {
    unsafe {
        let id = COUNTER;
        COUNTER += 1;
        id
    }
}