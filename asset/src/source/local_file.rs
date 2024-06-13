use std::{
    borrow::BorrowMut,
    io::{
        Write,
        BufReader,
    },
    path::Path,
    fs::File,
};

use xml::{
    name::Name, 
    reader::{
        XmlEvent as XmlReadEvent,
        EventReader,
    },
    writer::{
        EmitterConfig,
        XmlEvent as EventWriter,
    },
};

use crate::{
    asset::{
        Asset, AssetType, ID_ATTRIBUTE, LOCATION_ATTRIBUTE, MODEL_TYPE, NAME_ATTRIBUTE, SUB_TYPE_ATTRIBUTE, TEXTURE_TYPE, TYPE_ATTRIBUTE
    }, source::AssetSource
};

pub struct LocalFile {
    write_location: Option<Box<dyn Write>>,
    location: Option<String>, 
}

impl LocalFile {
    pub fn new(location: Box<dyn Write>) -> Self {
        Self{
            write_location: Some(location),
            location: None,
        }
    }

    pub fn load(manifest: &str) -> Result<Self, String> {
        let manifest_path = Path::new(manifest);
        if !manifest_path.exists() {
            return Err(format!("manifest file: '{manifest}' does not exist"));
        }
        Ok(
            Self{
                write_location: None,
                location: Some(manifest.to_string()),
            }
        )
    }
}

//was imagining a slightly more complicated structure at first, xml may not be
//the best choice after all...
impl AssetSource for LocalFile {
    //this method is pure dog shit!!!
    //fn save(&mut self, assets: Vec<Asset>) -> Result<(), String> {
    fn save(&mut self, assets: Vec<Asset>) -> Result<(), String> {
        let loc = match self.write_location.borrow_mut() {
            Some(location) => location,
            None => {
                return Err(
                    "write location must not be None and implement dyn Write"
                    .to_string(),
                );
            },
            //&mut *self.location;
        };
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
            //this is fucking retarded, but fuck the borrow checker....
            //basically start is being consumed and then i can't call write...
            let sub_type_str: String;
            let start = match sub_type {
                Some(tmp_sub_type_str) => {
                    sub_type_str = tmp_sub_type_str;
                    EventWriter::start_element(id)
                        .attr(NAME_ATTRIBUTE, &asset.name)
                        .attr(LOCATION_ATTRIBUTE, &loc)
                        .attr(TYPE_ATTRIBUTE, a_type)
                        .attr(SUB_TYPE_ATTRIBUTE, &sub_type_str)
                },
                None => {
                    EventWriter::start_element(id)
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
            let stop = EventWriter::EndElement{name: Some(end_name)};
            if let Err(_) = writer.write(stop) {
                return Err(format!("Could not write XML Element: {id}"));
            }
        }
        Ok(())
    }

    //this too is dog shit!!!
    fn get_by_id(&self, id: &str) -> Result<Asset, String> {
        let location = match &self.location {
            Some(read_loc) => read_loc,
            None => {
                return Err("manifest location unknown".to_string());
            }
        };
        let manifest = match File::open(location) {
            Ok(man_file) => man_file,
            Err(err) => {
                return Err(
                    format!("error: '{err}' when opening manifest file: '{location}'")
                );
            }
        };
        let manifest = BufReader::new(manifest);
        //not quite sure why it wasn't finding , don't really care
        let parser = EventReader::new(manifest);
        let mut to_ret: Option<Asset> = None;
        for e in parser {
            match e {
                Ok(event) => {
                    match event {
                        XmlReadEvent::StartElement { name, attributes, .. } => {
                            if name.local_name == id {
                                let mut asset_name: &str = "";
                                let asset_id = id;
                                let mut asset_type_str: &str = "";
                                let mut asset_sub_type_str: &str = "";
                                let mut asset_location: &str = "";
                                for (i, attr) in attributes.iter().enumerate() {
                                    let attr_name: &str = &attr.name.local_name;
                                    //probably not the best way to do this,
                                    //might miss one....
                                    match attr_name {
                                        NAME_ATTRIBUTE => {
                                            asset_name = &attributes[i].value;
                                        },
                                        ID_ATTRIBUTE => {
                                            if asset_id != attr.value {
                                                let tmp_id = &attr.value;
                                                return Err(format!(
                                                    "id tag: '{asset_id}' does not match attribute id: '{tmp_id}'"));
                                            }
                                        },
                                        LOCATION_ATTRIBUTE => {
                                             asset_location = 
                                                 &attributes[i].value;
                                        },
                                        TYPE_ATTRIBUTE => {
                                            asset_type_str = 
                                                &attributes[i].value;
                                        },
                                        SUB_TYPE_ATTRIBUTE => {
                                            asset_sub_type_str = 
                                                &attributes[i].value;
                                        },
                                        _ => {},
                                    }
                                }
                                to_ret = Some(
                                    Asset::new(
                                        asset_location, asset_id, 
                                        asset_name, asset_type_str, 
                                        asset_sub_type_str,
                                    )?
                                );
                                break;
                            } else {
                                continue;
                            }
                        },
                        _ => {},
                    }
                },
                Err(err) => {
                    return Err(format!("error reading manifest file, '{err}'"));
                },
            }
        }
        to_ret.ok_or(format!("could not find asset for id: {id}"))
    }
}
