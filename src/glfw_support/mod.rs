use std::ffi::CStr;
use glfw::ffi::{glfwGetClipboardString, glfwSetClipboardString, GLFWwindow};
use glfw::{Action, Cursor, CursorMode, Modifiers, MouseButton, StandardCursor, Window, WindowEvent};
use imgui::{BackendFlags, ClipboardBackend, ConfigFlags, Context, ImStr, ImString, Io, Key, Ui};
use maplit::hashmap;

pub struct GlfwPlatform {
    hidpi_mode: ActiveHiDpiMode,
    hidpi_factor: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ActiveHiDpiMode {
    Default,
    Rounded,
    Locked,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HiDpiMode {
    /// The DPI factor from glfw is used directly without adjustment
    Default,
    /// The DPI factor from glfw is rounded to an integer value.
    ///
    /// This prevents the user interface from becoming blurry with non-integer scaling.
    Rounded,
    /// The DPI factor from glfw is ignored, and the included value is used instead.
    ///
    /// This is useful if you want to force some DPI factor (e.g. 1.0) and not care about the value
    /// coming from glfw.
    Locked(f64),
}

struct Clipboard {
    window_ptr: *mut GLFWwindow,
}

impl ClipboardBackend for Clipboard {
    fn get(&mut self) -> Option<ImString> {
        unsafe {
            let s = glfwGetClipboardString(self.window_ptr);
            let s = CStr::from_ptr(s);
            let bytes = s.to_bytes();
            if !bytes.is_empty() {
                let v = String::from_utf8_lossy(bytes);
                Some(ImString::new(v))
            } else {
                None
            }
        }
    }

    fn set(&mut self, value: &ImStr) {
        unsafe {
            glfwSetClipboardString(self.window_ptr, value.as_ptr())
        }
    }
}

impl HiDpiMode {
    fn apply(&self, hidpi_factor: f64) -> (ActiveHiDpiMode, f64) {
        match *self {
            HiDpiMode::Default => (ActiveHiDpiMode::Default, hidpi_factor),
            HiDpiMode::Rounded => (ActiveHiDpiMode::Rounded, hidpi_factor.round()),
            HiDpiMode::Locked(value) => (ActiveHiDpiMode::Locked, value),
        }
    }
}

impl GlfwPlatform {
    pub const FLAGS: &'static [BackendFlags] = &[
        BackendFlags::HAS_MOUSE_CURSORS,
        BackendFlags::HAS_SET_MOUSE_POS,
        BackendFlags::HAS_MOUSE_CURSORS,
        BackendFlags::HAS_SET_MOUSE_POS
    ];

    pub fn init(imgui: &mut Context) -> GlfwPlatform {
        let mapping = hashmap! {
            Key::Tab => glfw::Key::Tab,

            Key::LeftArrow => glfw::Key::Left,
            Key::RightArrow => glfw::Key::Right,
            Key::UpArrow => glfw::Key::Up,
            Key::DownArrow => glfw::Key::Down,

            Key::PageUp => glfw::Key::PageUp,
            Key::PageDown => glfw::Key::PageDown,

            Key::Home => glfw::Key::Home,
            Key::End => glfw::Key::End,
            Key::Insert => glfw::Key::Insert,
            Key::Delete => glfw::Key::Delete,

            Key::Backspace => glfw::Key::Backspace,
            Key::Space => glfw::Key::Space,
            Key::Enter => glfw::Key::Enter,
            Key::Escape => glfw::Key::Escape,

            Key::A => glfw::Key::A,
            Key::C => glfw::Key::C,
            Key::V => glfw::Key::V,
            Key::X => glfw::Key::X,
            Key::Y => glfw::Key::Y,
            Key::Z => glfw::Key::Z
        };
        let io = imgui.io_mut();
        for flag in Self::FLAGS {
            io.backend_flags.insert(flag.clone());
        }
        for (from, to) in mapping {
            io[from] = to as _;
        }
        imgui.set_platform_name(Some(ImString::from(format!(
            "imgui-glfw-support {}",
            env!("CARGO_PKG_VERSION")
        ))));
        Self {
            hidpi_mode: ActiveHiDpiMode::Default,
            hidpi_factor: 1.0
        }
    }

    /// Adds platform clipboard integration for the provided window. The caller **must** ensure that
    /// the `Window` outlives the imgui `Context` **and** that any imgui functions that may access
    /// the clipboard are called from the **main thread** (the thread that's executing the event polling).
    pub unsafe fn set_clipboard_backend(&self, imgui: &mut Context, window: &Window) {
        use glfw::Context;
        let window_ptr = window.window_ptr();
        imgui.set_clipboard_backend(Box::new(Clipboard { window_ptr }));
    }

    pub fn attach_window(&mut self, io: &mut Io, window: &Window, hidpi_mode: HiDpiMode) {
        let (scale_factor_x, _scale_factor_y) = window.get_content_scale();
        let (hidpi_mode, hidpi_factor) = hidpi_mode.apply(scale_factor_x as _);
        self.hidpi_mode = hidpi_mode;
        self.hidpi_factor = hidpi_factor;
        io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
        let (width, height) = window.get_size();
        io.display_size = [width as f32, height as f32];
    }

    pub fn handle_event(&self, io: &mut Io, _window: &Window, event: &WindowEvent) {
        match *event {
            WindowEvent::Key(key, _scancode, action, modifiers) => {
                if key as i32 >= 0 {
                    if action == Action::Release {
                        io.keys_down[key as usize] = false;
                    } else {
                        io.keys_down[key as usize] = true;
                    }
                }
                io.key_shift = modifiers.contains(Modifiers::Shift);
                io.key_ctrl = modifiers.contains(Modifiers::Control);
                io.key_alt = modifiers.contains(Modifiers::Alt);
                io.key_super = modifiers.contains(Modifiers::Super);
            }
            WindowEvent::Size(width, height) => {
                io.display_size = [width as _, height as _];
            }
            WindowEvent::Char(ch) => {
                // Exclude the backspace key
                if ch != '\u{7f}' {
                    io.add_input_character(ch);
                }
            }
            WindowEvent::CursorPos(x, y) => {
                io.mouse_pos = [x as _, y as _];
            }
            WindowEvent::Scroll(x, y) => {
                io.mouse_wheel_h = x as _;
                io.mouse_wheel = y as _;
            }
            WindowEvent::MouseButton(button, action, _modifiers) => {
                let pressed = action == Action::Press;
                match button {
                    MouseButton::Button1 => io.mouse_down[0] = pressed,
                    MouseButton::Button2 => io.mouse_down[1] = pressed,
                    MouseButton::Button3 => io.mouse_down[2] = pressed,
                    _ => (),
                }
            }
            _ => {}
        }
    }

    pub fn prepare_frame(&self, io: &mut Io, window: &mut Window) -> Result<(), String> {
        if io.want_set_mouse_pos {
            let [x, y] = io.mouse_pos;
            window.set_cursor_pos(x as _, y as _);
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn prepare_render(&self, ui: &Ui, window: &mut Window) {
        let io = ui.io();
        if !io
            .config_flags
            .contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
        {
            match ui.mouse_cursor() {
                Some(mouse_cursor) if !io.mouse_draw_cursor => {
                    window.set_cursor_mode(CursorMode::Normal);
                    window.set_cursor(Some(match mouse_cursor {
                        // TODO: GLFW has more cursor options on master, but they aren't released yet
                        imgui::MouseCursor::Arrow => Cursor::standard(StandardCursor::Arrow),
                        imgui::MouseCursor::ResizeAll => Cursor::standard(StandardCursor::Arrow),
                        imgui::MouseCursor::ResizeNS => Cursor::standard(StandardCursor::VResize),
                        imgui::MouseCursor::ResizeEW => Cursor::standard(StandardCursor::HResize),
                        imgui::MouseCursor::ResizeNESW => Cursor::standard(StandardCursor::Arrow),
                        imgui::MouseCursor::ResizeNWSE => Cursor::standard(StandardCursor::Arrow),
                        imgui::MouseCursor::Hand => Cursor::standard(StandardCursor::Hand),
                        imgui::MouseCursor::TextInput => Cursor::standard(StandardCursor::IBeam),
                    }));
                }
                _ => window.set_cursor_mode(CursorMode::Hidden),
            }
        }
    }
}