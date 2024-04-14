use std::error::Error;

pub mod wavefront;

pub trait Importer {
    fn load_position_vector<'a, I>(&mut self, vals: I) -> Result<(), String>
        where I: Iterator<Item = &'a str>;
    fn load_texture_vector<'a, I>(&mut self, vals: I) -> Result<(), Box<dyn Error>>
        where I: Iterator<Item = &'a str>;
    fn load_normal_vector<'a, I>(&mut self, vals: I) -> Result<(), Box<dyn Error>>
        where I: Iterator<Item = &'a str>;
}
