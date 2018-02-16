use types::Vector3;
use glium::texture::Texture1d;
use glium::backend::{Context, Facade};
use std::cell::RefCell;
use std::rc::Rc;

use rayon::prelude::*;

/// Struct containing the data for the lighting system
pub struct Lighting {
    /// Ref to the render context
    context: Rc<Context>,
    /// Collection of the directional lights
    directional_lights: Vec<DirectionalLight>,
    /// Texture representing the positions of the directional lights
    pub directional_tex: RefCell<Texture1d>,
}

/// Struct for defining a directional light
#[derive(Builder, Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectionalLight {
    /// The name of the directional light
    #[builder(default = "\"light\".to_string()")]
    pub name: String,
    /// The direction the light is facing
    #[builder(default = "(1f32, 1f32, 1f32)")]
    pub dir: Vector3,
    /// Whether the light is active
    #[builder(default = "true")]
    pub active: bool,
}

impl Lighting {
    /// Create a new lighting system
    pub fn new<F>(facade: &F) -> Lighting
    where
        F: Facade + Clone,
    {
        let context = facade.get_context().clone();
        let dir_lights: Vec<Vector3> = Vec::new();
        let dir_tex = Texture1d::new(&context, dir_lights.as_slice()).unwrap();

        Lighting {
            context: context,
            directional_lights: Vec::new(),
            directional_tex: RefCell::new(dir_tex),
        }
    }

    /// Add a direction light with direction dir to the lighting system
    pub fn add_directional_light(&mut self, name: String, dir: Vector3) {
        let light = DirectionalLightBuilder::default()
            .name(name)
            .dir(dir)
            .build()
            .unwrap();
        self.directional_lights.push(light);
        self.regenerate_lighting_tex();
    }

    /// Regenerates the tex that is used to send location to shader
    /// Note: must be called when mutating any lighting data
    pub fn regenerate_lighting_tex(&mut self) {
        let mut dir_tex = self.directional_tex.borrow_mut();

        let lights = self.directional_lights
            .par_iter()
            .filter(|d| d.active)
            .map(|d| d.dir)
            .collect::<Vec<Vector3>>();

        let dir_tex_1d = Texture1d::new(&self.context, lights.as_slice()).unwrap();
        *dir_tex = dir_tex_1d;
    }

    /// Get a ref to a directional light
    pub fn get_directional_light(&mut self, index: usize) -> &mut DirectionalLight {
        &mut self.directional_lights[index]
    }

    /// Get a ref to a directional light from its name, returning the first found
    pub fn get_directional_light_by_name(&mut self, name: String) -> Option<&mut DirectionalLight> {
        for i in 0..self.directional_lights.len() {
            if self.directional_lights[i].name == name {
                return Some(&mut self.directional_lights[i]);
            }
        }
        None
    }
}
