use std::io::{
    Write,
};

use xml::{
    name::Name, 
    writer::{
        EmitterConfig,
        XmlEvent,
    },
};

use crate::{
    asset::{
        Asset, AssetType, LOCATION_ATTRIBUTE, MODEL_TYPE, NAME_ATTRIBUTE, SUB_TYPE_ATTRIBUTE, TEXTURE_TYPE, TYPE_ATTRIBUTE
    }, source::AssetSource
};

pub struct LocalFile {
    location: Box<dyn Write>,
}

impl LocalFile {
    pub fn new(location: Box<dyn Write>) -> Self {
        Self{location}
    }
}

//was imagining a slightly more complicated structure at first, xml may not be
//the best choice after all...
impl AssetSource for LocalFile {
    //this method is pure dog shit!!!
    //fn save(&mut self, assets: Vec<Asset>) -> Result<(), String> {
    fn save(&mut self, assets: Vec<Asset>) -> Result<(), String> {
        let loc = &mut *self.location;
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(loc);
        for asset in assets {
            let sub_type: Option<String>;
            let a_type: &str = match asset.asset_type {
                AssetType::Model(sub_type_opt) => {
                    sub_type = match sub_type_opt {
                        Some(model_type) => {
                            Some(format!("{model_type}").to_lowercase())
                        }
                        None => None,
                    };
                    MODEL_TYPE
                },
                AssetType::Texture => {
                    sub_type = None;
                    TEXTURE_TYPE
                },
            };
            let id: &str = &asset.id;
            let loc = asset.location.ok_or(
                format!("location required for asset: {id}")
            )?;
            /*
            let start = XmlEvent::start_element(id)
                .attr(NAME_ATTRIBUTE, &asset.name)
                .attr(LOCATION_ATTRIBUTE, &loc)
                .attr(TYPE_ATTRIBUTE, a_type);
            */
            //this is fucking retarded, but fuck the borrow checker....
            //basically start is being consumed and then i can't call write...
            let sub_type_str: String;
            let start = match sub_type {
                Some(tmp_sub_type_str) => {
                    sub_type_str = tmp_sub_type_str;
                    XmlEvent::start_element(id)
                        .attr(NAME_ATTRIBUTE, &asset.name)
                        .attr(LOCATION_ATTRIBUTE, &loc)
                        .attr(TYPE_ATTRIBUTE, a_type)
                        .attr(SUB_TYPE_ATTRIBUTE, &sub_type_str)
                },
                None => {
                    XmlEvent::start_element(id)
                        .attr(NAME_ATTRIBUTE, &asset.name)
                        .attr(LOCATION_ATTRIBUTE, &loc)
                        .attr(TYPE_ATTRIBUTE, a_type)
                },
            };
            //was going to get fancy and convert the raw_os_error: i32 into a meaningful
            //message, got too far into the weeds for something this basic
            if let Err(_) = writer.write(start) {
                return Err(format!("Could not write XML Element: {id}"));
            }
            //maybe kinda silly, but i'll probably fill it with stuff
            let end_name = Name{
                local_name: &id,
                namespace: None,
                prefix: None,
            };
            let stop = XmlEvent::EndElement{name: Some(end_name)};
            if let Err(_) = writer.write(stop) {
                return Err(format!("Could not write XML Element: {id}"));
            }
        }
        Ok(())
    }
}
