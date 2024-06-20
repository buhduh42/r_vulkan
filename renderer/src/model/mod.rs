use std::{
    fs::File,
    fmt::Debug,
};

use serde::{Serialize, Deserialize};

pub use glm::Vector4;
pub use glm::Vector2;

pub mod primitives;
pub mod model_manager;

//#[derive(Serialize, Deserialize, Debug)]
pub enum Mesh {
    PositionMesh(Vec<PostionVertex>),
    TextureMesh(Vec<TextureVertex>),
    NormalMesh(Vec<NormalVertex>),
}

//Note: this only works because a Mesh is a Vec underneath
/*
impl Debug for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //not sure of a cleaner way to implement this
        match self {
            Self::PositionMesh(mesh) => {
               f.debug_list().entries(mesh).finish()
            },
            Self::TextureMesh(mesh) => {
               f.debug_list().entries(mesh).finish()
            },
            Self::NormalMesh(mesh) => {
               f.debug_list().entries(mesh).finish()
            },
        }
    }
}
*/

pub type PositionVector = Vector4<PositionCoord>;
type PositionCoord = f32;

pub type TextureVector = Vector2<TextureCoord>;
type TextureCoord = f32;

pub type NormalVector = Vector4<NormalCoord>;
type NormalCoord = f32;

pub type IndexVector = Vec<IndexCoord>;
pub type IndexCoord = u32;

//impl PositionVector {
    //pub fn new(x: PositionCoord, y: PositionCoord, z: PositionCoord) -> Self {
        //Self::new(x, y, z, 1.0)
    //}
//}

//#[derive(Serialize, Deserialize, Debug)]
pub struct PostionVertex(PositionVector);

/*
impl Serialize for PositionVector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
    }
}
*/

//#[derive(Serialize, Deserialize, Debug)]
#[derive(Copy, Clone)]
pub struct TextureVertex {
    pub pos: PositionVector,
    pub uv: TextureVector,
}

//#[derive(Serialize, Deserialize, Debug)]
#[derive(Copy, Clone)]
pub struct NormalVertex {
    pub pos: PositionVector,
    pub uv: TextureVector,
    pub norm: NormalVector,
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

pub const DEFAULT_MODEL_NAME: &str = "unnamed";

//#[derive(Debug)]
//#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub name: String,
    pub mesh: Mesh,
    pub indeces: Vec<IndexCoord>,
}

impl Model {
    pub fn write_to_disk(&self, file: &mut File) -> Result<(), String> {
        //bincode::serialize_into(file, self)
        todo!("not implemented");
    }

    pub fn get_vertices(&self) -> &[NormalVertex] {
        match &self.mesh {
            Mesh::NormalMesh(vertices) => {
                vertices.as_slice()
            },
            _ => {
                panic!("only normal vertices supported")
            },
        }
    }
}
