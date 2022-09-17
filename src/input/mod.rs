use imgui_glfw_rs::glfw::{Action, Key, MouseButton};

static mut INPUT: Option<Input> = None;

#[derive(Debug)]
pub struct Mouse {
    pressed: Vec<MouseButton>,
    released: Vec<MouseButton>,
    held: Vec<MouseButton>,
    mouse: [f64; 2],
    ui_mouse: [f64; 2]
}

#[derive(Debug)]
pub struct Keys {
    pressed: Vec<Key>,
    released: Vec<Key>,
    held: Vec<Key>
}

impl Keys {
    pub fn update(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }
}

#[derive(Debug)]
pub struct Input {
    keys: Keys,
    mouse: Mouse
}

impl Input {
    pub fn init() {
        unsafe {
            INPUT = Some(Input {
                keys: Keys {
                    pressed: vec![],
                    released: vec![],
                    held: vec![]
                },
                mouse: Mouse {
                    pressed: vec![],
                    released: vec![],
                    held: vec![],
                    mouse: [0.; 2],
                    ui_mouse: [0.; 2]
                }
            });
        }
    }

    pub fn update() {
        unsafe {
            match &mut INPUT {
                None => {
                    panic!("Input not initialized.");
                }
                Some(input) => {
                    input.keys.update();
                }
            }
        }
    }

    pub fn is_key_pressed(key: Key) -> bool {
        unsafe {
            match &INPUT {
                None => {
                    panic!("Input not initialized.");
                }
                Some(input) => {
                    input.keys.pressed.contains(&key)
                }
            }
        }
    }

    pub fn is_key_released(key: Key) -> bool {
        unsafe {
            match &INPUT {
                None => {
                    panic!("Input not initialized.");
                }
                Some(input) => {
                    input.keys.released.contains(&key)
                }
            }
        }
    }

    pub fn is_key_held(key: Key) -> bool {
        unsafe {
            match &INPUT {
                None => {
                    panic!("Input not initialized.");
                }
                Some(input) => {
                    input.keys.held.contains(&key)
                }
            }
        }
    }

    pub fn map_held(neg: Key, pos: Key) -> f32 {
        let mut out = 0.0;
        if Self::is_key_held(neg) {
            out -= 1.0;
        }
        if Self::is_key_held(pos) {
            out += 1.0;
        }
        out
    }

    pub fn ui_cursor() -> [f64; 2] {
        unsafe {
            match &INPUT {
                None => {
                    panic!("Input not initialized.");
                }
                Some(input) => {
                    input.mouse.ui_mouse
                }
            }
        }
    }

    pub fn record_cursor(mouse: [f64; 2], ui_cursor: [f64; 2]) {
        unsafe {
            match &mut INPUT {
                None => {
                    panic!("Input not initialized.");
                }
                Some(input) => {
                    input.mouse.mouse = mouse;
                    input.mouse.ui_mouse = ui_cursor;
                }
            }
        }
    }

    pub fn record_mouse_button(mouse_button: MouseButton, action: Action) {
        unsafe {
            match &mut INPUT {
                None => {}
                Some(input) => {
                    match action {
                        Action::Release => {
                            input.mouse.released.push(mouse_button);
                            for i in 0..input.mouse.held.len() {
                                if mouse_button == input.mouse.held[i] {
                                    input.mouse.held.remove(i);
                                    break;
                                }
                            }
                        }
                        Action::Press => {
                            input.mouse.pressed.push(mouse_button);
                            input.mouse.held.push(mouse_button);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn record_keystroke(key: Key, code: Action) {
        unsafe {
            match &mut INPUT {
                None => {
                    panic!("Input not initialized.")
                }
                Some(input) => {
                    let held = &mut input.keys.held;
                    match code {
                        Action::Release => {
                            input.keys.released.push(key.clone());
                            for i in 0..held.len() {
                                if held[i] == key {
                                    held.remove(i);
                                    break;
                                }
                            }
                        }
                        Action::Press => {
                            input.keys.pressed.push(key.clone());
                            input.keys.held.push(key);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}