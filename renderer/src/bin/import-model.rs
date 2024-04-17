/*
 * I should probably add a backup as this gets more "serious"
 */
use std::fs::{
    read_to_string, File, OpenOptions,
};

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
    parser: String,

    #[arg(short, long)]
    in_files: Vec<String>,

    #[arg(short, long)]
    out_files: Vec<String>,
}

fn make_importer(parser: &str) -> Result<impl Importer, String> {
    match parser.trim() {
        WAVEFRONT => {
            Ok(Wavefront::new(None))
        },
        &_ => {
            Err(format!("Unrecognized file_type passed: '{parser}', exiting"))
        },
    }
}

//NOTE, only handles a single model per input...for now
fn main() {
    let args = Args::parse();
    if args.out_files.len() != args.in_files.len() {
        panic!("in_files count must be equal to out_files count");
    }
    let parser = make_importer(&args.parser).unwrap();
    let models: Vec<Model> = args.in_files.iter().map(|f| {
        //this is really dumb...but im new to rust so w/e, ill develop better patterns
        //not a critical loop anyway
        let lines: Vec<String> = read_to_string(f)
            .expect(format!("Could not open in_file: '{f}' for reading.").as_str())
            .lines().map(|l| l.to_string()).collect();
        parser.generate_model(lines.iter()).unwrap()
    }).collect();
    for (i, m) in models.iter().enumerate() {
        let out_file_name = &args.out_files[i];
        let out_file = OpenOptions::new().create(true).write(true).open(out_file_name);
        //m.write_to_disk(out_file).unwrap();
        //println!("mesh: {:?}", m);
        //write!("{}", m);
    }
}
