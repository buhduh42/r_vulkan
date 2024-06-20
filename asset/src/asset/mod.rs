use core::fmt;

pub mod path_defs;

pub enum AssetType {
    Model(ModelType),
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
pub const WAVEFRONT_TYPE: &str = "wavefront";

pub struct Asset {
    pub location: Option<String>,
    pub asset_type: AssetType,
    pub name: String,
    pub id: String,
}

//this shit is awful!!!
//desperately need to redo this once i know what im doing
impl AssetType {
    fn new(tpe: &str, sub_type: &str) -> Result<Self, String> {
        if tpe == MODEL_TYPE {
            if sub_type == WAVEFRONT_TYPE {
                return Ok(
                    AssetType::Model(ModelType::Wavefront)
                );
            }
            return Err("no subtype found for asset type: 'model'".to_string());
        }
        if tpe == TEXTURE_TYPE {
            return Ok(AssetType::Texture);
        }
        Err(format!("unkonw asset type: '{tpe}'"))
    }
}

//this shit is awful!!!
//desperately need to redo this once i know what im doing
impl Asset {
    pub fn new(loc: &str, id: &str, name: &str, tpe: &str, sub_type: &str) -> 
            Result<Self, String> {
        let asset_type = AssetType::new(tpe, sub_type)?;
        let location = if loc == "" {None} else {Some(loc.to_string())};
        Ok(Asset{
            location,
            asset_type,
            name: name.to_string(),
            id: id.to_string(),
        })
    }
}
