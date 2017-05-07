
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub enum IncommingMessage {
    Summon {
        // spawn child process
        what: String,
    },
    Url(String),
    Yt { vid: String },
    Gandalf,
    Disappear, // kill child process
    Retreat, // disconnect and kill daemon
}
