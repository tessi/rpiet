use std::fmt;

use crate::cmd_options::CmdOptions;

// pub const DP_UP: u8 = 0;
pub const DP_RIGHT: u8 = 1;
// pub const DP_DOWN: u8 = 2;
// pub const DP_LEFT: u8 = 3;

// pub const CC_RIGHT: u8 = 0;
pub const CC_LEFT: u8 = 1;

#[derive(Debug)]
struct Block {
    // stores (x,y) coordinates of codels furthest into the DP/CC direction
    dimensions: (
        // DP     CP
        ((u32, u32), (u32, u32)), // up:    up, down
        ((u32, u32), (u32, u32)), // right: up, down
        ((u32, u32), (u32, u32)), // down:  up, down
        ((u32, u32), (u32, u32)), // left:  up, down
    ),
    codel_count: u128,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Block<codel_count: {}, dimensions: {:?}>",
            self.codel_count, self.dimensions
        )
    }
}

#[derive(Debug)]
enum Codel<'a> {
    Color {
        x: usize,
        y: usize,
        hue: u8,
        light: u8,
        block: &'a Option<Block>,
    },
    Black {
        x: usize,
        y: usize,
    },
    White {
        x: usize,
        y: usize,
    },
    Edge,
}

impl fmt::Display for Codel<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Codel::Black { x, y } => write!(f, "Codel::Black<x: {}, y: {}>", x, y),
            Codel::White { x, y } => write!(f, "Codel::White<x: {}, y: {}>", x, y),
            Codel::Color {
                x,
                y,
                hue,
                light,
                block,
            } => write!(
                f,
                "Codel::Color<x: {}, y: {}, hue: {}, light: {}, block: {:?}>",
                x, y, hue, light, block
            ),
            Codel::Edge => write!(f, "Codel::Edge"),
        }
    }
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    dp: u8,
    cc: u8,
    alive: bool,
    stack: Vec<i64>,
    step_counter: u128,
    max_steps: u128,
    unlimited_steps: bool,
    canvas: Vec<Vec<Codel<'a>>>,
}

impl<'a> Interpreter<'a> {
    pub fn from_rgb_rows(
        rgb_rows: Vec<Vec<(u8, u8, u8)>>,
        options: &CmdOptions,
    ) -> Interpreter<'a> {
        let canvas = create_canvas(rgb_rows, options);
        let canvas = detect_blocks(canvas);

        Interpreter {
            dp: DP_RIGHT,
            cc: CC_LEFT,
            alive: true,
            stack: Vec::with_capacity(64),
            step_counter: 0,
            max_steps: options.max_steps,
            unlimited_steps: options.unlimited_steps,
            canvas: canvas,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn advance(&mut self) -> () {
        self.alive = false;
    }
}

fn create_canvas<'a>(
    rgb_rows: Vec<Vec<(u8, u8, u8)>>,
    options: &CmdOptions,
) -> Vec<Vec<Codel<'a>>> {
    let mut canvas = Vec::with_capacity(rgb_rows.len());
    for (y, rgb_row) in rgb_rows.into_iter().enumerate() {
        let mut codels = Vec::with_capacity(rgb_row.len());
        for (x, rgb) in rgb_row.into_iter().enumerate() {
            codels.push(rgb_to_codel(rgb, x, y, options.unknown_white));
        }
        canvas.push(codels);
    }
    canvas
}

fn rgb_to_codel<'a>(rgb: (u8, u8, u8), x: usize, y: usize, unknown_white: bool) -> Codel<'a> {
    match rgb {
        (0x00, 0x00, 0x00) => Codel::White { x: x, y: y },
        (0xFF, 0xFF, 0xFF) => Codel::Black { x: x, y: y },
        // light red
        (0xFF, 0xC0, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 0,
            light: 0,
            block: &None,
        },
        // red
        (0xFF, 0x00, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 0,
            light: 1,
            block: &None,
        },
        // dark  red
        (0xC0, 0x00, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 0,
            light: 2,
            block: &None,
        },
        // light yellow
        (0xFF, 0xFF, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 1,
            light: 0,
            block: &None,
        },
        // yellow
        (0xFF, 0xFF, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 1,
            light: 1,
            block: &None,
        },
        // dark  yellow
        (0xC0, 0xC0, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 1,
            light: 2,
            block: &None,
        },
        // light green
        (0xC0, 0xFF, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 2,
            light: 0,
            block: &None,
        },
        // green
        (0x00, 0xFF, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 2,
            light: 1,
            block: &None,
        },
        // dark  green
        (0x00, 0xC0, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 2,
            light: 2,
            block: &None,
        },
        // light cyan
        (0xC0, 0xFF, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 3,
            light: 0,
            block: &None,
        },
        // cyan
        (0x00, 0xFF, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 3,
            light: 1,
            block: &None,
        },
        // dark  cyan
        (0x00, 0xC0, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 3,
            light: 2,
            block: &None,
        },
        // light blue
        (0xC0, 0xC0, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 4,
            light: 0,
            block: &None,
        },
        // blue
        (0x00, 0x00, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 4,
            light: 1,
            block: &None,
        },
        // dark  blue
        (0x00, 0x00, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 4,
            light: 2,
            block: &None,
        },
        // light magenta
        (0xFF, 0xC0, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 5,
            light: 0,
            block: &None,
        },
        // magenta
        (0xFF, 0x00, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 5,
            light: 1,
            block: &None,
        },
        // dark  magenta
        (0xC0, 0x00, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 5,
            light: 2,
            block: &None,
        },
        _ => {
            if unknown_white {
                Codel::White { x: x, y: y }
            } else {
                Codel::Black { x: x, y: y }
            }
        }
    }
}

fn detect_blocks<'a>(canvas: Vec<Vec<Codel<'a>>>) -> Vec<Vec<Codel<'a>>> {
    canvas
}

impl fmt::Display for Interpreter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Interpreter<dp: {}, cc: {}, alive: {}, steps: {}, max_steps: {}, unlimited_steps: {}, stack: {:?}>", self.dp, self.cc, self.alive, self.step_counter, self.max_steps, self.unlimited_steps, self.stack)
    }
}
