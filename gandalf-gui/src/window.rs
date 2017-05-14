
extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;

pub trait View {
    fn view(&mut self, &mut imgui::Ui);
}

pub struct Window<V: View> {
    display: glium::Display,
    imgui: imgui::ImGui,
    renderer: imgui_glium_renderer::Renderer,
    last_frame: ::std::time::Instant,
    mouse_pressed: (bool, bool, bool),
    mouse_pos: (i32, i32),
    mouse_wheel: f32,
    view: V,
}

impl<V: View> Window<V> {
    pub fn new(title: &str, size: Option<(u32, u32)>, view: V) -> Self {
        use std::time::Instant;
        use self::glium::{glutin, DisplayBuild};
        use self::imgui::{ImGui, ImGuiKey};
        use self::imgui_glium_renderer::Renderer;

        let mut builder = glutin::WindowBuilder::new().with_title(title);

        if let Some((w, h)) = size {
            builder = builder.with_dimensions(w, h);
        }

        let display = builder.build_glium().unwrap();

        let mut imgui = ImGui::init();
        let renderer = Renderer::init(&mut imgui, &display).unwrap();

        imgui.set_imgui_key(ImGuiKey::Tab, 0);
        imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
        imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
        imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
        imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
        imgui.set_imgui_key(ImGuiKey::PageUp, 5);
        imgui.set_imgui_key(ImGuiKey::PageDown, 6);
        imgui.set_imgui_key(ImGuiKey::Home, 7);
        imgui.set_imgui_key(ImGuiKey::End, 8);
        imgui.set_imgui_key(ImGuiKey::Delete, 9);
        imgui.set_imgui_key(ImGuiKey::Backspace, 10);
        imgui.set_imgui_key(ImGuiKey::Enter, 11);
        imgui.set_imgui_key(ImGuiKey::Escape, 12);
        imgui.set_imgui_key(ImGuiKey::A, 13);
        imgui.set_imgui_key(ImGuiKey::C, 14);
        imgui.set_imgui_key(ImGuiKey::V, 15);
        imgui.set_imgui_key(ImGuiKey::X, 16);
        imgui.set_imgui_key(ImGuiKey::Y, 17);
        imgui.set_imgui_key(ImGuiKey::Z, 18);

        Self {
            display: display,
            imgui: imgui,
            renderer: renderer,
            last_frame: Instant::now(),
            mouse_pressed: (false, false, false),
            mouse_pos: (0, 0),
            mouse_wheel: 0.0,
            view: view,
        }
    }

    fn update_events(&mut self) -> bool {
        use self::glium::glutin::{ElementState, Event, MouseButton, MouseScrollDelta,
                                  VirtualKeyCode, TouchPhase};

        for event in self.display.poll_events() {
            match event {
                Event::Closed => return false,
                Event::KeyboardInput(state, _, code) => {
                    let pressed = state == ElementState::Pressed;
                    match code {
                        Some(VirtualKeyCode::Tab) => self.imgui.set_key(0, pressed),
                        Some(VirtualKeyCode::Left) => self.imgui.set_key(1, pressed),
                        Some(VirtualKeyCode::Right) => self.imgui.set_key(2, pressed),
                        Some(VirtualKeyCode::Up) => self.imgui.set_key(3, pressed),
                        Some(VirtualKeyCode::Down) => self.imgui.set_key(4, pressed),
                        Some(VirtualKeyCode::PageUp) => self.imgui.set_key(5, pressed),
                        Some(VirtualKeyCode::PageDown) => self.imgui.set_key(6, pressed),
                        Some(VirtualKeyCode::Home) => self.imgui.set_key(7, pressed),
                        Some(VirtualKeyCode::End) => self.imgui.set_key(8, pressed),
                        Some(VirtualKeyCode::Delete) => self.imgui.set_key(9, pressed),
                        Some(VirtualKeyCode::Back) => self.imgui.set_key(10, pressed),
                        Some(VirtualKeyCode::Return) => self.imgui.set_key(11, pressed),
                        Some(VirtualKeyCode::Escape) => self.imgui.set_key(12, pressed),
                        Some(VirtualKeyCode::A) => self.imgui.set_key(13, pressed),
                        Some(VirtualKeyCode::C) => self.imgui.set_key(14, pressed),
                        Some(VirtualKeyCode::V) => self.imgui.set_key(15, pressed),
                        Some(VirtualKeyCode::X) => self.imgui.set_key(16, pressed),
                        Some(VirtualKeyCode::Y) => self.imgui.set_key(17, pressed),
                        Some(VirtualKeyCode::Z) => self.imgui.set_key(18, pressed),
                        Some(VirtualKeyCode::LControl) |
                        Some(VirtualKeyCode::RControl) => self.imgui.set_key_ctrl(pressed),
                        Some(VirtualKeyCode::LShift) |
                        Some(VirtualKeyCode::RShift) => self.imgui.set_key_shift(pressed),
                        Some(VirtualKeyCode::LAlt) |
                        Some(VirtualKeyCode::RAlt) => self.imgui.set_key_alt(pressed),
                        Some(VirtualKeyCode::LWin) |
                        Some(VirtualKeyCode::RWin) => self.imgui.set_key_super(pressed),
                        _ => {}
                    }
                }
                Event::MouseMoved(x, y) => self.mouse_pos = (x, y),
                Event::MouseInput(state, MouseButton::Left) => {
                    self.mouse_pressed.0 = state == ElementState::Pressed
                }
                Event::MouseInput(state, MouseButton::Right) => {
                    self.mouse_pressed.1 = state == ElementState::Pressed
                }
                Event::MouseInput(state, MouseButton::Middle) => {
                    self.mouse_pressed.2 = state == ElementState::Pressed
                }
                Event::MouseWheel(MouseScrollDelta::LineDelta(_, y), TouchPhase::Moved) |
                Event::MouseWheel(MouseScrollDelta::PixelDelta(_, y), TouchPhase::Moved) => {
                    self.mouse_wheel = y
                }
                Event::ReceivedCharacter(c) => self.imgui.add_input_character(c),
                _ => (),
            }
        }

        true
    }

    pub fn update(&mut self, clear_color: (f32, f32, f32, f32)) -> bool {
        use std::time::Instant;
        use self::glium::Surface;

        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;

        if !self.update_events() {
            return false;
        }

        let scale = self.imgui.display_framebuffer_scale();
        self.imgui
            .set_mouse_pos(self.mouse_pos.0 as f32 / scale.0,
                           self.mouse_pos.1 as f32 / scale.1);
        self.imgui
            .set_mouse_down(&[self.mouse_pressed.0,
                              self.mouse_pressed.1,
                              self.mouse_pressed.2,
                              false,
                              false]);
        self.imgui.set_mouse_wheel(self.mouse_wheel / scale.1);
        self.mouse_wheel = 0.0;


        let mut target = self.display.draw();
        target.clear(None, Some(clear_color), false, None, None);

        let window = self.display.get_window().unwrap();
        let size_points = window.get_inner_size_points().unwrap();
        let size_pixels = window.get_inner_size_pixels().unwrap();

        let mut ui = self.imgui.frame(size_points, size_pixels, delta_s);

        self.view.view(&mut ui);

        self.renderer.render(&mut target, ui).unwrap();

        target.finish().unwrap();

        true
    }
}
