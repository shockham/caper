use types::Vector3;
use glium::texture::Texture1d;
use glium::backend::{ Facade, Context };
use std::cell::RefCell;
use std::rc::Rc;

/// Struct containing the data for the lighting system
pub struct Lighting {
    context: Rc<Context>,
    directional_lights: Vec<Vector3>,
    directional_tex: RefCell<Option<Texture1d>>,
}

impl Lighting {
    /// Create a new lighting system
    pub fn new<F>(facade: &F) -> Lighting where F: Facade + Clone {
        Lighting {
            context: facade.get_context().clone(),
            directional_lights: Vec::new(),
            directional_tex: RefCell::new(None), 
        }
    }


    /// Add a direction light with direction dir to the lighting system
    pub fn add_directional_light(&mut self, dir: Vector3) {
        self.directional_lights.push(dir); 

        // regenerate the tex that is used to send location to shader
        let mut dir_tex = self.directional_tex.borrow_mut();

        if dir_tex.is_none() {
            let dir_tex_1d = Texture1d::new(&self.context, self.directional_lights.as_slice()).unwrap();
            *dir_tex = Some(dir_tex_1d);
        }
    }
}
