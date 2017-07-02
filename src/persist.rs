use bincode::{serialize, deserialize, Infinite};
use serde::ser::Serialize;
use serde::de::Deserialize;

use std::fs::{File, create_dir };
use std::thread;
use std::io::{ Read, Write };

/// The base path the persistent items will be placed
const PERSIST_BASE_PATH:&'static str = "./persist/";

/// Save and encodable type to persistence at the key
pub fn save<T: Serialize>(to_save: &T, key: &'static str) {
    let encoded: Vec<u8> = serialize(to_save, Infinite).unwrap();

    let _ = thread::spawn(move || {
        // TODO handle this better
        let _ = match create_dir(PERSIST_BASE_PATH) {
            _ => (),
        };

        // create the file
        let mut f = File::create(format!("{}{}", PERSIST_BASE_PATH, key)).unwrap();
        // write the bytes to it
        f.write_all(encoded.as_slice()).unwrap();

        let _ = f.flush();
    }).join();
}

/// Load a decodable type from persistence using the key
pub fn load<T: Deserialize>(key: &'static str) -> Result<T, String> {
    let mut f = match File::open(format!("{}{}", PERSIST_BASE_PATH, key)) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };

    let mut byte_vec = Vec::new();
    let _ = f.read_to_end(&mut byte_vec);

    let decoded: T = deserialize(&byte_vec[..]).unwrap();

    Ok(decoded)
}

#[test]
fn save_load_test() {
    #[derive(Serialize, Deserialize, PartialEq)]
    struct Entity {
        x: f32,
        y: f32,
    }

    #[derive(Serialize, Deserialize, PartialEq)]
    struct World {
        entities: Vec<Entity>
    }

    let world = World {
        entities: vec![Entity {x: 0.0, y: 4.0}, Entity {x: 10.0, y: 20.5}]
    };

    save(&world, "test");
    let _ = load::<Entity>("test");
}
