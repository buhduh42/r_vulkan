/* A model can have more than one material.
 * Only using textures for now, and only supporting
 * a single material for now.
 * Might have to write some sort of "(Texture|Image)Manager" or something
 * if I end up having a bunch of duplicates, good enough for now.
 */

use std::{
    fs::{
        metadata, File
    }, io::Read, path::PathBuf,
    rc::{Rc, Weak},
};

use image::RgbaImage;

use crate::RenderResult;

pub type TextureImage = RgbaImage;

pub struct Material {
    //do i want this to be public for direct modification?
    pub texture: TextureImage,
}

impl Material {
    pub fn new(tex_loc: &PathBuf) -> RenderResult<Material> {
        let mut file = File::open(&tex_loc).map_err(|e| e.to_string())?;
        let metadata = metadata(&tex_loc).map_err(|e| e.to_string())?;
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer).map_err(|e| e.to_string())?;
        let texture = image::load_from_memory(&buffer).map_err(|e| e.to_string())?
            .to_rgba8();
        Ok(
            Material{
                texture,
            }
        )
    }

    /*
    pub fn get_texture(&self) -> Weak<TextureImage> {
        Rc::downgrade(&self.texture)
    }
    */
}
