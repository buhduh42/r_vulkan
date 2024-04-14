use std::error::Error;
use std::fmt::write;

use regex::Regex;

use super::Importer;

use crate::model::vertex::{
    PositionCoord,
    TextureCoord,
    NormalCoord,
};

pub struct Wavefront{
    pos: Vec<glm::Vector4<PositionCoord>>,
    uv: Vec<glm::Vector2<TextureCoord>>,
    norm: Vec<glm::Vector4<NormalCoord>>,
}

impl Wavefront {
    pub fn new() -> Self {
        Self{
            pos: Vec::new(),
            uv: Vec::new(),
            norm: Vec::new(),
        }
    }
}

//TODO(errors)
impl Importer for Wavefront {
    fn load_position_vector<'a, I>(&mut self, vals: I) -> Result<(), String>
            where I: Iterator<Item = &'a str> {
        let re = Regex::new(r"^v (?P<x>-?\d+\.\d+) (?P<y>-?\d+\.\d+) (?P<z>-?\d+\.\d+)$")
            .unwrap(); //should never panic as this pattern is hardcoded
        vals.map(|p| {
            if let Some(cap) = re.captures(p.trim()) {
                let (x, y, z) = (&cap["x"], &cap["y"], &cap["z"]);
                return Ok(glm::Vector4::new(
                    x.parse::<PositionCoord>().unwrap(), //panic impossible
                    y.parse::<PositionCoord>().unwrap(), //panic impossible
                    z.parse::<PositionCoord>().unwrap(), //panic impossible
                    1.0,
                ));
            } else {
                return Err(format!("Unable to parse wavefront position vector: {}", p));
            }
        }).collect::<Result<Vec<glm::Vector4<PositionCoord>>, String>>();
        todo!("huh");
    }
    fn load_texture_vector<'a, I>(&mut self, vals: I) -> Result<(), Box<dyn Error>>
            where I: Iterator<Item = &'a str> {
        todo!("not implemented");
    }
    fn load_normal_vector<'a, I>(&mut self, vals: I) -> Result<(), Box<dyn Error>>
            where I: Iterator<Item = &'a str> {
        todo!("not implemented");
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::importer::wavefront::Wavefront;
    use crate::importer::Importer;

    use std::env;

    const TEST_DIRECTORY: &str = "src/importer/testdata";

    #[test]
    fn wavefront_positions() {
        let data = read_to_string(format!("{TEST_DIRECTORY}/wavefront_positions.txt"))
            .expect("Could not open 'testdata/wavefront_positions.txt' for reading.");
        let mut wavefront = Wavefront::new();
        let _ = wavefront.load_position_vector(data.lines()).unwrap();
        println!("code is running from: {}", env::current_dir().unwrap().display());
        assert_eq!(wavefront.pos.len(), 63, "pos vector length");
        assert_eq!(wavefront.pos[0], glm::Vector4::new(0.0, 4.543867, 4.543867, 1.0));
        assert_eq!(
            wavefront.pos[25], glm::Vector4::new(-3.212999, -4.543867, -3.212999, 1.0),
        );
    }

    #[test]
    fn wavefront_textures() {
        let data = read_to_string(format!("{TEST_DIRECTORY}/wavefront_textures.txt"))
            .expect("Could not open 'testdata/wavefront_textures.txt' for reading.");
        let mut wavefront = Wavefront::new();
        let _ = wavefront.load_texture_vector(data.lines()).unwrap();
        assert_eq!(wavefront.uv.len(), 130, "uv vector length");
        assert_eq!(wavefront.uv[22], glm::Vector2::new(0.656250, 1.0));
        assert_eq!(wavefront.uv[86], glm::Vector2::new(0.485388, 0.203178));
    }

    #[test]
    fn wavefront_normals() {
        let data = read_to_string(format!("{TEST_DIRECTORY}/wavefront_normals.txt"))
            .expect("Could not open 'testdata/wavefront_normals.txt' for reading.");
        let mut wavefront = Wavefront::new();
        let _ = wavefront.load_normal_vector(data.lines()).unwrap();
        assert_eq!(wavefront.norm.len(), 34, "norm vector length");
        assert_eq!(wavefront.norm[13], glm::Vector4::new(-0.4714, -0.0, -0.8819, 1.0));
        assert_eq!(
            wavefront.pos[21], glm::Vector4::new(0.8819, -0.0, -0.4714, 1.0),
        );
    }
}
