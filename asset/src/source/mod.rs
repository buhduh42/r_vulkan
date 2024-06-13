use crate::asset::Asset;

pub mod local_file;

pub trait AssetSource {
    fn save(&mut self, assets: Vec<Asset>) -> Result<(), String>;
    fn get_by_id(&self, id: &str) -> Result<Asset, String>;
}
