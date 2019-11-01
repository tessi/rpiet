use std::fs::File;
use std::process;

use gif::SetParameter;
use png::ColorType::{Grayscale, GrayscaleAlpha, RGB, RGBA};

use crate::cmd_options::CmdOptions;

pub struct OutputInfo {
    pub width: u32,
    pub height: u32,
}

pub fn create_canvas(file: &File, options: &CmdOptions) -> Vec<Vec<(u8, u8, u8)>> {
    let (bytes, info) = parse_file(file, options);
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

fn parse_file(file: &File, options: &CmdOptions) -> (Vec<u8>, OutputInfo) {
    if options.file_path.ends_with(".png") {
        return parse_png_file(file, options);
    } else if options.file_path.ends_with(".gif") {
        return parse_gif_file(file, options);
    } else {
        eprintln!("Unknown filetype: {}", options.file_path);
        process::exit(1);
    }
}

fn parse_gif_file(file: &File, options: &CmdOptions) -> (Vec<u8>, OutputInfo) {
    let mut decoder = gif::Decoder::new(file);
    decoder.set(gif::ColorOutput::RGBA);
    let mut reader = match decoder.read_info() {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    };
    match reader.next_frame_info() {
        Ok(Some(_)) => (),
        Ok(None) => {
            eprintln!("Application error: No frame data while reading gif");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    };
    let mut data = vec![0; reader.buffer_size()];
    match reader.read_into_buffer(&mut data) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    };
    if options.verbose {
        eprintln!(
            "Parsed the file as valid GIF (width={}, height={})",
            reader.width(),
            reader.height()
        );
    }
    let output_info = OutputInfo {
        width: reader.width() as u32,
        height: reader.height() as u32,
    };
    (convert_to_rgb(data, png::ColorType::RGBA), output_info)
}

fn parse_png_file(file: &File, options: &CmdOptions) -> (Vec<u8>, OutputInfo) {
    let decoder = png::Decoder::new(file);
    let (info, mut reader) = match decoder.read_info() {
        Ok(decoded) => decoded,
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    };
    if info.width % options.codel_size != 0 || info.height % options.codel_size != 0 {
        eprintln!(
            "Application error: codel_size {} does not fit into image dimensions ({}, {})",
            options.codel_size, info.width, info.height
        );
        process::exit(1);
    }
    let mut data = vec![0; info.buffer_size()];
    reader.next_frame(&mut data).unwrap_or_else(|e| {
        eprintln!("Application error: {}", e);
        process::exit(1);
    });
    if options.verbose {
        eprintln!(
            "Parsed the file as valid PNG (width={}, height={})",
            info.width, info.height
        );
    }
    let output_info = OutputInfo {
        width: info.width,
        height: info.height,
    };
    (convert_to_rgb(data, info.color_type), output_info)
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
