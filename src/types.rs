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
#[builder(default)]
pub struct Transform {
    /// The position of the transform
    pub pos: Vector3,
    /// The rotation of the transform
    pub rot: Quaternion,
    /// The scale of the transform
    pub scale: Vector3,
    /// Whether the transform is currently active/should be rendered
    pub active: bool,
    /// Whether the transform is frustum culled
    pub cull: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            pos: (0f32, 0f32, 0f32),
            rot: (0f32, 0f32, 0f32, 0f32),
            scale: (1f32, 1f32, 1f32),
            active: true,
            cull: true,
        }
    }
}

unsafe impl Send for Transform {}

/// Denotes how the RenderItem acts in the physics engine
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum PhysicsType {
    /// Item is Collidable but does not move
    Static,
    /// Item is Colliable and moves
    Dynamic,
    /// Item does not use the physics engine
    None,
}

/// Default RenderItem utype
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct DefaultTag;

/// struct for abstracting items to be sent to render
#[derive(Builder, Clone, Serialize, Deserialize, PartialEq)]
#[builder(default)]
pub struct RenderItem<T: Default> {
    /// The vertices representing this items mesh
    pub vertices: Vec<Vertex>,
    /// The material that will be used for rendering the Item
    pub material: Material,
    /// The instances of this item
    pub instance_transforms: Vec<Transform>,
    /// Whether the item is active/should be rendered
    pub active: bool,
    /// How this item acts in the physics engine
    pub physics_type: PhysicsType,
    /// The name of the RenderItem for lookup
    #[builder(setter(into))]
    pub name: String,
    /// Tag Type for grouping similar items
    pub tag: T,
}

impl<T: Default> Default for RenderItem<T> {
    fn default() -> Self {
        RenderItem {
            vertices: Default::default(),
            material: Default::default(),
            instance_transforms: Default::default(),
            active: true,
            physics_type: PhysicsType::None,
            name: "ri".into(),
            tag: Default::default(),
        }
    }
}

unsafe impl<T: Default> Send for RenderItem<T> {}

/// Struct for containing material information
#[derive(Builder, Clone, Serialize, Deserialize, PartialEq)]
#[builder(default)]
pub struct Material {
    /// The shader that will used to render this item
    #[builder(setter(into))]
    pub shader_name: String,
    /// The texture that will be used
    pub texture_name: Option<String>,
    /// The normal texture that will be used
    pub normal_texture_name: Option<String>,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            shader_name: "dist".into(),
            texture_name: None,
            normal_texture_name: None,
        }
    }
}

/// struct for abstacting text items to be rendered
#[derive(Builder, Clone, Serialize, Deserialize, PartialEq)]
#[builder(default)]
pub struct TextItem {
    /// The text that the item displays
    #[builder(setter(into))]
    pub text: String,
    /// The color the text is displayed in
    pub color: (f32, f32, f32, f32),
    /// The position to display this text
    pub pos: Vector3,
    /// The scale/size the text is displayed at
    pub scale: Vector3,
    /// Whether this item is active/should be rendered
    pub active: bool,
    /// The name of the RenderItem for lookup
    #[builder(setter(into))]
    pub name: String,
}

impl Default for TextItem {
    fn default() -> Self {
        TextItem {
            text: Default::default(),
            color: (0f32, 0f32, 0f32, 1f32),
            pos: (0f32, 0f32, 0f32),
            scale: (1f32, 1f32, 1f32),
            active: true,
            name: "ti".into(),
        }
    }
}

unsafe impl Send for TextItem {}

/// struct for abstracting the camera state
#[derive(Builder, Copy, Clone, Serialize, Deserialize, PartialEq)]
#[builder(default)]
pub struct Camera {
    /// The position of the camera in 3d space
    pub pos: Vector3,
    /// The euler rotation of the camera
    pub euler_rot: Vector3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: (0f32, 0f32, 0f32),
            euler_rot: (0f32, 0f32, 0f32),
        }
    }
}

/// struct for shader attributes
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShaderIn {
    /// The world position to be passed to the shader
    pub world_position: Vector3,
    /// The world rotation to be passed to the shader
    pub world_rotation: Quaternion,
    /// The world scale to be passed to the shader
    pub world_scale: Vector3,
}
implement_vertex!(ShaderIn, world_position, world_rotation, world_scale);
