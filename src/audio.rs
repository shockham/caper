use rodio;
use rodio::{ Endpoint, Sink };
use std::collections::HashMap;
use std::io::{ BufReader, SeekFrom, Seek };
use std::fs::File;

/// Struct representing the Audio system
pub struct Audio {
    endpoint: Endpoint,
    audio: HashMap<&'static str, File>,
    channels: HashMap<&'static str, Sink>,
}

impl Audio {
    /// Creates a new instance of the Audio system
    pub fn new() -> Audio {
        Audio {
            endpoint: rodio::get_default_endpoint().unwrap(),
            audio: HashMap::new(),
            channels: HashMap::new(),
        }
    }

    /// Adds a piece of audio to the system
    pub fn add_audio(&mut self, name: &'static str, path: &'static str) {
        let file = File::open(path).unwrap();
        self.audio.insert(name, file);
        self.channels.insert(name, Sink::new(&self.endpoint));
    }

    /// Plays an piece of Audio that has been loaded into the system
    pub fn play(&mut self, name: &'static str) {
        let audio = self.audio.get(name).unwrap();
        let mut audio = (*audio).try_clone().unwrap();
        audio.seek(SeekFrom::Start(0u64)).unwrap();

        self.channels.get(name).unwrap().append(rodio::Decoder::new(BufReader::new(audio)).unwrap());

    }
}
