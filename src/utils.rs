use png::ColorType::{Grayscale, GrayscaleAlpha, RGB, RGBA};
use png::Decoder;
use std::fs::File;
use std::process;
use std::vec::Vec;

use crate::cmd_options::CmdOptions;

pub fn create_canvas<'a>(file: &'a File, options: CmdOptions<'a>) -> Vec<Vec<(u8, u8, u8)>> {
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
    let pixels = data
        .chunks_exact(3)
        .map(|rgb| (rgb[0], rgb[1], rgb[2]))
        .collect::<Vec<_>>();
    let codels = pixels
        .into_iter()
        .step_by(options.codel_size as usize)
        .collect::<Vec<_>>();
    let canvas = codels
        .chunks_exact((info.width / options.codel_size) as usize)
        .step_by(options.codel_size as usize)
        .map(|row| Vec::from(row))
        .collect::<Vec<_>>();
    if options.verbose {
        eprintln!(
            "Creating canvas with {} codels per row and {} rows",
            canvas[0].len(),
            canvas.len()
        );
    }
    canvas
}
