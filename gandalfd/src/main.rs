extern crate gandalf;
extern crate bincode;

extern crate pnet;

#[cfg(build = "release")]
#[cfg(target_os = "windows")]
extern crate kernel32;

extern crate serde_json;

mod process;

mod browser;

const BIND_SOCKET: (&'static str, u16) = ("0.0.0.0", 23441);

pub struct Summoner {
    child: Option<process::Process>,
    browser: browser::Browser,
}

impl Summoner {
    pub fn new() -> Self {
        Summoner {
            child: None,
            browser: browser::Browser::new(),
        }
    }

    pub fn disappear(&mut self) {
        if let Some(ref child) = self.child {
            drop(child);
        } else if self.browser.session_active() {
            let _ = self.browser.hide();
        }

        self.child = None;
    }

    pub fn summon(&mut self, what: &str) {
        self.disappear();
        self.child = process::Process::start(what, true);
    }

    pub fn gandalf(&mut self) {
        let cwd = std::env::current_dir().unwrap();
        let cwd_path = cwd.as_path().to_str().unwrap();
        let path = format!("file://{}\\gandalf.html", cwd_path).replace("\\", "\\\\");
        self.url(&path);
    }

    pub fn url(&mut self, url: &str) {
        if !self.browser.session_active() {
            self.disappear();
            let _ = self.browser.show();
        }

        let _ = self.browser.url(url);
    }

    pub fn yt(&mut self, url: &str) {
        if !self.browser.session_active() {
            self.disappear();
            let _ = self.browser.show();
        }

        let _ = self.browser.yt(url);
    }
}

impl std::ops::Drop for Summoner {
    fn drop(&mut self) {
        self.disappear();
    }
}

fn main() {
    use gandalf::IncommingMessage;
    use bincode::deserialize;
    use std::net::{UdpSocket, IpAddr, Ipv4Addr};
    use pnet::datalink::interfaces;

    #[cfg(build = "release")]
    #[cfg(target_os = "windows")]
    unsafe {
        kernel32::FreeConsole();
    }

    let socket = UdpSocket::bind(BIND_SOCKET).expect("Unable to bind socket 0.0.0.0:23441");
    let multicast_ip = Ipv4Addr::new(237, 77, 7, 77);

    for int in interfaces() {
        for ip in int.ips {
            if let IpAddr::V4(ip) = ip.ip() {
                if !ip.is_unspecified() {
                    println!("JOINING 237.77.7.77 VIA {}", ip);
                    socket.join_multicast_v4(&multicast_ip, &ip).unwrap();
                }
            }
        }
    }

    let mut buf = [0u8; 1024];
    let mut summoner = Summoner::new();

    loop {
        let msg: IncommingMessage = match socket.recv_from(&mut buf) {
            Ok((size, src_addr)) => {
                println!("RECV {} FROM {:?}", size, src_addr);

                match deserialize(&buf[..size]) {
                    Ok(msg) => msg,
                    Err(err) => panic!("Unable to deserialize. {:?}", err),
                }
            }
            Err(..) => panic!("Unable to recv from socket 0.0.0.0:23441"),
        };

        println!("MSG {:?}", msg);

        match msg {
            IncommingMessage::Summon { what } => summoner.summon(&what),
            IncommingMessage::Gandalf => summoner.gandalf(),
            IncommingMessage::Disappear => summoner.disappear(),
            IncommingMessage::Url(url) => summoner.url(url.as_str()),
            IncommingMessage::Yt { vid } => summoner.yt(vid.as_str()),
            IncommingMessage::Retreat => {
                summoner.disappear();
                return;
            }
        }
    }
}
