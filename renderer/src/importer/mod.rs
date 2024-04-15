use std::fs::File;

use crate::model::Model;

pub mod wavefront;

pub trait Importer {
    fn load_position_vector<'a, I>(&mut self, vals: I) -> Result<(), String>
        where I: Iterator<Item = &'a str>;
    fn load_texture_vector<'a, I>(&mut self, vals: I) -> Result<(), String>
        where I: Iterator<Item = &'a str>;
    fn load_normal_vector<'a, I>(&mut self, vals: I) -> Result<(), String>
        where I: Iterator<Item = &'a str>;
    fn generate_model<'a, I>(&self, vals: I) -> Result<Model, String>
        where I: Iterator<Item = &'a str>;
    fn get_position_iterator<'a, I>(&self, f: &File) -> Result<I, String>
        where I: Iterator<Item = &'a str>;
    fn get_texture_iterator<'a, I>(&self, f: &File) -> Result<I, String>
        where I: Iterator<Item = &'a str>;
    fn get_normal_iterator<'a, I>(&self, f: &File) -> Result<I, String>
        where I: Iterator<Item = &'a str>;
    fn get_index_iterator<'a, I>(&self, f: &File) -> Result<I, String>
        where I: Iterator<Item = &'a str>;
}
