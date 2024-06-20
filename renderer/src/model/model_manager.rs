use std::{
    collections::HashMap, fs::read_to_string, rc::{
        Rc, Weak
    }
};

use asset::{
    source::AssetSource,
    asset::{
        AssetType,
        ModelType,
    },
};

use crate::importer::{wavefront::Wavefront, Importer};

use super::Model;

pub struct ModelManager {
    model_map: HashMap<String, Weak<Model>>,
    asset_source: Box<dyn AssetSource>,
}

impl ModelManager {
    pub fn new(asset_source: Box<dyn AssetSource>) -> Self {
        Self{
            model_map: HashMap::new(),
            asset_source,
        }
    }

    fn load_model(&mut self, id: &str) -> Result<Rc<Model>, String> {
        let asset = self.asset_source.get_by_id(id)?;
        let model_type = match asset.asset_type {
            AssetType::Model(model_type) => model_type,
            AssetType::Texture => {
                return Err(format!("asset is not a model for id: {id}"));   
            },
        };
        //wanted to make importer generic, but wasn't letting me do: Box<dyn Importer>
        //for some reason, concrete for now
        let importer = match model_type {
            ModelType::Wavefront => {
                Wavefront::new(Some(id.to_string()))
            },
        };
        if let None = asset.location {
            return Err(format!("location required to load model, id: {id}"));
        }
        let location = asset.location.unwrap();
        let lines: Vec<String> = match read_to_string(&location) {
            Ok(lines) => {
                lines.lines().map(|l| l.to_string()).collect()
            },
            Err(err) => {
                return Err(format!("got error: {err} when reading model file: '{location}', for id: {id}"));
            },
        };
        let model = importer.generate_model(lines.iter())?;
        Ok(Rc::new(model))
    }

    //not quite as pretty as the other one, but more efficient at least, i think?
    pub fn get_model_by_id(&mut self, id: &str) -> Result<Rc<Model>, String> {
        match self.model_map.get(id) {
            Some(model_ref) => {
                match model_ref.upgrade() {
                    Some(to_ret) => {
                        Ok(to_ret)
                    },
                    None => {
                        self.load_model(id)
                    },
                }
            },
            None => {
                self.load_model(id)
            },
        }
    }

    //probably not the most efficient, but good enough for now
    /*
    pub fn get_model_by_id(&mut self, id: &str) -> Result<Rc<Model>, String> {
        //i end up inserting twice if no key is found....
        if !self.model_map.contains_key(id) {
            self.model_map.insert(id.to_string(), Weak::new());
        }
        //unwrap should be safe here because of above
        let model_ref = self.model_map.get(id).unwrap();
        let to_ret = match model_ref.upgrade() {
            Some(to_ret) => to_ret,
            None => {
                let model = self.load_model(id)?;
                let to_ret = Rc::new(model); 
                self.model_map.insert(id.to_string(), Rc::downgrade(&to_ret));
                to_ret
            },
        };
        Ok(to_ret)
    }
    */

}
