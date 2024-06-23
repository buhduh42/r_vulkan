use crate::model::Model;
use crate::RenderResult;

pub mod wavefront;

pub trait Importer {
    fn generate_model(&self, loc: &str) -> RenderResult<Model>;
}
