extern crate clap;
extern crate png;

use std::fs::File;
use std::process;

mod cmd_options;
use cmd_options::*;

mod utils;

fn main() {
    let clap_args = &clap_options();
    let options = cmd_options(&clap_args);

    if options.verbose {
        eprintln!("Reading file {}", options.file_path);
    }
    let file = File::open(options.file_path);
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    };
    let canvas = utils::create_canvas(&file, options);
    for row in canvas {
        print!("row:");
        for rgb in row {
            print!(" ({}, {}, {})", rgb.0, rgb.1, rgb.2);
        }
        println!("")
    }
}
