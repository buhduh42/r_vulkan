pub use glm::Vector4;
pub use glm::Vector2;

pub enum Mesh {
    PositionMesh(Vec<PostionVertex>),
    TextureMesh(Vec<TextureVertex>),
    NormalMesh(Vec<NormalVertex>),
}

/*
pub enum Vertex {
    PositionVertex,
    TextureVertex,
    NormalVertex,
}
*/

pub type PositionCoord = f32;
pub type TextureCoord = f32;
pub type NormalCoord = f32;
pub type IndexCoord = u32;

pub struct PostionVertex(glm::Vector4<PositionCoord>);

pub struct TextureVertex {
    pos: Vector4<PositionCoord>,
    uv: Vector2<TextureCoord>,
}

pub struct NormalVertex {
    pos: Vector4<PositionCoord>,
    uv: Vector2<TextureCoord>,
    norm: Vector4<NormalCoord>,
}

impl NormalVertex {
    //cache decoherent
    pub fn new(
        pos: Vector4<PositionCoord>, 
        uv: Vector2<TextureCoord>, 
        norm: Vector4<NormalCoord>,
    ) -> Self {
        NormalVertex{
            pos,
            uv,
            norm,
        }
    }
}

pub struct Model {
    pub mesh: Mesh,
    pub indeces: Vec<IndexCoord>,
}
