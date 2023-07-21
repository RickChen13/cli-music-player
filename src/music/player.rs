use crate::common::{event::event_bus::EVENTBUS, path};
use rodio::{Decoder, Sink};
use std::{fs, io::BufReader};

pub struct Player {
    sink: Sink,
    name: String,
}
// play stop pause done
// event
impl Player {
    pub fn new(sink: Sink) -> Player {
        Player {
            sink,
            name: "".to_owned(),
        }
    }

    pub fn init(&mut self, path: &String) -> bool {
        if !self.sink.empty() {
            self.stop();
        }

        let (_dirname, filename, _extension) = path::pathinfo(&path);
        let name: &str = match filename {
            Some(value) => value,
            _ => "",
        };
        self.name = name.to_string();
        let file = fs::File::open(&path).unwrap();
        let source = Decoder::new(BufReader::new(file));
        match source {
            Ok(decoder) => {
                self.sink.append(decoder);

                true
            }
            Err(error) => {
                println!("解码失败:{:?}", error);
                false
            }
        }
    }

    pub fn play(&self) {
        self.sink.play();
        EVENTBUS.emit("musicPlay", self.name.to_string());
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end();
        self.stop();
        EVENTBUS.emit("musicDone", self.name.to_owned());
    }
}
