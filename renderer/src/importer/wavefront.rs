use std::{
    collections::HashMap,
    fs::File,
};

use regex::Regex;

use super::Importer;

use crate::model::{
    IndexCoord, Model, NormalCoord, NormalVertex, 
    PositionCoord, TextureCoord, Vector2, Vector4,
    Mesh,
};

pub struct Wavefront{
    pos: Vec<Vector4<PositionCoord>>,
    uv: Vec<Vector2<TextureCoord>>,
    norm: Vec<Vector4<NormalCoord>>,
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
        self.pos = vals.map(|p| {
            if let Some(cap) = re.captures(p.trim()) {
                let (x, y, z) = (&cap["x"], &cap["y"], &cap["z"]);
                Ok(Vector4::new(
                    x.parse::<PositionCoord>().unwrap(), //panic impossible
                    y.parse::<PositionCoord>().unwrap(), //panic impossible
                    z.parse::<PositionCoord>().unwrap(), //panic impossible
                    1.0,
                ))
            } else {
                Err(format!("Unable to parse wavefront position vector: {}", p))
            }
        }).collect::<Result<Vec<Vector4<PositionCoord>>, String>>()?;
        Ok(())
    }

    fn load_texture_vector<'a, I>(&mut self, vals: I) -> Result<(), String>
            where I: Iterator<Item = &'a str> {
        let re = Regex::new(r"^vt (?P<u>-?\d+\.\d+) (?P<v>-?\d+\.\d+)$")
            .unwrap(); //should never panic as this pattern is hardcoded
        self.uv = vals.map(|t| {
            if let Some(cap) = re.captures(t.trim()) {
                let (u, v) = (&cap["u"], &cap["v"]);
                Ok(Vector2::new(
                    u.parse::<PositionCoord>().unwrap(), //panic impossible
                    v.parse::<PositionCoord>().unwrap(), //panic impossible
                ))
            } else {
                Err(format!("Unable to parse wavefront texture vector: {}", t))
            }
        }).collect::<Result<Vec<Vector2<TextureCoord>>, String>>()?;
        Ok(())
    }

    fn load_normal_vector<'a, I>(&mut self, vals: I) -> Result<(), String>
            where I: Iterator<Item = &'a str> {
        let re = Regex::new(r"^vn (?P<x>-?\d+\.\d+) (?P<y>-?\d+\.\d+) (?P<z>-?\d+\.\d+)$")
            .unwrap(); //should never panic as this pattern is hardcoded
        self.norm = vals.map(|n| {
            if let Some(cap) = re.captures(n.trim()) {
                let (x, y, z) = (&cap["x"], &cap["y"], &cap["z"]);
                Ok(Vector4::new(
                    x.parse::<NormalCoord>().unwrap(), //panic impossible
                    y.parse::<NormalCoord>().unwrap(), //panic impossible
                    z.parse::<NormalCoord>().unwrap(), //panic impossible
                    1.0,
                ))
            } else {
                Err(format!("Unable to parse wavefront normal vector: {}", n))
            }
        }).collect::<Result<Vec<Vector4<TextureCoord>>, String>>()?;
        Ok(())
    }

    //Only supports model::Mesh(NormalMesh) for now
    //Once the model is generated, the memory will be all over the place
    //copy the model's data so it's cache friendly
    //this call isn't in the critical loop, but what it generates WILL be
    //not sure about the above anymore...will need to run it through a debugger
    //though may be good at some point to write a defragger for Model
    fn generate_model<'a, I>(&self, vals: I) -> Result<Model, String>
            where I: Iterator<Item = &'a str> {
        let vert_1 = r"(?P<vert_1>(?P<pos_1>\d+)/(?P<tex_1>\d+)/(?P<norm_1>\d+))";
        let vert_2 = r"(?P<vert_2>(?P<pos_2>\d+)/(?P<tex_2>\d+)/(?P<norm_2>\d+))";
        let vert_3 = r"(?P<vert_3>(?P<pos_3>\d+)/(?P<tex_3>\d+)/(?P<norm_3>\d+))";
        let re = Regex::new(format!(r"^f {vert_1} {vert_2} {vert_3}$").as_str())
            .unwrap(); //should never panic as this pattern is hardcoded
        let mut face_map: HashMap<String, IndexCoord> = HashMap::new();
        let mut indeces: Vec<IndexCoord> = vec![];
        let mut vertices: Vec<NormalVertex> = vec![];
        let mut update_fn = |cap: &regex::Captures, i: i32| {
            if face_map.contains_key(&cap[format!("vert_{i}").as_str()]) {
                //panic impossible
                indeces.push(
                    *(face_map.get(&cap[format!("vert_{i}").as_str()]).unwrap())
                );
            } else {
                let p_index: usize = (&cap[format!("pos_{i}").as_str()])
                    .parse().unwrap(); //panic impossible
                let t_index: usize = (&cap[format!("tex_{i}").as_str()])
                    .parse().unwrap(); //panic impossible
                let n_index: usize = (&cap[format!("norm_{i}").as_str()])
                    .parse().unwrap(); //panic impossible
                vertices.push(NormalVertex::new(
                    self.pos[p_index - 1],
                    self.uv[t_index - 1],
                    self.norm[n_index - 1],
                ));
                let index: IndexCoord = (vertices.len() - 1).try_into().unwrap();
                face_map.insert(
                    cap[format!("vert_{i}").as_str()].to_string(), index,
                );
                indeces.push(index);
            }
        };
        for val in vals {
            if let Some(cap) = re.captures(val.trim()) {
                update_fn(&cap, 1);
                update_fn(&cap, 2);
                update_fn(&cap, 3);
            } else {
                return Err(format!("Unable to parse wavefront index for {val}, exiting"));
            }
        }
        Ok(Model{
            mesh: Mesh::NormalMesh(vertices),
            indeces,
        })
    }

    fn get_position_iterator<'a, I>(&self, f: &File) -> Result<I, String>
            where I: Iterator<Item = &'a str> {
        todo!("not implemented");
    }

    fn get_texture_iterator<'a, I>(&self, f: &File) -> Result<I, String>
            where I: Iterator<Item = &'a str> {
        todo!("not implemented");
    }

    fn get_normal_iterator<'a, I>(&self, f: &File) -> Result<I, String>
            where I: Iterator<Item = &'a str> {
        todo!("not implemented");
    }

    fn get_index_iterator<'a, I>(&self, f: &File) -> Result<I, String>
            where I: Iterator<Item = &'a str> {
        todo!("not implemented");
    }

}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::{
        importer::{
            wavefront::Wavefront,
            Importer,
        },
        model::{
            Vector2,
            Vector4,
        },
    };

    const TEST_DIRECTORY: &str = "src/importer/testdata";

    #[test]
    fn wavefront_good_positions() {
        let file_name = "wavefront_good_positions.txt";
        let data = read_to_string(format!("{TEST_DIRECTORY}/{file_name}"))
            .expect(format!("Could not open 'testdata/{file_name}' for reading.")
                .as_str());
        let mut wavefront = Wavefront::new();
        let _ = wavefront.load_position_vector(data.lines()).unwrap();
        assert_eq!(wavefront.pos.len(), 64, "pos vector length");
        assert_eq!(wavefront.pos[0], Vector4::new(0.0, 3.080803, 3.080803, 1.0));
        assert_eq!(
            wavefront.pos[25], Vector4::new(-2.178457, -3.080803, -2.178457, 1.0),
        );
    }

    #[test]
    fn wavefront_good_textures() {
        let file_name = "wavefront_good_textures.txt";
        let data = read_to_string(format!("{TEST_DIRECTORY}/{file_name}"))
            .expect(format!("Could not open 'testdata/{file_name}' for reading.")
                .as_str());
        let mut wavefront = Wavefront::new();
        let _ = wavefront.load_texture_vector(data.lines()).unwrap();
        assert_eq!(wavefront.uv.len(), 130, "uv vector length");
        assert_eq!(wavefront.uv[22], Vector2::new(0.656250, 0.5));
        assert_eq!(wavefront.uv[86], Vector2::new(0.203178, 0.014612));
    }

    #[test]
    fn wavefront_good_normals() {
        let file_name = "wavefront_good_normals.txt";
        let data = read_to_string(format!("{TEST_DIRECTORY}/{file_name}"))
            .expect(format!("Could not open 'testdata/{file_name}' for reading.")
                    .as_str());
        let mut wavefront = Wavefront::new();
        let _ = wavefront.load_normal_vector(data.lines()).unwrap();
        assert_eq!(wavefront.norm.len(), 34, "norm vector length");
        assert_eq!(wavefront.norm[13], Vector4::new(-0.4714, -0.0, -0.8819, 1.0));
        assert_eq!(
            wavefront.norm[21], Vector4::new(0.8819, -0.0, -0.4714, 1.0),
        );
    }

    #[test]
    //requires proper position, texture, and normal loading
    //I could hardcode those, but meh
    //This still might be wrong...might need to test better
    fn wavefront_good_indeces() {
        let index_file = "wavefront_good_indeces.txt";
        let pos_file = "wavefront_good_positions.txt";
        let tex_file = "wavefront_good_textures.txt";
        let norm_file = "wavefront_good_normals.txt";
        let index_data = read_to_string(format!("{TEST_DIRECTORY}/{index_file}"))
            .expect(format!("Could not open 'testdata/{index_file}' for reading.")
                .as_str());
        let pos_data = read_to_string(format!("{TEST_DIRECTORY}/{pos_file}"))
            .expect(format!("Could not open 'testdata/{pos_file}' for reading.")
                .as_str());
        let tex_data = read_to_string(format!("{TEST_DIRECTORY}/{tex_file}"))
            .expect(format!("Could not open 'testdata/{tex_file}' for reading.")
                    .as_str());
        let norm_data = read_to_string(format!("{TEST_DIRECTORY}/{norm_file}"))
            .expect(format!("Could not open 'testdata/{norm_file}' for reading.")
                .as_str());
        let mut wavefront = Wavefront::new();
        wavefront.load_position_vector(pos_data.lines()).unwrap();
        wavefront.load_texture_vector(tex_data.lines()).unwrap();
        wavefront.load_normal_vector(norm_data.lines()).unwrap();
        let model = wavefront.generate_model(index_data.lines()).unwrap();
        assert_eq!(model.indeces.len(), 124*3);
        //6/64/31 first occurrence is 91st, 192nd
        //43/43/21 62nd, 163rd
        //wavefront.pos[5], wavefront.tex[63], wavefront.norm[30]
    }

    #[test]
    #[should_panic(expected = "yolo")]
    fn wavefront_bad_positions() {
        let file_name = "wavefront_bad_positions.txt";
        let data = read_to_string(format!("{TEST_DIRECTORY}/{file_name}"))
            .expect(format!("Could not open 'testdata/{file_name}' for reading.")
                .as_str());
        let mut wavefront = Wavefront::new();
        let _ = wavefront.load_position_vector(data.lines()).unwrap();
    }

    #[test]
    #[should_panic(expected = "0.593750 0.500000")]
    fn wavefront_bad_textures() {
        let file_name = "wavefront_bad_textures.txt";
        let data = read_to_string(format!("{TEST_DIRECTORY}/{file_name}"))
            .expect(format!("Could not open 'testdata/{file_name}' for reading.")
                .as_str());
        let mut wavefront = Wavefront::new();
        let _ = wavefront.load_texture_vector(data.lines()).unwrap();
        assert_eq!(wavefront.uv.len(), 130, "uv vector length");
        assert_eq!(wavefront.uv[22], glm::Vector2::new(0.656250, 1.0));
        assert_eq!(wavefront.uv[86], glm::Vector2::new(0.485388, 0.203178));
    }

    #[test]
    #[should_panic(expected = "blarg")]
    fn wavefront_bad_normals() {
        let file_name = "wavefront_bad_normals.txt";
        let data = read_to_string(format!("{TEST_DIRECTORY}/{file_name}"))
            .expect(format!("Could not open 'testdata/{file_name}' for reading.")
                    .as_str());
        let mut wavefront = Wavefront::new();
        let _ = wavefront.load_normal_vector(data.lines()).unwrap();
        assert_eq!(wavefront.norm.len(), 34, "norm vector length");
        assert_eq!(wavefront.norm[13], glm::Vector4::new(-0.4714, -0.0, -0.8819, 1.0));
        assert_eq!(
            wavefront.norm[21], glm::Vector4::new(0.8819, -0.0, -0.4714, 1.0),
        );
    }
}
