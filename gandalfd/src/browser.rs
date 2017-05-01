
extern crate serde_json;
extern crate hyper;

use process;

#[derive(Debug)]
pub enum Error {
    IoError(::std::io::Error),
    JsonError(self::serde_json::Error),
    HyperError(self::hyper::Error),
    MsgError(String),
}

pub struct Browser {
    client: self::hyper::client::Client,
    // child: ::std::process::Child,
    child: process::Process,
    session_id: Option<String>,
}

impl Browser {

    pub fn new() -> Browser {
        use std::process::Command;
        use self::hyper::client::{Client};
        use process::Process;

        Browser { client: Client::new(), child: Process::start("chromedriver --silent", false).unwrap(), session_id: None }

        // let child = Command::new("chromedriver")
        //     .arg("--port=9515")
        //     .spawn();
        // 
        // return match child {
        //     Ok(child) => Ok(Browser {
        //         client: Client::new(),
        //         child: child,
        //         session_id: None,
        //     }),
        //     Err(err) => Err(Error::IoError(err))
        // };  
    }

    fn req(&mut self, method: hyper::method::Method, url: &str, body: &str) -> Result<String, Error> {
        use ::std::io::Read;

        println!("{:?} {} {}", method, url, body);
        let mut res = self.client.request(method, url)
            .body(body)
            .send();
        
        println!("SENT");

        match res {
            Ok(mut res) => {
                let mut buf = String::new();
                if let Err(err) = res.read_to_string(&mut buf) {
                    return Err(Error::IoError(err));
                }

                println!("RESPONSE {}", buf);
                Ok(buf)
            },
            Err(err) => {
                println!("ERROR {}", err);
                Err(Error::HyperError(err))
            },
        }
    }

    pub fn show(&mut self) -> Result<(), Error> {
        use self::serde_json;
        use self::serde_json::Value;
        use self::hyper::Post;

        if let Some(_) = self.session_id {
            return Ok(());
        }

        let res = self.req(Post, "http://127.0.0.1:9515/session",r#"{
                "capabilities": {
                    "alwaysMatch": {
                        "browserName": "chrome",
                        "chromeOptions": {
                            "args": [ "--start-maximized", "--kiosk", "--disable-infobars", "--disable-notifications" ],
                            "extensions": [  ]
                        },
                        "platform": "ANY",
                        "version": ""
                    },
                    "firstMatch": [  ]
                },
                "desiredCapabilities": {
                    "browserName": "chrome",
                    "chromeOptions": {
                        "args": [ "--start-maximized", "--kiosk", "--disable-infobars", "--disable-notifications" ],
                        "extensions": [  ]
                    },
                    "platform": "ANY",
                    "version": ""
                }
            }"#);

        match res {
            Ok(res) => match serde_json::from_str(&res){
                Ok(Value::Object(val)) => match val["sessionId"] { 
                    Value::String(ref session_id) => {
                        self.session_id = Some(session_id.to_owned());
                        println!("GOT SESSION {:?}", self.session_id);
                        Ok(())
                    },
                    _ => Err(Error::MsgError(res)),
                },
                _ => Err(Error::MsgError(res)),
            },
            Err(err) => Err(err),
        }
    }

    pub fn url(&mut self, url: &str) -> Result<(), Error> {
        use self::hyper::Post;

        if let None = self.session_id {
            return Ok(());
        }

        let post = {
            let session_id = self.session_id.as_ref().unwrap();
            format!("http://127.0.0.1:9515/session/{}/url", session_id)
        };

        let res = self.req(Post, post.as_str(), format!(r#"{{"url":"{}"}}"#, url).as_str());
        println!("URL {:?}", res);
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn hide(&mut self) -> Result<(), Error> {
        use self::hyper::Delete;

        if let None = self.session_id {
            return Ok(());
        }

        let url = {
            let session_id = self.session_id.as_ref().unwrap();
            format!("http://127.0.0.1:9515/session/{}", session_id)
        };

        let res = self.req(Delete, url.as_str(), "");
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn click(&mut self, selector: &str) -> Result<(), Error> {
        use self::serde_json;
        use self::serde_json::Value;
        use self::hyper::Post;

        if let None = self.session_id {
            return Ok(());
        }
        
        let url = {
            let session_id = self.session_id.as_ref().unwrap();
            format!("http://127.0.0.1:9515/session/{}/elements", session_id)
        };

        let res = self.req(Post, url.as_str(), format!(r#"{{"using":"css selector","value":{:?}}}"#, selector).as_str());
        let res = match res {
            Ok(res) => match serde_json::from_str::<Value>(&res) {
                Ok(res) => res,
                Err(err) => return Err(Error::JsonError(err)),
            },
            Err(err) => return Err(err),
        };

        let eid = match res.pointer("/value/0/ELEMENT") {
            Some(&Value::String(ref eid)) => eid,
            _ => return Err(Error::MsgError("Element not found".to_owned())),
        };
        
        let url = {
            let session_id = self.session_id.as_ref().unwrap();
            format!("http://127.0.0.1:9515/session/{}/element/{}/click", session_id, eid)
        };

        let res = self.req(Post, url.as_str(), "");
        match res {
            Ok(_) => Ok(()),
            Err(err) => return Err(err),
        }
    }

    pub fn yt(&mut self, vid: &str) -> Result<(), Error> {
        if let Err(err) = self.url(format!("https://www.youtube.com/watch?v={}", vid).as_str()) {
            return Err(err);
        }
        // if let Err(err) = self.click(".ytp-fullscreen-button.ytp-button") {
        //     println!("CLICK ERR {:?}", err);
        //     return Err(err);
        // }
        // self.click(".ytp-play-button.ytp-button")
        self.click(".ytp-fullscreen-button.ytp-button")
    }

    pub fn session_active(&self) -> bool {
        self.session_id.is_some()
    }

}

impl ::std::ops::Drop for Browser {
    fn drop(&mut self) {
        self.hide();
    }
}

impl ::std::fmt::Debug for Browser {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Browser {{ session_id: {:?} }}", self.session_id)
    }
}