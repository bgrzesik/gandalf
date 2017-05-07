
extern crate gandalf;
extern crate bincode;
extern crate clap;
extern crate pnet;

use gandalf::IncommingMessage;
use bincode::{serialize, Infinite};
use std::net;
use clap::{App, Arg, SubCommand, AppSettings};

const BIND_SOCKET: (&'static str, u16) = ("0.0.0.0", 23442);

fn main() {
    let matches = App::new("Gandalf Announcer")
        .version("shitty-alpha-that-will-never-leave")
        .author("Bartłomiej Grzesik <grzechovsky@gmail.com>")
        .about("Announces to server that it should make summoners summon gandalf.")
        .setting(AppSettings::SubcommandRequired)
        .arg(Arg::with_name("ip")
                 .short("I")
                 .long("ip")
                 .takes_value(true))
        .subcommand(SubCommand::with_name("summon")
                        .about("Summons proccess")
                        .arg(Arg::with_name("cmd")
                                 .required(true)
                                 .index(1)
                                 .takes_value(true)))
        .subcommand(SubCommand::with_name("cmd")
                        .about("Runs command")
                        .arg(Arg::with_name("cmd")
                                 .required(true)
                                 .index(1)
                                 .takes_value(true)))
        .subcommand(SubCommand::with_name("gandalf").about("Spawns Gandalf"))
        .subcommand(SubCommand::with_name("url")
                        .about("Opens up browser with specified url")
                        .arg(Arg::with_name("url")
                                 .required(true)
                                 .index(1)
                                 .takes_value(true)))
        .subcommand(SubCommand::with_name("yt")
                        .about("Opens up browser with youtube video")
                        .arg(Arg::with_name("yt")
                                 .required(true)
                                 .index(1)
                                 .takes_value(true)))
        .subcommand(SubCommand::with_name("yte")
                        .about("Opens up browser with youtube video")
                        .arg(Arg::with_name("yt")
                                 .required(true)
                                 .index(1)
                                 .takes_value(true)))
        .subcommand(SubCommand::with_name("blank").about("Opens up browser with plank page"))
        .subcommand(SubCommand::with_name("retreat").about("Kills child processes"))
        .subcommand(SubCommand::with_name("disappear").about("Shutdowns daemons"))
        .get_matches();

    let msg: IncommingMessage = match matches.subcommand() {
        ("summon", Some(matches)) => {
            IncommingMessage::Summon { what: matches.value_of("cmd").unwrap().to_string() }
        }
        ("cmd", Some(matches)) => {
            let cmd = matches.value_of("cmd").unwrap().to_string();
            IncommingMessage::Summon { what: format!("cmd /C {:?}", cmd) }
        }
        ("url", Some(matches)) => {
            IncommingMessage::Url(matches.value_of("url").unwrap().to_string())
        }
        ("yte", Some(matches)) => {
            let vid = matches.value_of("yt").unwrap().to_string();
            IncommingMessage::Url(format!("https://www.youtube.com/embed/{}?rel=0&autoplay=1", vid))
        }
        ("yt", Some(matches)) => {
            IncommingMessage::Yt { vid: matches.value_of("yt").unwrap().to_string() }
        }
        ("blank", Some(_)) => IncommingMessage::Url("about:blank".to_owned()),
        ("gandalf", Some(_)) => IncommingMessage::Gandalf,
        ("disappear", Some(_)) => IncommingMessage::Disappear,
        ("retreat", Some(_)) => IncommingMessage::Retreat,
        _ => unimplemented!(),
    };

    let send_socket = (matches.value_of("ip").unwrap_or("237.77.7.77"), 23441);

    let buf = serialize(&msg, Infinite).expect("Unable to serialize message");
    let socket = net::UdpSocket::bind(BIND_SOCKET).expect("Unable to bind socket 0.0.0.0:23441");
    socket
        .set_broadcast(true)
        .expect("Unable to turn off broadcast mode");
    socket
        .send_to(&buf[..], send_socket)
        .expect("Unable to send message");
}
