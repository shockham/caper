use types::Vector3;
use glium::texture::Texture1d;
use glium::backend::{ Facade, Context };
use std::cell::RefCell;
use std::rc::Rc;

/// Struct containing the data for the lighting system
pub struct Lighting {
    context: Rc<Context>,
    directional_lights: Vec<Vector3>,
    /// Texture representing the positions of the directional lights
    pub directional_tex: RefCell<Texture1d>,
}

impl Lighting {
    /// Create a new lighting system
    pub fn new<F>(facade: &F) -> Lighting where F: Facade + Clone {
        let context = facade.get_context().clone();
        let dir_lights:Vec<Vector3> = Vec::new();
        let dir_tex = Texture1d::new(&context, dir_lights.as_slice()).unwrap();

        Lighting {
            context: context,
            directional_lights: Vec::new(),
            directional_tex: RefCell::new(dir_tex), 
        }
    }


    /// Add a direction light with direction dir to the lighting system
    pub fn add_directional_light(&mut self, dir: Vector3) {
        self.directional_lights.push(dir); 
        self.regenerate_dir_tex();
    }


    /// Regenerates the tex that is used to send location to shader
    fn regenerate_dir_tex(&mut self) {
        let mut dir_tex = self.directional_tex.borrow_mut();
        let dir_tex_1d = Texture1d::new(&self.context, self.directional_lights.as_slice()).unwrap();
        *dir_tex = dir_tex_1d;
    }

    /// Get a ref to a directional light
    pub fn get_directional_light(&mut self, index: usize) -> &mut Vector3 {
        &mut self.directional_lights[index]
    }
}
