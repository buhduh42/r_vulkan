use clap::Parser;

use renderer::{
    window::Window,
    vulkan::Vulkan,
    model::model_manager::ModelManager,
};

use asset::source::{
    local_file::LocalFile,
    AssetSource,
};

/// Main renderer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)] 
    assets_manifest: String,

    #[arg(short, long)] 
    shader_dir: String,
}

fn get_asset_source(manifest: &str) -> Result<impl AssetSource, String> {
    Ok(LocalFile::load(manifest)?)
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let asset_source = get_asset_source(&args.assets_manifest)?;
    let mut model_manager = ModelManager::new(Box::new(asset_source));
    //TODO these shouldn't be called (Window|Vulkan)::new()
    let window = Window::new(1920, 1080, None).unwrap();
    let vulkan = Vulkan::new(&window).unwrap();
    let draw = vulkan.get_draw_fn(&mut model_manager);
    let _ = window.render_loop(draw);
    Ok(())
}
