extern crate clap;
extern crate png;

mod block;
mod block_exit;
mod cmd_options;
mod command;
mod counters;
mod interpreter;
mod utils;

use cmd_options::{clap_options, cmd_options};
use interpreter::Interpreter;
use std::fs::File;
use std::process;

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
    let canvas = utils::create_canvas(&file, &options);
    let mut interpreter = Interpreter::from_rgb_rows(canvas, &options);
    if options.verbose {
        eprintln!("Start State:   {}", interpreter);
    }
    while interpreter.is_alive() {
        interpreter.advance();
        if options.verbose {
            eprintln!("Current State: {}", interpreter);
        }
    }
}
