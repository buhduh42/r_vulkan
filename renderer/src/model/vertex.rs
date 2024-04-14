pub trait DataType: glm::Primitive { type A; }

pub type PositionCoord = f32;
pub type TextureCoord = f32;
pub type NormalCoord = f32;

pub struct PositionVertex {
    pos: glm::Vector4<PositionCoord>,
}

pub struct TextureVertex {
    pos: glm::Vector4<PositionCoord>,
    uv: glm::Vector2<TextureCoord>,
}

pub struct NormalVertex {
    pos: glm::Vector4<PositionCoord>,
    uv: glm::Vector2<TextureCoord>,
    norm: glm::Vector4<NormalCoord>, //using homogenous for now
}

impl PositionVertex {
    //pub fn new(
}

//pub struct Vertex<T: DataType>(glm::Vector3<T>);
