#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("myapp")
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
                     Thus, piet things in codels which consist of many pixels.\n\
                     Setting codel-size to 2 would mean a codel is the size of 2x2 pixels.",
                )
                .takes_value(true)
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
    let verbose = matches.is_present("verbose");
    if verbose {
        println!("Reading file {}", matches.value_of("file").unwrap());
    }
}

fn is_png(val: String) -> Result<(), String> {
    if val.ends_with(".png") {
        Ok(())
    } else {
        Err(String::from("the file format must be png."))
    }
}