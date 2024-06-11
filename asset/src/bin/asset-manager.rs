use std::{
    fs::{self, File}, io::{
        stdout, Write
    }, path::Path,
};

use clap::Parser;

use glob::glob;

use asset::{
    source::local_file::LocalFile,
    asset::{
        Asset,
        ModelType,
        AssetSubType,
        path_defs::{
            REL_MODEL_PATH,
            REL_TEXTURE_PATH,
            REL_WAVEFRONT_MODELS_PATH,
            WAVEFRONT_EXTENSION,
            TEXTURE_EXTENSION,
        },
    },
};

/*
 * probably a pipe dream, but if i ever integrate sqlite into this thing,
 * assets will need to be integrated into that with this tool as well,
 * for now, only supporting xml
 * might need to think a little about the -x flag, fine for now, but
 * is actually probably mutually exclusive with all other forms of output
 * TODO: need to unify an xml manifest writer AND whatever other stuff I support
 * eg, the sqlite writer and w/e else needs to be compatible with:
 * location: &'r mut dyn Write, as defined in the xml local file definition
 */

/// CLI tool to generate an XML(for now) manifest file of all assets
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Root directory of assets, locally, required
    #[arg(short, long)] 
    assets_directory: String,

    /// Superfluous for now, defaults to true, specifies output should be xml
    #[arg(short, long, action)]
    xml: bool,

    /// Where to write the xml manifest file use "-" for stdout
    #[arg(short, long, default_value = "-")] 
    manifest_file: String,
}

fn parse_assets_dir(assets_path: &Path) -> Result<Vec<Asset>, String> {
    let models_path = assets_path.join(REL_MODEL_PATH);
    let to_ret: Vec<Asset> = vec!();
    if models_path.exists() {
        let wavefront_path = models_path.join(REL_WAVEFRONT_MODELS_PATH);
        if wavefront_path.exists() {
            let wavefront_glob = wavefront_path.join(format!("*.{WAVEFRONT_EXTENSION}"));
            for entry in glob(wavefront_glob.to_str().unwrap()).unwrap() {
                //probably a cleaner if let syntax here...
                match entry {
                    Ok(path) => {
                        let name: String = path.file_stem().unwrap();
                        to_ret.push(
                            Asset{
                                location: Some(path.display().to_string()),
                                asset_type: AssetType::Model(Some(ModelType::Wavefront)),
                                name,
                                id: name,
                            },
                        );
                    },
                    Err(_) => {
                        return entry;
                    },
                }
            }
        }
    }
    let texture_path = assets_path.join(REL_TEXTURE_PATH);
    if texture_path.exists() {
        let texture_glob = texture_path.join(format!("*.{TEXTURE_EXTENSION}"));
        for entry in glob(texture_glob.to_str().unwrap()).unwrap() {
            //probably a cleaner if let syntax here...
            match entry {
                Ok(path) => {
                    let name: String = path.file_stem().unwrap();
                    to_ret.push(
                        Asset{
                            location: Some(path.display().to_string()),
                            asset_type: AssetType::Texture,
                            name,
                            id: name,
                        },
                    );
                },
                Err(_) => {
                    return entry;
                },
            }
        }

    }
    Ok(to_ret)
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    if !args.xml {
        return Err("xml flag MUST be present for now".to_string());
    } else {
        //TODO, move this out of main and create a custom type that wraps 
        //ALL potential formats
        let writer: Box<dyn Write> = if args.manifest_file == "-" {
            Box::new(stdout())
        } else {
            //this is lazy....
            Box::new(File::create(args.manifest_file).unwrap())
        };
        let xml_file = LocalFile::new(writer);
    }
    let assets_directory = args.assets_directory;
    let assets_path = Path::new(&assets_directory);
    if !assets_path.exists() {
        return Err(
            format!("assets_directory does not exist: '{assets_directory}', exiting")
        );
    }
    let abs_assets_path = fs::canonicalize(assets_path);
    todo!("not implemented");
}
