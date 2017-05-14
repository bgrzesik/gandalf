
#[cfg(build = "release")]
#[cfg(target_os = "windows")]
extern crate kernel32;

#[macro_use]
extern crate imgui;
extern crate gandalf;
extern crate bincode;

mod window;

struct GandalfCtl {
    cmd: imgui::ImString,
    url: imgui::ImString,
    yt: imgui::ImString,
    yte: imgui::ImString,
    ip: [i32; 4],
}

impl GandalfCtl {
    fn new() -> Self {
        use imgui::ImString;
        Self {
            cmd: ImString::with_capacity(32),
            url: ImString::with_capacity(32),
            yt: ImString::with_capacity(32),
            yte: ImString::with_capacity(32),
            ip: [237, 77, 7, 77],
        }
    }
}

impl window::View for GandalfCtl {
    fn view(&mut self, ui: &mut imgui::Ui) {
        use gandalf::IncommingMessage;
        use imgui::{ImGuiSetCond_FirstUseEver, ImVec2};

        ui.window(im_str!("Gandalf Control"))
            .size((500.0, 150.0), ImGuiSetCond_FirstUseEver)
            .build(|| {
                ui.input_int4(im_str!("IP"), &mut self.ip).build();
                ui.separator();

                let mut a = ui.input_text(im_str!("Command"), &mut self.cmd)
                    .enter_returns_true(true)
                    .build();

                ui.same_line(0.0);
                a |= ui.button(im_str!("Run"), ImVec2::new(100.0, 20.0));
                if a {
                    send_msg(IncommingMessage::Summon {
                                 what: format!("cmd /C {:?}", self.cmd.to_str()),
                             });
                }

                ui.separator();

                let mut a = ui.input_text(im_str!("URL"), &mut self.url)
                    .enter_returns_true(true)
                    .build();


                ui.same_line(0.0);
                a |= ui.button(im_str!("Open URL"), ImVec2::new(100.0, 20.0));
                if a {
                    send_msg(IncommingMessage::Url(self.url.to_str().to_owned()));
                }

                let mut a = ui.input_text(im_str!("YT"), &mut self.yt)
                    .enter_returns_true(true)
                    .build();

                ui.same_line(0.0);
                a |= ui.button(im_str!("Open YT"), ImVec2::new(100.0, 20.0));
                if a {
                    send_msg(IncommingMessage::Yt { vid: self.yt.to_str().to_owned() });
                }


                let mut a = ui.input_text(im_str!("YTE"), &mut self.yte)
                    .enter_returns_true(true)
                    .build();

                ui.same_line(0.0);
                a |= ui.button(im_str!("Open YTE"), ImVec2::new(100.0, 20.0));
                if a {
                    let url = format!("https://www.youtube.com/embed/{}?rel=0&autoplay=1",
                                      self.yte.to_str());
                    send_msg(IncommingMessage::Url(url))
                }

                if ui.button(im_str!("Blank"), ImVec2::new(100.0, 20.0)) {
                    send_msg(IncommingMessage::Url("about:blank".to_owned()));
                }

                ui.separator();
                if ui.button(im_str!("Gandalf"), ImVec2::new(100.0, 20.0)) {
                    send_msg(IncommingMessage::Gandalf);
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Hide"), ImVec2::new(100.0, 20.0)) {
                    send_msg(IncommingMessage::Disappear);
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Turn off"), ImVec2::new(100.0, 20.0)) {
                    ui.open_popup(im_str!("Turn off?"));
                }

                ui.popup(im_str!("Turn off?"), || {
                    ui.text(im_str!("Are you sure?"));
                    if ui.button(im_str!("Yes"), ImVec2::new(100.0, 20.0)) {
                        ui.close_current_popup();
                        send_msg(IncommingMessage::Retreat);
                    }
                    ui.same_line(0.0);
                    if ui.button(im_str!("No"), ImVec2::new(100.0, 20.0)) {
                        ui.close_current_popup();
                    }
                });
            });
    }
}

fn send_msg(msg: gandalf::IncommingMessage) {
    use std::net::UdpSocket;
    use bincode::{Infinite, serialize};

    let send_socket = ("237.77.7.77", 23441);

    let buf = serialize(&msg, Infinite).expect("Unable to serialize message");
    let socket = UdpSocket::bind(("0.0.0.0", 23440)).expect("Unable to bind socket 0.0.0.0:23441");
    socket
        .set_broadcast(true)
        .expect("Unable to turn off broadcast mode");
    socket
        .send_to(&buf[..], send_socket)
        .expect("Unable to send message");

}


fn main() {
    use window::Window;

    #[cfg(build = "release")]
    #[cfg(target_os = "windows")]
    unsafe {
        kernel32::FreeConsole();
    }

    let mut view = Window::new("Gandalf Control", None, GandalfCtl::new());
    while view.update((114.0 / 255.0, 144.0 / 255.0, 154.0 / 255.0, 1.0)) {}
}
