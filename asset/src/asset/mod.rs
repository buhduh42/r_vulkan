pub mod path_defs;

pub enum AssetType {
    Model(Option<ModelType>),
    Texture,
}

pub enum ModelType {
    Wavefront,
}

pub const ID_ATTRIBUTE: &str = "id";
pub const LOCATION_ATTRIBUTE: &str = "location";
pub const NAME_ATTRIBUTE: &str = "name";
pub const TYPE_ATTRIBUTE: &str = "type";
pub const MODEL_TYPE: &str = "model";
pub const TEXTURE_TYPE: &str = "texture";

pub struct Asset {
    pub location: Option<String>,
    pub asset_type: AssetType,
    pub name: String,
    pub id: String,
}
