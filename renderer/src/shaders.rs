//I'm over engineering this bs

use std::collections::HashMap;
use std::path::PathBuf;

use crate::RenderResult;

type StageData = Vec<u8>;

enum Shader {
    Vertex(StageData),
    Fragment(StageData),
}

struct PipelineData {
    vertex: Option<StageData>,
    fragment: Option<StageData>,
}

pub struct ShaderLoader {
    shader_dir: PathBuf,
    shaders: HashMap<String, PipelineData>
}

impl ShaderLoader {
    pub fn new(shader_path: &str) -> RenderResult<Self> {
        let shader_dir = PathBuf::from(shader_path);
        if !shader_dir.exists() {
            return Err(format!("shader directory does not exist: '{shader_path}'"));
        }
        Ok(Self{shader_dir})
    }
}
