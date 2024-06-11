use crate::asset::Asset;

pub mod local_file;

pub trait AssetSource {
    fn save(&mut self, assets: Vec<Asset>) -> Result<(), String>;
}
