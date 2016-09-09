use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};
use rustc_serialize::{Encodable, Decodable};

use std::fs::{File, create_dir };
use std::thread;
use std::io::{ Read, Write };

const PERSIST_BASE_PATH:&'static str = "./persist/";

pub fn save<T: Encodable>(to_save: &T, key: &'static str) {
    let encoded: Vec<u8> = encode(to_save, SizeLimit::Infinite).unwrap();

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

pub fn load<T: Decodable>(key: &'static str) -> Result<T, String> {
    let mut f = match File::open(format!("{}{}", PERSIST_BASE_PATH, key)) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}", e)),
    };

    let mut byte_vec = Vec::new();
    let _ = f.read_to_end(&mut byte_vec);

    let decoded: T = decode(&byte_vec[..]).unwrap();

    Ok(decoded)
}

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct World {
    entities: Vec<Entity>
}

#[test]
fn save_load_test() {
    let world = World {
        entities: vec![Entity {x: 0.0, y: 4.0}, Entity {x: 10.0, y: 20.5}]
    };

    save(&world, "test");
    let _ = load::<Entity>("test");
}
