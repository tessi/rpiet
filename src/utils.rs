use png::ColorType::{Grayscale, GrayscaleAlpha, RGB, RGBA};
use png::Decoder;
use std::fs::File;
use std::process;
use std::vec::Vec;

use crate::cmd_options::CmdOptions;

pub fn create_canvas(file: &File, options: &CmdOptions) -> Vec<Vec<(u8, u8, u8)>> {
    let (bytes, info) = parse_file(file, options);
    let bytes = convert_to_rgb(bytes, info.color_type);
    let pixels = group_pixels(bytes);
    let canvas = reduce_to_codels_and_group_into_rows(pixels, options.codel_size, info.width);
    if options.verbose {
        eprintln!(
            "Creating canvas with {} codels per row and {} rows",
            canvas[0].len(),
            canvas.len()
        );
    }
    canvas
}

fn parse_file(file: &File, options: &CmdOptions) -> (Vec<u8>, png::OutputInfo) {
    let decoder = Decoder::new(file);
    let (info, mut reader) = match decoder.read_info() {
        Ok(decoded) => decoded,
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    };
    if options.verbose {
        eprintln!(
            "Parsed the file as valid PNG (width={}, height={})",
            info.width, info.height
        );
    }
    if info.width % options.codel_size != 0 || info.height % options.codel_size != 0 {
        println!(
            "Application error: codel_size {} does not fit into image dimensions ({}, {})",
            options.codel_size, info.width, info.height
        );
        process::exit(1);
    }
    let mut data = vec![0; info.buffer_size()];
    reader.next_frame(&mut data).unwrap_or_else(|e| {
        println!("Application error: {}", e);
        process::exit(1);
    });
    (data, info)
}

fn convert_to_rgb(img_data: Vec<u8>, color_type: png::ColorType) -> Vec<u8> {
    match color_type {
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
    }
}

fn group_pixels(bytes: Vec<u8>) -> Vec<(u8, u8, u8)> {
    bytes
        .chunks_exact(3)
        .map(|rgb| (rgb[0], rgb[1], rgb[2]))
        .collect::<Vec<_>>()
}

fn reduce_to_codels_and_group_into_rows(
    pixels: Vec<(u8, u8, u8)>,
    codel_size: u32,
    width: u32,
) -> Vec<Vec<(u8, u8, u8)>> {
    let codels = pixels
        .into_iter()
        .step_by(codel_size as usize)
        .collect::<Vec<_>>();
    codels
        .chunks_exact((width / codel_size) as usize)
        .step_by(codel_size as usize)
        .map(|row| Vec::from(row))
        .collect::<Vec<_>>()
}
