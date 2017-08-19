use renderer::Renderer;
use audio::Audio;
use types::{RenderItem, TextItem, CamState, PhysicsType};
use input::Input;
use imgui::Ui;
use nalgebra::Vector3 as nVector3;
use nalgebra::Translation3;
use nalgebra::core::storage::OwnedStorage;
use nphysics3d::world::World;
use nphysics3d::object::{RigidBody, WorldObject};
use ncollide::shape::Cuboid;

use std::boxed::Box;
use std::time::Instant;

/// The divisor for the physics space to align with render space
const PHYSICS_DIVISOR: f32 = 2f32;
/// global restitution for physics objects
const GLOBAL_REST: f32 = 0.05f32;

/// Struct for creating an instance of a game with all systems and items contained
pub struct Game {
    /// The input system for the game
    pub input: Input,
    /// The render system for the game
    pub renderer: Renderer,
    /// The physics system
    pub physics: World<f32>,
    /// The audio system
    pub audio: Audio,
    /// Simple struct for camera data
    pub cam_state: CamState,
    /// All of the mesh items to be rendered in the game
    render_items: Vec<RenderItem>,
    /// All the text items to be rendered in the game
    text_items: Vec<TextItem>,
    /// The delta time for each frame
    delta: f32,
}

impl Game {
    /// Creates a new instance of a game
    pub fn new() -> Game {
        // init physics
        let mut world = World::new();
        world.set_gravity(nVector3::new(0.0, -9.81, 0.0));

        //cam state
        let cam_state = CamState {
            cam_pos: (0.0f32, 0.0, 0.0),
            cam_rot: (0.0f32, 0.0, 0.0),
        };

        let (renderer, events_loop) = Renderer::new("caper window".to_string());

        Game {
            input: Input::from_existing(events_loop),
            renderer: renderer,
            physics: world,
            audio: Audio::new(),
            cam_state: cam_state,
            render_items: Vec::new(),
            text_items: Vec::new(),
            delta: 0.016666667f32,
        }
    }

    /// Get the len of render_items
    pub fn render_items_len(&self) -> usize {
        self.render_items.len()
    }

    /// Get a ref to a render item
    pub fn get_render_item(&mut self, index: usize) -> &mut RenderItem {
        &mut self.render_items[index]
    }

    /// Get a ref to a render item from its name, returning the first found
    pub fn get_render_item_by_name(&mut self, name: String) -> Option<&mut RenderItem> {
        for i in 0..self.render_items.len() {
            if self.render_items[i].name == name {
                return Some(&mut self.render_items[i]);
            }
        }
        None
    }

    /// Add a render item to the game
    pub fn add_render_item(&mut self, render_item: RenderItem) {
        // add the render item
        self.render_items.push(render_item);

        // the index of the newly added item
        let i = self.render_items.len() - 1;

        // setup the physics for the item
        self.add_physics(i);
    }

    /// Initalise physics depending on PhysicsType
    pub fn add_physics(&mut self, i: usize) {
        // add the rigid body if needed
        match self.render_items[i].physics_type {
            PhysicsType::Static => {
                for j in 0..self.render_items[i].instance_transforms.len() {
                    let ri_trans = self.render_items[i].instance_transforms[j];

                    let geom = Cuboid::new(nVector3::new(
                        ri_trans.scale.0,
                        ri_trans.scale.1,
                        ri_trans.scale.2,
                    ));
                    let mut rb = RigidBody::new_static(geom, GLOBAL_REST, 0.6);

                    rb.append_translation(&Translation3::new(
                        ri_trans.pos.0 * PHYSICS_DIVISOR,
                        ri_trans.pos.1 * PHYSICS_DIVISOR,
                        ri_trans.pos.2 * PHYSICS_DIVISOR,
                    ));

                    // track which render item instance this refers to
                    rb.set_user_data(Some(Box::new((i, j))));

                    rb.set_margin(0f32);

                    self.physics.add_rigid_body(rb);
                }
            }
            PhysicsType::Dynamic => {
                for j in 0..self.render_items[i].instance_transforms.len() {
                    let ri_trans = self.render_items[i].instance_transforms[j];

                    let geom = Cuboid::new(nVector3::new(
                        ri_trans.scale.0,
                        ri_trans.scale.1,
                        ri_trans.scale.2,
                    ));
                    let mut rb = RigidBody::new_dynamic(geom, 5.0, GLOBAL_REST, 0.8);

                    rb.append_translation(&Translation3::new(
                        ri_trans.pos.0 * PHYSICS_DIVISOR,
                        ri_trans.pos.1 * PHYSICS_DIVISOR,
                        ri_trans.pos.2 * PHYSICS_DIVISOR,
                    ));

                    // track which render item instance this refers to
                    rb.set_user_data(Some(Box::new((i, j))));

                    rb.set_margin(0f32);

                    if i == 1 && j == 0 {
                        rb.set_deactivation_threshold(None);
                    }

                    self.physics.add_rigid_body(rb);
                }
            }
            PhysicsType::None => {}
        }
    }

    /// Get the len of render_items
    pub fn text_items_len(&self) -> usize {
        self.text_items.len()
    }

    /// Get a ref to a text item
    pub fn get_text_item(&mut self, index: usize) -> &mut TextItem {
        &mut self.text_items[index]
    }

    /// Get a ref to a text item from its name, returning the first found
    pub fn get_text_item_by_name(&mut self, name: String) -> Option<&mut TextItem> {
        for i in 0..self.text_items.len() {
            if self.text_items[i].name == name {
                return Some(&mut self.text_items[i]);
            }
        }
        None
    }

    /// Add a text item to the game
    pub fn add_text_item(&mut self, text_item: TextItem) {
        self.text_items.push(text_item);
    }

    /// Starting the game loop
    pub fn update<F: FnMut(&Ui)>(&mut self, mut render_imgui: F) {
        let frame_start = Instant::now();

        // update the inputs
        {
            // updating and handling the inputs
            let gl_window = self.renderer.display.gl_window();
            let window = gl_window.window();
            self.input.update_inputs(window);
        }
        {
            // update the inputs for imgui
            self.renderer.update_imgui_input(&self.input);
        }

        // update the new positions back to rb
        {
            for rbi in self.physics.rigid_bodies() {
                // actually get access to the rb :|
                let mut wo = WorldObject::RigidBody(rbi.clone());

                let (ri_i, ri_it_i) = {
                    let rb = wo.borrow_rigid_body();

                    let user_data = rb.user_data().unwrap();
                    let tup_ref = user_data.downcast_ref::<(usize, usize)>().unwrap();

                    *tup_ref
                };

                // check if it actually exists, if it doesn't remove
                if self.render_items.len() > ri_i &&
                    self.render_items[ri_i].instance_transforms.len() > ri_it_i
                {
                    // update the rb transform pos
                    let mut rb = wo.borrow_mut_rigid_body();
                    let ri_pos = self.render_items[ri_i].instance_transforms[ri_it_i].pos;
                    rb.set_translation(Translation3::new(
                        ri_pos.0 * PHYSICS_DIVISOR,
                        ri_pos.1 * PHYSICS_DIVISOR,
                        ri_pos.2 * PHYSICS_DIVISOR,
                    ));
                }
            }
        }

        // block for updating physics
        {
            // update all the physics items
            self.physics.step(self.delta);

            for rbi in self.physics.rigid_bodies() {
                // actually get access to the rb :|
                let wo = WorldObject::RigidBody(rbi.clone());
                let rb = wo.borrow_rigid_body();

                // update the RenderItem transform pos
                let trans = rb.position().translation.vector;
                let rot = rb.position().rotation.coords.data.as_slice();

                let user_data = rb.user_data().unwrap();
                let &(ri_i, ri_it_i) = user_data.downcast_ref::<(usize, usize)>().unwrap();

                if self.render_items.len() > ri_i &&
                    self.render_items[ri_i].instance_transforms.len() > ri_it_i
                {
                    self.render_items[ri_i].instance_transforms[ri_it_i].pos =
                        (
                            trans.x / PHYSICS_DIVISOR,
                            trans.y / PHYSICS_DIVISOR,
                            trans.z / PHYSICS_DIVISOR,
                        );
                    self.render_items[ri_i].instance_transforms[ri_it_i].rot =
                        (rot[0], rot[1], rot[2], rot[3]);
                }
            }
        }

        // render the frame
        {
            self.renderer.draw(
                &mut self.cam_state,
                &mut self.render_items,
                &mut self.text_items,
                &mut render_imgui,
            );
        }

        self.delta = 0.000000001f32 * frame_start.elapsed().subsec_nanos() as f32;
    }
}
