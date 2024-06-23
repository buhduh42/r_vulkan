use std::{
    collections::HashMap, fs::read_to_string, path::{
        Path, PathBuf
    },
};

use regex::Regex;

use super::Importer;

const WAVEFRONT_MAT_EXTENSION: &str = "mtl";

use crate::{
    model::{
        material::Material, 
        IndexCoord, 
        IndexVector, 
        DEFAULT_MODEL_NAME,
        Mesh, Model, NormalVector, 
        NormalVertex, PositionVector, TextureVector,
    },
    RenderResult,
};

pub struct Wavefront{
    pos: Vec<PositionVector>,
    uv: Vec<TextureVector>,
    norm: Vec<NormalVector>,
    name: String,
}

fn mat_file_from_obj_file(obj_file: &str) -> Option<PathBuf> {
    let mut mat_file = PathBuf::from(obj_file);
    mat_file.set_extension(WAVEFRONT_MAT_EXTENSION);
    if !Path::new(&mat_file).exists() {
        return None;
    }
    Some(mat_file)
}

impl Wavefront {
    pub fn new(name: Option<String>) -> Self {
        Self{
            name: name.unwrap_or_else(|| DEFAULT_MODEL_NAME.to_string()),
            pos: Vec::new(),
            uv: Vec::new(),
            norm: Vec::new(),
        }
    }

    fn load_position_vector<'a, I>(&mut self, vals: I) -> Result<(), String>
            where I: Iterator<Item = &'a String> {
        let re = Regex::new(r"^v (?P<x>-?\d+\.\d+) (?P<y>-?\d+\.\d+) (?P<z>-?\d+\.\d+)$")
            .unwrap(); //should never panic as this pattern is hardcoded
        self.pos = vals.map(|p| {
            if let Some(cap) = re.captures(p.trim()) {
                let (x, y, z) = (&cap["x"], &cap["y"], &cap["z"]);
                Ok(PositionVector::new(
                    x.parse().unwrap(),
                    y.parse().unwrap(),
                    z.parse().unwrap(),
                    1.0,
                ))
            } else {
                Err(format!("Unable to parse wavefront position vector: {}", p))
            }
        }).collect::<Result<Vec<PositionVector>, String>>()?;
        Ok(())
    }

    fn load_texture_vector<'a, I>(&mut self, vals: I) -> Result<(), String>
            where I: Iterator<Item = &'a String> {
        let re = Regex::new(r"^vt (?P<u>-?\d+\.\d+) (?P<v>-?\d+\.\d+)$")
            .unwrap(); //should never panic as this pattern is hardcoded
        self.uv = vals.map(|t| {
            if let Some(cap) = re.captures(t.trim()) {
                let (u, v) = (&cap["u"], &cap["v"]);
                Ok(TextureVector::new(
                    u.parse().unwrap(), v.parse().unwrap(),
                ))
            } else {
                Err(format!("Unable to parse wavefront texture vector: {}", t))
            }
        }).collect::<Result<Vec<TextureVector>, String>>()?;
        Ok(())
    }

    fn load_normal_vector<'a, I>(&mut self, vals: I) -> Result<(), String>
            where I: Iterator<Item = &'a String> {
        let re = Regex::new(r"^vn (?P<x>-?\d+\.\d+) (?P<y>-?\d+\.\d+) (?P<z>-?\d+\.\d+)$")
            .unwrap(); //should never panic as this pattern is hardcoded
        self.norm = vals.map(|n| {
            if let Some(cap) = re.captures(n.trim()) {
                let (x, y, z) = (&cap["x"], &cap["y"], &cap["z"]);
                Ok(NormalVector::new(
                    x.parse().unwrap(),
                    y.parse().unwrap(),
                    z.parse().unwrap(),
                    1.0,
                ))
            } else {
                Err(format!("Unable to parse wavefront normal vector: {}", n))
            }
        }).collect::<Result<Vec<NormalVector>, String>>()?;
        Ok(())
    }

    //Only supports model::Mesh(NormalMesh) for now
    //Once the model is generated, the memory will be all over the place
    //copy the model's data so it's cache friendly
    //this call isn't in the critical loop, but what it generates WILL be
    //not sure about the above anymore...will need to run it through a debugger
    //though may be good at some point to write a defragger for Model
    fn generate_model<'a, I>(&self, vals: I) -> RenderResult<Model>
            where I: Iterator<Item = &'a String> {
        let vert_1 = r"(?P<vert_1>(?P<pos_1>\d+)/(?P<tex_1>\d+)/(?P<norm_1>\d+))";
        let vert_2 = r"(?P<vert_2>(?P<pos_2>\d+)/(?P<tex_2>\d+)/(?P<norm_2>\d+))";
        let vert_3 = r"(?P<vert_3>(?P<pos_3>\d+)/(?P<tex_3>\d+)/(?P<norm_3>\d+))";
        let re = Regex::new(format!(r"^f {vert_1} {vert_2} {vert_3}$").as_str())
            .unwrap(); //should never panic as this pattern is hardcoded
        let mut face_map: HashMap<String, IndexCoord> = HashMap::new();
        let mut indeces: IndexVector = vec![];
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
            name: self.name.clone(),
            materials: vec!(),
        })
    }

    //only supporting a single, optional(Result) material for now
    //this sucks, but just need to keep moving on, I can clean up the model stuff later
    //basically, wanted to have Model.materials == Vec<Material>
    //where Material.texture == TextureImage, but can't get a Material out of the Vec
    //without cloning, need to think about this some
    fn parse_mat_file(&self, mat_file: &PathBuf) -> RenderResult<Material> {
        let lines = read_to_string(mat_file).map_err(|e| e.to_string())?;
        for l in lines.lines() {
            let to_check: &str = l.trim();
            //let to_ret: Material;
            if to_check.starts_with("map_Kd ") {
                if let Some((_, path)) = to_check.split_once(char::is_whitespace) {
                    if let Ok(mat) = Material::new(&PathBuf::from(path)) {
                        return Ok(mat);
                    };
                }
            }

        }
        //don't fucking care, shut the fuck up and print it
        let file_tex = mat_file.to_string_lossy();
        Err(format!("Could not parse material file: {file_tex}"))
    }

}

enum WavefrontLineType {
    Position,
    Texture,
    Normal,
    Face,
    Name,
}

impl WavefrontLineType {
    fn get(line: &str) -> Option<Self> {
        if line.trim().starts_with("v ") {
            return Some(Self::Position);
        } else if line.trim().starts_with("vn ") {
            return Some(Self::Normal);
        } else if line.trim().starts_with("vt ") {
            return Some(Self::Texture);
        } else if line.trim().starts_with("f ") {
            return Some(Self::Face);
        } else if line.trim().starts_with("o ") {
            return Some(Self::Name);
        }
        None
    }
}

impl Importer for Wavefront {
    //only doing textures for now
    fn generate_model(&self, loc: &str) -> RenderResult<Model> {
        let mut pos_vec: Vec<String> = vec![];
        let mut text_vec: Vec<String> = vec![];
        let mut norm_vec: Vec<String> = vec![];
        let mut face_vec: Vec<String> = vec![];
        let mut name_opt: Option<String> = None;
        let name_re = Regex::new(r"^o (?P<name>\w+)\s*$").map_err(|e| {
            format!(
                "failed generating regex for a wavefront model parser, with error: {e}")
                .to_string()
            }
        )?;
        let lines = read_to_string(loc).map_err(|e| e.to_string())?;
        lines.lines().for_each(|l| {
            if let Some(line_type) = WavefrontLineType::get(l) {
                let line = l.to_string();
                match line_type {
                    WavefrontLineType::Position => {
                        pos_vec.push(line);
                    },
                    WavefrontLineType::Texture => {
                        text_vec.push(line);
                    },
                    WavefrontLineType::Normal => {
                        norm_vec.push(line);
                    },
                    WavefrontLineType::Face => {
                        face_vec.push(line);
                    },
                    WavefrontLineType::Name => {
                        if let Some(cap) = name_re.captures(&line) {
                            name_opt = Some(cap["name"].to_string());
                        }
                    },
                }
            }
        });
        if name_opt.is_none() {
            return Err(
                String::from("Could not deternmine model name from passed iterator")
            );
        }
        let mut wavefront = Self::new(name_opt);
        if let Err(res) = wavefront.load_position_vector(pos_vec.iter()) {
            return Err(res);
        }
        if let Err(res) = wavefront.load_normal_vector(norm_vec.iter()) {
            return Err(res);
        }
        if let Err(res) = wavefront.load_texture_vector(text_vec.iter()) {
            return Err(res);
        }
        let mut model = wavefront.generate_model(face_vec.iter())?;
        if let Some(mat_file) = mat_file_from_obj_file(loc) {
            model.materials = vec!(self.parse_mat_file(&mat_file)?);
        }
        Ok(model)
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
                .as_str()).to_string();
        let mut wavefront = Wavefront::new(None);
        //let _ = wavefront.load_position_vector(data.lines()).unwrap();
        let to_parse: Vec<String> = data.lines().map(|l| l.to_string()).collect();
        let _ = wavefront.load_position_vector(to_parse.iter()).unwrap();
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
        let mut wavefront = Wavefront::new(None);
        let to_parse: Vec<String> = data.lines().map(|l| l.to_string()).collect();
        let _ = wavefront.load_texture_vector(to_parse.iter()).unwrap();
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
        let mut wavefront = Wavefront::new(None);
        let to_parse: Vec<String> = data.lines().map(|l| l.to_string()).collect();
        let _ = wavefront.load_normal_vector(to_parse.iter()).unwrap();
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
        let mut wavefront = Wavefront::new(None);

        let to_pos_parse: Vec<String> = pos_data.lines().map(|l| l.to_string()).collect();
        wavefront.load_position_vector(to_pos_parse.iter()).unwrap();

        let to_tex_parse: Vec<String> = tex_data.lines().map(|l| l.to_string()).collect();
        wavefront.load_texture_vector(to_tex_parse.iter()).unwrap();

        let to_norm_parse: Vec<String> = norm_data.lines()
            .map(|l| l.to_string()).collect();
        wavefront.load_normal_vector(to_norm_parse.iter()).unwrap();

        let to_index_parse: Vec<String> = index_data.lines()
            .map(|l| l.to_string()).collect();
        let model = wavefront.generate_model(to_index_parse.iter()).unwrap();

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
        let mut wavefront = Wavefront::new(None);
        let to_parse: Vec<String> = data.lines().map(|l| l.to_string()).collect();
        let _ = wavefront.load_position_vector(to_parse.iter()).unwrap();
    }

    #[test]
    #[should_panic(expected = "0.593750 0.500000")]
    fn wavefront_bad_textures() {
        let file_name = "wavefront_bad_textures.txt";
        let data = read_to_string(format!("{TEST_DIRECTORY}/{file_name}"))
            .expect(format!("Could not open 'testdata/{file_name}' for reading.")
                .as_str());
        let mut wavefront = Wavefront::new(None);
        let to_parse: Vec<String> = data.lines().map(|l| l.to_string()).collect();
        let _ = wavefront.load_texture_vector(to_parse.iter()).unwrap();
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
        let mut wavefront = Wavefront::new(None);
        let to_parse: Vec<String> = data.lines().map(|l| l.to_string()).collect();
        let _ = wavefront.load_normal_vector(to_parse.iter()).unwrap();
        assert_eq!(wavefront.norm.len(), 34, "norm vector length");
        assert_eq!(wavefront.norm[13], glm::Vector4::new(-0.4714, -0.0, -0.8819, 1.0));
        assert_eq!(
            wavefront.norm[21], glm::Vector4::new(0.8819, -0.0, -0.4714, 1.0),
        );
    }
}
