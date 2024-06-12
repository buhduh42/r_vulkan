use core::fmt;

pub mod path_defs;

pub enum AssetType {
    Model(Option<ModelType>),
    Texture,
}

#[derive(Debug)]
pub enum ModelType {
    Wavefront,
}

impl fmt::Display for ModelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const ID_ATTRIBUTE: &str = "id";
pub const LOCATION_ATTRIBUTE: &str = "location";
pub const NAME_ATTRIBUTE: &str = "name";
pub const TYPE_ATTRIBUTE: &str = "type";
pub const SUB_TYPE_ATTRIBUTE: &str = "sub_type";
pub const MODEL_TYPE: &str = "model";
pub const TEXTURE_TYPE: &str = "texture";

pub struct Asset {
    pub location: Option<String>,
    pub asset_type: AssetType,
    pub name: String,
    pub id: String,
}
