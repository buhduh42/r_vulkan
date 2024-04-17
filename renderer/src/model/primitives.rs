//TODO, need to formalize asset locations
use std::fs::read_to_string;

use crate::importer::wavefront::Wavefront;
use crate::importer::Importer;
use crate::model::{
    Model,
    Mesh,
    NormalVertex,
    PositionVector,
    TextureVector,
    NormalVector,
    IndexCoord,
};

//MUST be run from the crate director, eg renderer
pub enum Primitive {
    Sphere,
    Cylinder,
}

const SPHERE: &str = "../assets/models/wavefront/sphere.obj";
const CYLINDER: &str = "../assets/models/wavefront/cylinder.obj";

pub fn make_primitive(primitive: Primitive) -> Model {
    let path = match primitive {
        Primitive::Sphere => {SPHERE},
        Primitive::Cylinder => {CYLINDER},
    };

    let lines: Vec<String> = read_to_string(path)
        .expect(format!("Could not open in_file: '{path}' for reading.").as_str())
        .lines().map(|l| l.to_string()).collect();

    let parser = Wavefront::new(None);
    parser.generate_model(lines.iter()).unwrap()
}

pub fn hardcoded_square() -> Model {
    let mesh = Mesh::NormalMesh(vec![
        NormalVertex {
            pos: PositionVector::new(
                -1.0, -1.0, 0.0, 1.0
            ),
            uv: TextureVector::new(
                0.0, 0.0
            ),
            norm: NormalVector::new(
                -1.0, -1.0, 0.0, 1.0
            ),
        },
        NormalVertex {
            pos: PositionVector::new(
                -1.0, 1.0, 0.0, 1.0
            ),
            uv: TextureVector::new(
                0.0, 1.0
            ),
            norm: NormalVector::new(
                -1.0, -1.0, 0.0, 1.0
            ),
        },
        NormalVertex {
            pos: PositionVector::new(
                1.0, 1.0, 0.0, 1.0
            ),
            uv: TextureVector::new(
                1.0, 1.0
            ),
            norm: NormalVector::new(
                -1.0, -1.0, 0.0, 1.0
            ),
        },
        NormalVertex {
            pos: PositionVector::new(
                1.0, -1.0, 0.0, 1.0
            ),
            uv: TextureVector::new(
                1.0, 0.0
            ),
            norm: NormalVector::new(
                -1.0, -1.0, 0.0, 1.0
            ),
        },
    ]);
    let indeces: Vec<IndexCoord> = vec![0, 1, 2, 2, 3, 0];
    Model{
        name: String::from("who cares"),
        mesh,
        indeces,
    }
}
