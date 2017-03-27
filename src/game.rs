use renderer::Renderer;
use types::{ RenderItem, TextItem, CamState, PhysicsType };
use input::{ Input, Key, MouseButton };
use imgui::Ui;
use nalgebra::Vector3 as nVector3;
use nalgebra::Translation3;
use nalgebra::core::storage::OwnedStorage;
use nphysics3d::world::World;
use nphysics3d::object::{ RigidBody, WorldObject };
use ncollide::shape::Cuboid;

use std::boxed::Box;
use std::time::Instant;

// some physics constants
const PHYSICS_DIVISOR:f32 = 2f32;
const GLOBAL_REST:f32 = 0.05f32;

/// Struct for creating an instance of a game with all systems and items contained
pub struct Game {
    /// The input system for the game
    pub input: Input,
    /// The render system for the game
    pub renderer: Renderer,
    /// The physics system
    pub physics: World<f32>,
    /// Simple struct for camera data
    pub cam_state: CamState,
    /// All of the mesh items to be rendered in the game
    pub render_items: Vec<RenderItem>,
    /// All the text items to be rendered in the game
    pub text_items: Vec<TextItem>,
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
            cam_rot: (0.0f32, 0.0, 0.0)
        };

        Game {
            input: Input::new(),
            renderer: Renderer::new("caper window".to_string()),
            physics: world,
            cam_state: cam_state,
            render_items: Vec::new(),
            text_items: Vec::new(),
        }
    }

    /// Initialised physics on the render items
    pub fn init(&mut self) {
        // add physics items
        for i in 0 .. self.render_items.len() {
            match self.render_items[i].physics_type {
                PhysicsType::Static => {
                    for j in 0 .. self.render_items[i].instance_transforms.len() {
                        let ri_trans = self.render_items[i].instance_transforms[j];

                        let geom = Cuboid::new(
                            nVector3::new(ri_trans.scale.0, ri_trans.scale.1, ri_trans.scale.2));
                        let mut rb = RigidBody::new_static(geom, GLOBAL_REST, 0.6);

                        rb.append_translation(&Translation3::new(ri_trans.pos.0, ri_trans.pos.1, ri_trans.pos.2));

                        // track which render item instance this refers to
                        rb.set_user_data(Some(Box::new((i, j))));

                        rb.set_margin(0f32);

                        self.physics.add_rigid_body(rb);
                    }
                },
                PhysicsType::Dynamic => {
                    for j in 0 .. self.render_items[i].instance_transforms.len() {
                        let ri_trans = self.render_items[i].instance_transforms[j];

                        let geom = Cuboid::new(
                            nVector3::new(ri_trans.scale.0, ri_trans.scale.1, ri_trans.scale.2));
                        let mut rb = RigidBody::new_dynamic(geom, 5.0, GLOBAL_REST, 0.8);

                        rb.append_translation(&Translation3::new(ri_trans.pos.0, ri_trans.pos.1, ri_trans.pos.2));

                        // track which render item instance this refers to
                        rb.set_user_data(Some(Box::new((i, j))));

                        rb.set_margin(0f32);

                        if i == 1 && j == 0 {
                            rb.set_deactivation_threshold(None);
                        }

                        self.physics.add_rigid_body(rb);
                    }
                },
                PhysicsType::None => {},
            }
        }
    }

    /// Starting the game loop
    pub fn start<U: FnMut(&Game), F: FnMut(&Ui)>(&mut self, mut update: U, mut render_imgui: F) {

        let mut delta = 0.016666667f32;

        // the main loop
        loop {
            let frame_start = Instant::now();
            // quit
            if self.input.keys_down.contains(&Key::Escape) { break; }

            // block for updating physics
            {
                self.physics.step(delta);

                for rbi in self.physics.rigid_bodies() {
                    // actually get access to the rb :|
                    let wo = WorldObject::RigidBody(rbi.clone());
                    let rb = wo.borrow_rigid_body();

                    // update the RenderItem transform pos
                    let trans = rb.position().translation.vector;
                    let rot = rb.position().rotation.coords.data.as_slice();
                    //let quat = to_quaternion((rot.x, rot.y, rot.z));

                    let user_data = rb.user_data().unwrap();
                    let &(ri_i, ri_it_i) = user_data.downcast_ref::<(usize, usize)>().unwrap();

                    if self.render_items.len() > ri_i && self.render_items[ri_i].instance_transforms.len() > ri_it_i {
                        self.render_items[ri_i].instance_transforms[ri_it_i].pos =
                            (trans.x / PHYSICS_DIVISOR,
                             trans.y / PHYSICS_DIVISOR,
                             trans.z / PHYSICS_DIVISOR);
                        self.render_items[ri_i].instance_transforms[ri_it_i].rot = (rot[0], rot[1], rot[2], rot[3]);
                    }
                }
            }

            {
                // render the frame
                self.renderer.draw(&self.cam_state, &self.render_items, &self.text_items, &mut render_imgui);

                // updating and handling the inputs
                self.input.update_inputs(&self.renderer.display);

                // update the inputs for imgui
                self.renderer.update_imgui_input(self.input.mouse_pos,
                                             (self.input.mouse_btns_down.contains(
                                                     &MouseButton::Left), false, false));
            }

            // the update block for other updates
            {
                update(&self);
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

                    let mut rb = wo.borrow_mut_rigid_body();

                    // check if it actually exists, if it doesn't remove
                    if self.render_items.len() > ri_i && self.render_items[ri_i].instance_transforms.len() > ri_it_i {
                        // update the rb transform pos
                        let ri_pos = self.render_items[ri_i].instance_transforms[ri_it_i].pos;
                        rb.set_translation(Translation3::new(ri_pos.0 * PHYSICS_DIVISOR,
                                                             ri_pos.1 * PHYSICS_DIVISOR,
                                                             ri_pos.2 * PHYSICS_DIVISOR));
                    }
                }
            }

            delta = 0.000000001f32 * frame_start.elapsed().subsec_nanos() as f32;
        }
    }
}