/// type definition for a Vector3
pub type Vector3 = (f32, f32, f32);

/// type definition for a Quaternion
pub type Quaternion = (f32, f32, f32, f32);

/// struct for defining a Vector for creating meshes
#[derive(Copy, Clone, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Vertex {
    /// The position of the vertex
    pub position: [f32; 3],
    /// The normal direction of the vertex
    pub normal: [f32; 3],
    /// The uv/texture coordinates
    pub texture: [f32; 2],
}
implement_vertex!(Vertex, position, normal, texture);

/// struct for handling transform data
#[derive(Copy, Clone, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Transform {
    /// The position of the transform
    pub pos: Vector3,
    /// The rotation of the transform
    pub rot: Quaternion,
    /// The scale of the transform
    pub scale: Vector3,
    /// Whether the transform is currently active/should be rendered
    pub active: bool,
}

/// Denotes how the RenderItem acts in the physics engine
#[derive(Clone, RustcEncodable, RustcDecodable, PartialEq)]
pub enum PhysicsType {
    /// Item is Collidable but does not move
    Static,
    /// Item is Colliable and moves
    Dynamic,
    /// Item does not use the physics engine
    None,
}

/// struct for abstracting items to be sent to render
#[derive(Clone, RustcEncodable, RustcDecodable, PartialEq)]
pub struct RenderItem {
    /// The vertices representing this items mesh
    pub vertices: Vec<Vertex>,
    /// The shader that will used to render this item
    pub shader_name: String,
    /// The instances of this item
    pub instance_transforms: Vec<Transform>,
    /// Whether the item is active/should be rendered
    pub active: bool,
    /// How this item acts in the physics engine
    pub physics_type: PhysicsType,
}

/// struct for abstacting text items to be rendered
#[derive(Clone, RustcEncodable, RustcDecodable, PartialEq)]
pub struct TextItem {
    /// The text that the item displays
    pub text: String,
    /// The color the text is displayed in
    pub color: (f32, f32, f32, f32),
    /// The position to display this text
    pub pos: Vector3,
    /// The scale/size the text is displayed at
    pub scale: Vector3,
    /// Whether this item is active/should be rendered
    pub active: bool,
}

/// struct for abstracting the camera state
#[derive(Copy, Clone, RustcEncodable, RustcDecodable, PartialEq)]
pub struct CamState {
    /// The position of the camera in 3d space
    pub cam_pos: Vector3,
    /// The euler rotation of the camera
    pub cam_rot: Vector3,
}

/// struct for shader attributes
#[derive(Copy, Clone, RustcEncodable, RustcDecodable, PartialEq)]
pub struct Attr {
    /// The world position to be passed to the shader
    pub world_position: Vector3,
    /// The world rotation to be passed to the shader
    pub world_rotation: Quaternion,
    /// The world scale to be passed to the shader
    pub world_scale: Vector3,
}
implement_vertex!(Attr, world_position, world_rotation, world_scale);
