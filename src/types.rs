/// type definition for a Vector3
pub type Vector3 = (f32, f32, f32);

/// type definition for a Quaternion
pub type Quaternion = (f32, f32, f32, f32);

/// type definition for a matrix4
pub type Matrix4 = [[f32; 4]; 4];

/// struct for defining a Vector for creating meshes
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Builder, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transform {
    /// The position of the transform
    #[builder(default="(0f32, 0f32, 0f32)")]
    pub pos: Vector3,
    /// The rotation of the transform
    #[builder(default="(0f32, 0f32, 0f32, 1f32)")]
    pub rot: Quaternion,
    /// The scale of the transform
    #[builder(default="(1f32, 1f32, 1f32)")]
    pub scale: Vector3,
    /// Whether the transform is currently active/should be rendered
    #[builder(default="true")]
    pub active: bool,
}

/// Denotes how the RenderItem acts in the physics engine
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum PhysicsType {
    /// Item is Collidable but does not move
    Static,
    /// Item is Colliable and moves
    Dynamic,
    /// Item does not use the physics engine
    None,
}

/// struct for abstracting items to be sent to render
#[derive(Builder, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenderItem {
    /// The vertices representing this items mesh
    #[builder(default="Vec::new()")]
    pub vertices: Vec<Vertex>,
    /// The material that will be used for rendering the Item
    #[builder(default="MaterialBuilder::default().build().unwrap()")]
    pub material: Material,
    /// The instances of this item
    #[builder(default="Vec::new()")]
    pub instance_transforms: Vec<Transform>,
    /// Whether the item is active/should be rendered
    #[builder(default="true")]
    pub active: bool,
    /// How this item acts in the physics engine
    #[builder(default="PhysicsType::None")]
    pub physics_type: PhysicsType,
    /// The name of the RenderItem for lookup
    #[builder(default="\"ri\".to_string()")]
    pub name: String,
}

/// Struct for containing material information
#[derive(Builder, Clone, Serialize, Deserialize, PartialEq)]
pub struct Material {
    /// The shader that will used to render this item
    #[builder(default="\"dist\".to_string()")]
    pub shader_name: String,
    /// The texture that will be used
    #[builder(default="None")]
    pub texture_name: Option<String>,
    /// The normal texture that will be used
    #[builder(default="None")]
    pub normal_texture_name: Option<String>,
}

/// struct for abstacting text items to be rendered
#[derive(Builder, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextItem {
    /// The text that the item displays
    #[builder(default="\"\".to_string()")]
    pub text: String,
    /// The color the text is displayed in
    #[builder(default="(0f32, 0f32, 0f32, 1f32)")]
    pub color: (f32, f32, f32, f32),
    /// The position to display this text
    #[builder(default="(0f32, 0f32, 0f32)")]
    pub pos: Vector3,
    /// The scale/size the text is displayed at
    #[builder(default="(1f32, 1f32, 1f32)")]
    pub scale: Vector3,
    /// Whether this item is active/should be rendered
    #[builder(default="true")]
    pub active: bool,
    /// The name of the RenderItem for lookup
    #[builder(default="\"ti\".to_string()")]
    pub name: String,
}

/// struct for abstracting the camera state
#[derive(Builder, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct CamState {
    /// The position of the camera in 3d space
    #[builder(default="(0f32, 0f32, 0f32)")]
    pub cam_pos: Vector3,
    /// The euler rotation of the camera
    #[builder(default="(0f32, 0f32, 0f32)")]
    pub cam_rot: Vector3,
}

/// struct for shader attributes
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attr {
    /// The world position to be passed to the shader
    pub world_position: Vector3,
    /// The world rotation to be passed to the shader
    pub world_rotation: Quaternion,
    /// The world scale to be passed to the shader
    pub world_scale: Vector3,
}
implement_vertex!(Attr, world_position, world_rotation, world_scale);
