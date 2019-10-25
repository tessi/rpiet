#[macro_use]
extern crate clap;
use clap::{App, Arg};
use png::ColorType::{Grayscale, GrayscaleAlpha, RGB, RGBA};
use png::Decoder;
use std::fs::File;
use std::process;

fn main() {
    let options = App::new("myapp")
        .version(crate_version!())
        .author("Philipp Tessenow <philipp@tessenow.org>")
        .about("An interpreter for the piet programming language")
        .arg(
            Arg::with_name("file")
                .help("The image to execute. Supports png files only")
                .default_value("input.png")
                .index(1)
                .required(true)
                .validator(is_png),
        )
        .arg(
            Arg::with_name("codel_size")
                .help("The length of a codel in pixels")
                .default_value("1")
                .short("c")
                .long("codel-size")
                .long_help(
                    "Piet works by going through the pixels of an image.\n\
                     However, this makes piet images visually small when viewing them.\n\
                     Thus, piet allows interpreting images in codels which consist of larger pixels blocks.\n\
                     Setting codel-size to 2 would mean a codel is the size of 2x2 pixels.",
                )
                .takes_value(true)
                .required(false)
                .validator(|s| {
                    s.parse::<u32>()
                        .map(|_| ())
                        .map_err(|_| String::from("Must be a positive number!"))
                }),
        )
        .arg(
            Arg::with_name("max_steps")
                .help("The max number of allowed execution steps")
                .short("e")
                .long("max-steps")
                .long_help(
                    "This stops the piet interpreter after the given number of steps and\n\
                    solves the halting problem once and for all :)\n\
                    Very useful to debug endless loops",
                )
                .takes_value(true)
                .required(false)
                .validator(|s| {
                    s.parse::<u32>()
                        .map(|_| ())
                        .map_err(|_| String::from("Must be a positive number!"))
                }),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Logs debug information to stderr")
                .short("v")
                .long("verbose")
        )
        .get_matches();
    let verbose = options.is_present("verbose");
    let codel_size = options
        .value_of("codel_size")
        .map_or(1, |s| s.parse::<u32>().unwrap_or(1));
    let path = options.value_of("file").unwrap();
    if verbose {
        eprintln!("Reading file {}", options.value_of("file").unwrap());
    }
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    };
    let decoder = Decoder::new(file);
    let (info, mut reader) = match decoder.read_info() {
        Ok(decoded) => decoded,
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    };
    if verbose {
        eprintln!(
            "Parsed the file as valid PNG (width={}, height={})",
            info.width, info.height
        );
    }
    if info.width % codel_size != 0 || info.height % codel_size != 0 {
        println!(
            "Application error: codel_size {} does not fit into image dimensions ({}, {})",
            codel_size, info.width, info.height
        );
        process::exit(1);
    }
    let mut img_data = vec![0; info.buffer_size()];
    reader.next_frame(&mut img_data).unwrap_or_else(|e| {
        println!("Application error: {}", e);
        process::exit(1);
    });
    let data = match info.color_type {
        RGB => img_data,
        RGBA => {
            let mut vec = Vec::with_capacity(img_data.len() / 4 * 3);
            for rgba in img_data.chunks(4) {
                let r = rgba[0];
                let g = rgba[1];
                let b = rgba[2];
                vec.extend([r, g, b].iter().cloned())
            }
            vec
        }
        Grayscale => {
            let mut vec = Vec::with_capacity(img_data.len() * 3);
            for g in img_data {
                vec.extend([g, g, g].iter().cloned())
            }
            vec
        }
        GrayscaleAlpha => {
            let mut vec = Vec::with_capacity(img_data.len() * 3);
            for ga in img_data.chunks(2) {
                let g = ga[0];
                let a = ga[1];
                vec.extend([g, g, g, a].iter().cloned())
            }
            vec
        }
        _ => unreachable!("uncovered color type"),
    };
    for rgb in data.chunks_exact(3) {
        println!("{}, {}, {}", rgb[0], rgb[1], rgb[2]);
    }
}

fn is_png(val: String) -> Result<(), String> {
    if val.ends_with(".png") {
        Ok(())
    } else {
        Err(String::from("the file format must be png."))
    }
}
