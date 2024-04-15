use std::fs::File;

use clap::Parser;

use renderer::{
    importer::{
        wavefront::Wavefront,
        Importer,
    },
    model::Model,
};

const WAVEFRONT: &str = "wavefront";

/// CLI tool to parse model files exported by blender
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = WAVEFRONT)]
    file_type: String,

    #[arg(short, long)]
    in_files: Vec<String>,

    #[arg(short, long)]
    out_files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    if args.out_files.len() > args.in_files.len() {
        panic!("in_files count must be equal to or greater than out_files");
    }

    let wavefront = String::from(WAVEFRONT);
    let file_type = args.file_type;
    let mut parser = match file_type {
        wavefront => Wavefront::new(),
    };
    let models: Vec<Model> = vec![];
    for (i, f) in args.in_files.iter().enumerate() {
        let in_file = File::open(f)
            .expect(format!("Could not open file: {f} for reading").as_str());
        let pos_iter = parser.get_position_iterator(&in_file).unwrap();
        let tex_iter = parser.get_position_iterator(&in_file).unwrap();
        let norm_iter = parser.get_position_iterator(&in_file).unwrap();
        let index_iter = parser.get_index_iterator(&in_file).unwrap();
        parser.load_position_vector(pos_iter);
        parser.load_texture_vector(tex_iter);
        parser.load_normal_vector(norm_iter);
    }
    //parser.get_position_iterator(f)
}
