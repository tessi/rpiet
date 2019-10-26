use clap::{App, Arg, ArgMatches};

pub struct CmdOptions<'a> {
    pub verbose: bool,
    pub codel_size: u32,
    pub max_steps: u128,
    pub unlimited_steps: bool,
    pub file_path: &'a str,
}

fn is_png(val: String) -> Result<(), String> {
    if val.ends_with(".png") {
        Ok(())
    } else {
        Err(String::from("the file format must be png."))
    }
}

pub fn clap_options() -> ArgMatches<'static> {
    App::new("myapp")
        .version(clap::crate_version!())
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
        .get_matches()
}

pub fn cmd_options<'a>(options: &'a ArgMatches) -> CmdOptions<'a> {
    let verbose = options.is_present("verbose");
    let codel_size = options
        .value_of("codel_size")
        .map_or(1, |s| s.parse::<u32>().unwrap_or(1));
    let max_steps = options
        .value_of("max_steps")
        .map_or(-1, |s| s.parse::<i128>().unwrap_or(-1));
    let file_path = options.value_of("file").unwrap();

    CmdOptions {
        verbose: verbose,
        codel_size: codel_size,
        max_steps: if max_steps < 0 { 0 } else { max_steps as u128 },
        unlimited_steps: max_steps < 0,
        file_path: file_path,
    }
}
