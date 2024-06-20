use crate::model::Model;

pub mod wavefront;

pub trait Importer {
    fn generate_model<'a, I>(&self, lines: I) -> Result<Model, String>
        where I: Iterator<Item = &'a String>;

    fn attach_material<'a, I>(&self, model: &mut Model, lines: I) -> Result<(), String>
        where I: Iterator<Item = &'a String>;
}
