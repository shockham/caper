use rodio;
use rodio::Endpoint;
use std::collections::HashMap;
use std::io::{ BufReader, SeekFrom, Seek };
use std::fs::File;

/// Struct representing the Audio system
pub struct Audio {
    endpoint: Endpoint,
    audio: HashMap<&'static str, File>,
}

impl Audio {
    /// Creates a new instance of the Audio system
    pub fn new() -> Audio {
        Audio {
            endpoint: rodio::get_default_endpoint().unwrap(),
            audio: HashMap::new(),
        }
    }

    /// Adds a piece of audio to the system
    pub fn add_audio(&mut self, name: &'static str, path: &'static str) {
        let file = File::open(path).unwrap();
        self.audio.insert(name, file);
    }

    /// Plays an piece of Audio that has been loaded into the system
    pub fn play(&self, name: &'static str) {
        let audio = self.audio.get(name).unwrap();
        let mut audio = (*audio).try_clone().unwrap();
        audio.seek(SeekFrom::Start(0u64));
        rodio::play_once(&self.endpoint, BufReader::new(audio)).unwrap(); 
    }
}
