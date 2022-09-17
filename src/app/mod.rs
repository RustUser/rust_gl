use std::borrow::BorrowMut;
use imgui_glfw_rs::{imgui, ImguiGLFW};


use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use imgui_glfw_rs::glfw::{Action, Context, FAIL_ON_ERRORS, flush_messages, Glfw, Key, Modifiers, MouseButton, OpenGlProfileHint, Scancode, Window, WindowEvent, WindowHint, WindowMode};
use serde::*;
use crate::{Camera, clear, clear_color, cull_face, depth_mask, disable, enable, GLConsts, Input, Ordering, TextureLibraryFlag, UI, viewport, Face, front_face, depth_func, ColorFlags, FragmentShader, VertexShader, Program, OBJ};
use enum_iterator::{all, Sequence};
use imgui_glfw_rs::imgui::Ui;
use rlua::{Lua, UserData, UserDataMethods};
use crate::GLConsts::{CullFace, DepthTest};
use crate::gfx::consts::color::Color;
use crate::math::linear_algebra::types::Vec4;

pub type EventHandler = Receiver<(f64, WindowEvent)>;

#[derive(Debug, Clone)]
pub enum AppError {}

pub static mut WINDOW_SIZE: [u32; 2] = [512; 2];

static mut IMGUI: Option<Arc<Mutex<imgui::Context>>> = None;
static mut IMGUI_GLFW: Option<Arc<Mutex<ImguiGLFW>>> = None;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Sequence)]
#[repr(u8)]
pub enum AppConfigFlags {
    ///OpenGL 3.3
    OpenGLPrimary = 1,
    ///OpenGL 4.5
    OpenGLAdvanced = 2,
    ///OpenGL Core Profile
    OpenGLCore = 4,
}

impl AppConfigFlags {
    fn configure(init: &mut Glfw, flags: u8) {
        let config_flags = all::<AppConfigFlags>();

        let mut index = 0;
        for flag in config_flags {
            if (flags >> index & 1) != 0 {
                match flag {
                    AppConfigFlags::OpenGLPrimary => {
                        init.window_hint(WindowHint::ContextVersionMajor(3));
                        init.window_hint(WindowHint::ContextVersionMinor(3));
                    }
                    AppConfigFlags::OpenGLAdvanced => {
                        init.window_hint(WindowHint::ContextVersionMajor(4));
                        init.window_hint(WindowHint::ContextVersionMinor(5));
                    }
                    AppConfigFlags::OpenGLCore => {
                        init.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
                    }
                }
            }
            index += 1;
        }
    }
}

pub struct App<A: Application> {
    glfw: Glfw,
    window: Window,
    events: EventHandler,
    app: A,
    context: GLContext,
    imgui: Arc<Mutex<imgui::Context>>,
    imgui_glfw: Arc<Mutex<ImguiGLFW>>,
}

#[derive(Debug, Clone)]
pub struct Clear {
    pub clear_flags: u32,
    pub clear_color: [f32; 4],
}

impl Clear {
    pub fn set_color(&mut self, color: Color) {
        self.clear_color = color.into();
    }
}

#[derive(Debug, Clone)]
pub struct FaceCulling {
    pub enabled: bool,
    pub face: Face,
    pub front_face: Ordering,
}

#[derive(Debug, Clone)]
pub struct Depth {
    pub enabled: bool,
    pub depth_func: GLConsts,
}

#[derive(Debug, Clone)]
pub struct GLContext {
    pub viewport: [i32; 4],
    pub clear: Clear,
    pub depth_testing: Depth,
    pub cull_face: FaceCulling,
    pub depth_mask: bool,
}

impl UserData for GLContext {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(_methods: &mut T) {
        _methods.add_method_mut("clear_color_set", |_, m, color: Vec4| {
            m.clear.clear_color = color;
            Ok(())
        });

        _methods.add_method("clear_color", |_, m, _: ()| {
            m.clear_color(m.clear.clear_color);
            Ok(())
        });

        _methods.add_method("clear", |_, m, _: ()| {
            m.clear(m.clear.clear_flags);
            Ok(())
        });

        _methods.add_method("viewport", |_, me, _: ()| {
            me.viewport([me.viewport[0], me.viewport[1]], [me.viewport[2], me.viewport[3]]);
            Ok(())
        });

        _methods.add_method("gl_clear", |_, m, flags: u32| {
            m.clear(flags);
            Ok(())
        });

        _methods.add_method("gl_viewport", |_, m, viewport: [i32; 4]| {
            m.viewport([viewport[0], viewport[1]], [viewport[2], viewport[3]]);
            Ok(())
        });

        _methods.add_method("gl_enable", |_, m, con: GLConsts| {
            m.enable(con);
            Ok(())
        });

        _methods.add_method("gl_disable", |_, m, con: GLConsts| {
            m.disable(con);
            Ok(())
        });

        _methods.add_method("gl_depth_mask", |_, m, mask: bool| {
            m.depth_mask(mask);
            Ok(())
        });

        _methods.add_method("gl_cull_face", |_, m, face: Face| {
            m.cull_face(face);
            Ok(())
        });

        _methods.add_method("gl_front_face", |_, m, ordering: Ordering| {
            m.front_face(ordering);
            Ok(())
        });

        _methods.add_method("gl_depth_func", |_, m, depth_func: GLConsts| {
            m.depth_func(depth_func);
            Ok(())
        });
    }
}

impl GLContext {
    pub fn translate(&mut self, pos: [i32; 2]) {
        self.viewport[0] = pos[0] as i32;
        self.viewport[1] = pos[1] as i32;
    }
    pub fn resize(&mut self, size: [u32; 2]) {
        self.viewport[2] = size[0] as i32;
        self.viewport[3] = size[1] as i32;
    }

    pub fn prepare_render_lua(&mut self, lua_file: &String, camera: &mut Camera) {
        let lua = Lua::new();
        lua.context(|ctx| {
            let globals = ctx.globals();
            GLConsts::load_lua(&ctx);
            globals.set("context", self.clone());
            globals.set("camera", camera.clone());
            ctx.load(lua_file.as_str()).exec().unwrap();
            *self = globals.get("context").unwrap();
            *camera = globals.get("camera").unwrap();
        });
    }

    pub fn prepare_render(&self) {
        self.viewport([self.viewport[0], self.viewport[1]], [self.viewport[2], self.viewport[3]]);
        match self.cull_face.enabled {
            true => self.enable(CullFace),
            false => self.disable(CullFace)
        }
        match self.depth_testing.enabled {
            true => self.enable(DepthTest),
            false => self.disable(DepthTest)
        }
        self.depth_mask(self.depth_mask);

        self.clear_color(self.clear.clear_color);
        self.clear(self.clear.clear_flags);

        self.cull_face(self.cull_face.face);
        self.front_face(self.cull_face.front_face);
        self.depth_func(self.depth_testing.depth_func);
    }

    pub fn enable(&self, constant: GLConsts) {
        enable(constant);
    }

    pub fn disable(&self, constant: GLConsts) {
        disable(constant);
    }

    pub fn viewport(&self, position: [i32; 2], size: [i32; 2]) {
        viewport(position, size)
    }

    pub fn clear_color(&self, color: [f32; 4]) {
        clear_color(color);
    }

    pub fn clear(&self, flags: u32) {
        clear(flags);
    }

    pub fn cull_face(&self, face: Face) {
        cull_face(face);
    }

    pub fn front_face(&self, ordering: Ordering) {
        front_face(ordering);
    }

    pub fn depth_func(&self, func: GLConsts) {
        depth_func(func);
    }

    pub fn depth_mask(&self, mask: bool) {
        depth_mask(mask);
    }
}

pub trait Application {
    ///This function, by default, calls the super_init function.
    fn init(&mut self, context: &mut GLContext) {
        self.super_init(context);
    }
    ///This function, by default, calls the super_handle_event function.
    fn handle_event(&mut self, window: &mut Window, context: &mut GLContext, event: WindowEvent) {
        self.super_handle_event(window, context, &event);
    }
    fn update(&mut self, delta: f32) {
        self.super_update(delta);
    }
    fn late_update(&mut self, delta: f32) {
        self.super_late_update(delta);
    }
    fn render(&mut self, context: &mut GLContext, delta: f32) {
        context.prepare_render();
        self.super_render(context, delta);
        self.render_ui(context, delta);
    }
    fn super_update(&mut self, delta: f32) {
        UI::update(delta);
    }
    fn super_late_update(&mut self, delta: f32) {}
    fn super_render(&mut self, _context: &mut GLContext, _delta: f32) {}
    fn render_ui(&mut self, context: &GLContext, _delta: f32) {
        if let Some(camera) = self.camera() {
            context.depth_mask(false);
            context.disable(DepthTest);
            UI::draw(camera);
        }
    }
    fn on_pos(&self, _window: &mut Window, _x: &i32, _y: &i32) {}
    fn on_size(&self, _window: &mut Window, _width: &i32, _height: &i32) {}
    fn on_close(&self, _window: &mut Window) {}
    fn on_refresh(&self, _window: &mut Window) {}
    fn on_focus(&self, _window: &mut Window, _focus: &bool) {}
    fn on_iconify(&self, _window: &mut Window, _iconify: &bool) {}
    fn on_frame_buffer_size(&self, _window: &mut Window, _width: &i32, _height: &i32) {}
    fn on_mouse_button(&self, _window: &mut Window, _button: &MouseButton, _action: &Action, _modifiers: &Modifiers) {}
    fn on_cursor_pos(&self, _window: &mut Window, _x: &f64, _y: &f64) {}
    fn on_cursor_enter(&self, _window: &mut Window, _enter: &bool) {}
    fn on_scroll(&self, _window: &mut Window, _x: &f64, _y: &f64) {}
    fn on_key(&mut self, _window: &mut Window, _key: &Key, _scan_code: &Scancode, _action: &Action, _modifiers: &Modifiers) {}
    fn on_content_scale(&self, _window: &mut Window, _width: &f32, _height: &f32) {}
    fn on_maximize(&self, _window: &mut Window, _maximize: &bool) {}
    fn on_file_drop(&self, _window: &mut Window, _file_drop: &Vec<PathBuf>) {}
    fn on_char(&self, _window: &mut Window, _c: &char) {}
    fn on_char_modifiers(&self, _window: &mut Window, _c: &char, _m: &Modifiers) {}
    ///Initializes the Input and UI systems. Without these calls, the Input and UI systems will not work. Will cause errors and crash.
    fn super_init(&self, _context: &mut GLContext) {
        unsafe {
            Input::init();
            UI::init();
        }
    }
    ///
    fn super_handle_event(&mut self, window: &mut Window, context: &mut GLContext, event: &WindowEvent) {
        unsafe {
            match event {
                WindowEvent::Pos(x, y) => self.on_pos(window, x, y),
                WindowEvent::Size(width, height) => {
                    let w = width;
                    let h = height;
                    let (width, height) = (*width, *height);
                    WINDOW_SIZE = [width as u32, height as u32];
                    if let Some(camera) = self.camera_mut() {
                        camera.update_aspect_ratio(WINDOW_SIZE);
                    }
                    UI::update_screen_size(WINDOW_SIZE);
                    self.on_size(window, w, h);
                    context.resize(WINDOW_SIZE);
                }
                WindowEvent::Close => self.on_close(window),
                WindowEvent::Refresh => self.on_refresh(window),
                WindowEvent::Focus(focus) => self.on_focus(window, focus),
                WindowEvent::Iconify(iconify) => self.on_iconify(window, iconify),
                WindowEvent::FramebufferSize(width, height) => self.on_frame_buffer_size(window, width, height),
                WindowEvent::MouseButton(button, action, modifiers) => {
                    Input::record_mouse_button(*button, *action);
                    UI::update_mouse(*button, *action);
                    self.on_mouse_button(window, button, action, modifiers);
                }
                WindowEvent::CursorPos(x, y) => {
                    let cursor = [*x, *y];
                    let ui_cursor = [cursor[0], WINDOW_SIZE[1] as f64 - cursor[1]];
                    UI::update_cursor(ui_cursor);
                    Input::record_cursor(cursor, ui_cursor);
                    self.on_cursor_pos(window, x, y);
                }
                WindowEvent::CursorEnter(enter) => self.on_cursor_enter(window, enter),
                WindowEvent::Scroll(x, y) => self.on_scroll(window, x, y),
                WindowEvent::Key(key, scan_code, action, modifiers) => {
                    Input::record_keystroke(*key, *action);
                    self.on_key(window, key, scan_code, action, modifiers);
                }
                WindowEvent::Char(c) => self.on_char(window, c),
                WindowEvent::CharModifiers(c, d) => self.on_char_modifiers(window, c, d),
                WindowEvent::FileDrop(file_drop) => self.on_file_drop(window, file_drop),
                WindowEvent::Maximize(max) => self.on_maximize(window, max),
                WindowEvent::ContentScale(width, height) => self.on_content_scale(window, width, height),
            }
        }
    }
    fn texture_flags(&self) -> Option<&TextureLibraryFlag>;
    fn camera(&self) -> Option<&Camera>;
    fn camera_mut(&mut self) -> Option<&mut Camera>;

    fn create_program(&self, fragment_shader: FragmentShader, vertex_shader: VertexShader) -> Result<Program, Box<dyn Error>> {
        Program::new(fragment_shader, vertex_shader)
    }

    fn load_obj_by_path<P: AsRef<Path>>(&self, path: P) -> Result<OBJ, Box<dyn Error>> {
        OBJ::from_file(path)
    }

    fn imgui<'a>(&'a mut self, window: &mut Window, ctx: &'a mut imgui::Context, imgui_glfw: &mut ImguiGLFW) -> Ui<'a> {
        imgui_glfw.frame(window, ctx)
    }
}

impl<A: Application> App<A> {
    pub fn new(title: &str, window_size: [u32; 2], window_mode: WindowMode, flags: u8, mut app: A) -> Result<Self, Box<dyn Error>> {
        let mut glfw = imgui_glfw_rs::glfw::init(FAIL_ON_ERRORS)?;

        let (mut window, events) = glfw.create_window(window_size[0], window_size[1], title, window_mode).unwrap();
        window.make_current();
        window.set_all_polling(true);

        AppConfigFlags::configure(&mut glfw, flags);

        gl::load_with(|address| {
            window.get_proc_address(address)
        });

        let mut context = GLContext {
            viewport: [0, 0, window_size[0] as i32, window_size[1] as i32],
            clear: Clear { clear_flags: ColorFlags::DepthBufferBit | ColorFlags::ColorBufferBit, clear_color: [0.0; 4] },
            depth_testing: Depth { enabled: true, depth_func: GLConsts::Less },
            cull_face: FaceCulling {
                enabled: true,
                face: Face::Back,
                front_face: Ordering::ClockWise,
            },
            depth_mask: false,
        };

        let mut imgui = imgui::Context::create();
        let mut imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);

        let imgui_arc = Arc::new(Mutex::new(imgui));
        let imgui_glfw_arc = Arc::new(Mutex::new(imgui_glfw));

        app.init(&mut context);
        Ok(Self {
            glfw,
            window,
            events,
            app,
            context,
            imgui: imgui_arc,
            imgui_glfw: imgui_glfw_arc,
        })
    }
    pub fn run(&mut self) {
        let mut last_update_time = 0f64;
        while !self.window.should_close() {
            let now = self.window.glfw.get_time();
            let delta_time = (now - last_update_time) as f32;
            self.glfw.poll_events();
            self.app.update(delta_time);
            self.app.late_update(delta_time);
            self.app.render(&mut self.context, delta_time);

            let imgui = self.imgui.clone();
            let mut imgui = imgui.lock().unwrap();
            let imgui_glfw = self.imgui_glfw.clone();
            let mut imgui_glfw = imgui_glfw.lock().unwrap();

            let ui = self.app.imgui(&mut self.window, imgui.borrow_mut(), imgui_glfw.borrow_mut());

            imgui_glfw.draw(ui, &mut self.window);

            self.window.swap_buffers();
            for (_, event) in flush_messages(&self.events) {
                imgui_glfw.handle_event(imgui.borrow_mut(), &event);
                self.app.handle_event(&mut self.window, &mut self.context, event);
            }
            Input::update();
            last_update_time = now;
        }
    }
    pub fn glfw(&self) -> &Glfw {
        &self.glfw
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
    pub fn events(&self) -> &EventHandler {
        &self.events
    }
    pub fn app(&self) -> &A {
        &self.app
    }
    pub fn context(&self) -> &GLContext {
        &self.context
    }
    pub fn context_mut(&mut self) -> &mut GLContext { &mut self.context }
    pub fn set_title<T: ToString>(&mut self, title: T) {
        self.window.set_title(title.to_string().as_str());
    }
}