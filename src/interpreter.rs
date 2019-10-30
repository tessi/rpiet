use std::fmt;

use crate::block_exit::BlockExit;
use crate::cmd_options::CmdOptions;

// TODO: this file is too big, probably needs being split up
// TODO: we needs tests (also for other modules)
// TODO: needs better module/method level documentation

const MAX_ALLOWED__POINTER_TOGGLES: u8 = 8;

#[derive(Debug)]
enum DirectionPointer {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for DirectionPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DirectionPointer::Up => write!(f, "up"),
            DirectionPointer::Right => write!(f, "right"),
            DirectionPointer::Down => write!(f, "down"),
            DirectionPointer::Left => write!(f, "left"),
        }
    }
}

#[derive(Debug)]
enum CodelChooser {
    Right,
    Left,
}

impl fmt::Display for CodelChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodelChooser::Right => write!(f, "right"),
            CodelChooser::Left => write!(f, "left"),
        }
    }
}

enum Command {
    Push,
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Not,
    Greater,
    Pointer,
    Switch,
    Duplicate,
    Roll,
    InNumber,
    InChar,
    OutNumber,
    OutChar,
}

#[derive(Debug)]
enum Counters {
    DirectionPointer,
    CodelChooser,
}

#[derive(Debug)]
struct Block {
    codel_coordinates: Vec<(usize, usize)>,
    hue: u8,
    light: u8,
    block_exit: Option<BlockExit>,
}

impl Block {
    fn exit_coordinates(&self, dp: &DirectionPointer, cc: &CodelChooser) -> Option<(usize, usize)> {
        if let Some(block_exit) = &self.block_exit {
            let coord = match dp {
                DirectionPointer::Up => match cc {
                    CodelChooser::Right => block_exit.exits[0][1],
                    CodelChooser::Left => block_exit.exits[0][0],
                },
                DirectionPointer::Right => match cc {
                    CodelChooser::Right => block_exit.exits[1][1],
                    CodelChooser::Left => block_exit.exits[1][0],
                },
                DirectionPointer::Down => match cc {
                    CodelChooser::Right => block_exit.exits[2][0],
                    CodelChooser::Left => block_exit.exits[2][1],
                },
                DirectionPointer::Left => match cc {
                    CodelChooser::Right => block_exit.exits[3][0],
                    CodelChooser::Left => block_exit.exits[3][1],
                },
            };
            Some(coord)
        } else {
            None
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Block<codel_count: {}, hue: {}, light: {}>",
            self.codel_coordinates.len(),
            self.hue,
            self.light
        )
    }
}

#[derive(Debug)]
enum Codel {
    Color {
        x: usize,
        y: usize,
        hue: u8,
        light: u8,
        block_index: Option<usize>,
    },
    Black {
        x: usize,
        y: usize,
    },
    White {
        x: usize,
        y: usize,
    },
}

impl fmt::Display for Codel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Codel::Black { x, y } => write!(f, "Codel::Black<{}, {}>", x, y),
            Codel::White { x, y } => write!(f, "Codel::White<{}, {}>", x, y),
            Codel::Color {
                x,
                y,
                hue,
                light,
                block_index,
            } => write!(
                f,
                "Codel::Color<{}, {}> hue: {}, light: {}, block_index: {:?}",
                x, y, hue, light, block_index
            ),
        }
    }
}

#[derive(Debug)]
pub struct Interpreter {
    dp: DirectionPointer,
    cc: CodelChooser,
    alive: bool,
    stack: Vec<i64>,
    step_counter: u128,
    max_steps: u128,
    unlimited_steps: bool,
    canvas: Vec<Vec<Codel>>,
    blocks: Vec<Block>,
    width: usize,
    height: usize,
    current_position: (usize, usize),
    toggled_pointers_without_move: u8,
    last_toggled_pointer: Counters,
}

impl Interpreter {
    pub fn from_rgb_rows(rgb_rows: Vec<Vec<(u8, u8, u8)>>, options: &CmdOptions) -> Interpreter {
        let canvas = create_canvas(rgb_rows, options);
        let width = canvas[0].len();
        let height = canvas.len();
        let mut interpreter = Interpreter {
            dp: DirectionPointer::Right,
            cc: CodelChooser::Left,
            alive: true,
            stack: Vec::with_capacity(64),
            step_counter: 0,
            max_steps: options.max_steps,
            unlimited_steps: options.unlimited_steps,
            canvas: canvas,
            blocks: Vec::new(),
            width: width,
            height: height,
            current_position: (0, 0),
            toggled_pointers_without_move: 0,
            last_toggled_pointer: Counters::DirectionPointer,
        };
        interpreter.detect_blocks();
        interpreter.assign_codels_to_blocks();
        interpreter.find_exits_for_blocks();
        interpreter
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn advance(&mut self) -> () {
        self.step_counter += 1;
        if self.max_steps_reached() || self.program_should_end() {
            self.exit();
            return;
        }
        let command = match self.find_next_codel() {
            Some((new_position, traveled_through_white)) => {
                self.toggled_pointers_without_move = 0;
                let old_position = self.current_position;
                self.current_position = new_position;
                if !traveled_through_white {
                    Some(self.command_to_execute(old_position, new_position))
                } else {
                    None
                }
            }
            None => {
                self.toogle_counters();
                self.toggled_pointers_without_move += 1;
                None
            }
        };
        if let Some(command) = command {
            self.execute(command)
        };
    }

    fn execute(&mut self, command: Command) {
        // TODO: implement commands to execute
    }

    fn command_to_execute(
        &mut self,
        old_position: (usize, usize),
        new_position: (usize, usize),
    ) -> Command {
        Command::Add // TODO: return actual commands
    }

    fn find_next_codel(&self) -> Option<((usize, usize), bool)> {
        let current_block_index = match self.current_codel() {
            Codel::Color {
                x: _x,
                y: _y,
                hue: _hue,
                light: _light,
                block_index,
            } => block_index,
            _ => return None,
        };
        let &current_block_index = match current_block_index {
            Some(i) => i,
            None => return None,
        };

        let coord = self.blocks[current_block_index].exit_coordinates(&self.dp, &self.cc);
        let mut coord = match coord {
            Some(coord) => coord,
            None => return None,
        };

        let mut traveled_through_white = false;
        while let Some(next_codel) = self.find_next_codel_from(coord) {
            match *next_codel {
                Codel::Black { x: _, y: _ } => break,
                Codel::White { x, y } => {
                    traveled_through_white = true;
                    coord = (x, y);
                }
                Codel::Color {
                    x,
                    y,
                    hue: _,
                    light: _,
                    block_index,
                } => {
                    let block_index = match block_index {
                        Some(i) => i,
                        None => return None,
                    };
                    if current_block_index != block_index {
                        return Some(((x, y), traveled_through_white));
                    }
                    coord = (x, y);
                }
            }
        }
        None
    }

    fn find_next_codel_from(&self, start: (usize, usize)) -> Option<&Codel> {
        let next_coords = match self.dp {
            DirectionPointer::Up => coord_up(start, self.width, self.height),
            DirectionPointer::Right => coord_right(start, self.width, self.height),
            DirectionPointer::Down => coord_down(start, self.width, self.height),
            DirectionPointer::Left => coord_left(start, self.width, self.height),
        };
        next_coords.map(|coord| self.codel_for(coord))
    }

    fn current_codel(&self) -> &Codel {
        self.codel_for(self.current_position)
    }

    fn codel_for(&self, coord: (usize, usize)) -> &Codel {
        &self.canvas[coord.1][coord.0]
    }

    fn max_steps_reached(&self) -> bool {
        !self.unlimited_steps && self.step_counter >= self.max_steps
    }

    fn program_should_end(&self) -> bool {
        self.toggled_pointers_without_move >= MAX_ALLOWED__POINTER_TOGGLES
    }

    fn toogle_counters(&mut self) {
        match self.last_toggled_pointer {
            Counters::DirectionPointer => {
                self.last_toggled_pointer = Counters::CodelChooser;
                self.cc = match self.cc {
                    CodelChooser::Right => CodelChooser::Left,
                    CodelChooser::Left => CodelChooser::Right,
                };
            }
            Counters::CodelChooser => {
                self.last_toggled_pointer = Counters::DirectionPointer;
                self.dp = match self.dp {
                    DirectionPointer::Up => DirectionPointer::Right,
                    DirectionPointer::Right => DirectionPointer::Down,
                    DirectionPointer::Down => DirectionPointer::Left,
                    DirectionPointer::Left => DirectionPointer::Up,
                };
            }
        }
    }

    fn exit(&mut self) {
        self.alive = false;
    }

    fn assign_codels_to_blocks(&mut self) -> () {
        for row in self.canvas.iter_mut() {
            for codel in row.iter_mut() {
                if let Codel::Color {
                    block_index,
                    x,
                    y,
                    hue: _,
                    light: _,
                } = codel
                {
                    *block_index = self
                        .blocks
                        .iter_mut()
                        .position(|b| b.codel_coordinates.contains(&(*x, *y)));
                }
            }
        }
    }

    fn detect_blocks(&mut self) -> () {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];
        for row in self.canvas.iter() {
            for codel in row.iter() {
                if let Codel::Color {
                    block_index: _,
                    x,
                    y,
                    hue,
                    light,
                } = codel
                {
                    if visited[*y][*x] {
                        continue;
                    }
                    let block = Block {
                        codel_coordinates: Vec::new(),
                        hue: *hue,
                        light: *light,
                        block_exit: None,
                    };
                    self.blocks.push(block);
                    let new_block_index = self.blocks.len() - 1;

                    let mut visit_list: Vec<(usize, usize)> = [(*x, *y)].to_vec();
                    while !visit_list.is_empty() {
                        let coord = visit_list.pop().unwrap();
                        if visited[coord.1][coord.0] {
                            continue;
                        }
                        visited[coord.1][coord.0] = true;
                        let block = &mut self.blocks[new_block_index];
                        block.codel_coordinates.push(coord);

                        // right neighbour
                        if let Some(other_coord) = coord_right(coord, self.width, self.height) {
                            let other_codel = &self.canvas[other_coord.1][other_coord.0];
                            if let Codel::Color {
                                block_index: _,
                                x,
                                y,
                                hue,
                                light,
                            } = other_codel
                            {
                                if !visited[*y][*x] && block.hue == *hue && block.light == *light {
                                    visit_list.push(other_coord);
                                }
                            }
                        }

                        // left neighbour
                        if let Some(other_coord) = coord_left(coord, self.width, self.height) {
                            let other_codel = &self.canvas[other_coord.1][other_coord.0];
                            if let Codel::Color {
                                block_index: _,
                                x,
                                y,
                                hue,
                                light,
                            } = other_codel
                            {
                                if !visited[*y][*x] && block.hue == *hue && block.light == *light {
                                    visit_list.push(other_coord);
                                }
                            }
                        }

                        // up neighbour
                        if let Some(other_coord) = coord_up(coord, self.width, self.height) {
                            let other_codel = &self.canvas[other_coord.1][other_coord.0];
                            if let Codel::Color {
                                block_index: _,
                                x,
                                y,
                                hue,
                                light,
                            } = other_codel
                            {
                                if !visited[*y][*x] && block.hue == *hue && block.light == *light {
                                    visit_list.push(other_coord);
                                }
                            }
                        }

                        // down neighbour
                        if let Some(other_coord) = coord_down(coord, self.width, self.height) {
                            let other_codel = &self.canvas[other_coord.1][other_coord.0];
                            if let Codel::Color {
                                block_index: _,
                                x,
                                y,
                                hue,
                                light,
                            } = other_codel
                            {
                                if !visited[*y][*x] && block.hue == *hue && block.light == *light {
                                    visit_list.push(other_coord);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn find_exits_for_blocks(&mut self) -> () {
        for block in self.blocks.iter_mut() {
            block.block_exit = Some(BlockExit::from_coords(&block.codel_coordinates));
        }
    }
}

fn coord_right(coord: (usize, usize), width: usize, _height: usize) -> Option<(usize, usize)> {
    if coord.0 + 1 >= width {
        None
    } else {
        Some {
            0: (coord.0 + 1, coord.1),
        }
    }
}

fn coord_left(coord: (usize, usize), _width: usize, _height: usize) -> Option<(usize, usize)> {
    if coord.0 == 0 {
        None
    } else {
        Some {
            0: (coord.0 - 1, coord.1),
        }
    }
}

fn coord_up(coord: (usize, usize), _width: usize, _height: usize) -> Option<(usize, usize)> {
    if coord.1 == 0 {
        None
    } else {
        Some {
            0: (coord.0, coord.1 - 1),
        }
    }
}

fn coord_down(coord: (usize, usize), _width: usize, height: usize) -> Option<(usize, usize)> {
    if coord.1 + 1 >= height {
        None
    } else {
        Some {
            0: (coord.0, coord.1 + 1),
        }
    }
}

fn create_canvas<'a>(rgb_rows: Vec<Vec<(u8, u8, u8)>>, options: &CmdOptions) -> Vec<Vec<Codel>> {
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

fn rgb_to_codel<'a>(rgb: (u8, u8, u8), x: usize, y: usize, unknown_white: bool) -> Codel {
    match rgb {
        (0x00, 0x00, 0x00) => Codel::Black { x: x, y: y },
        (0xFF, 0xFF, 0xFF) => Codel::White { x: x, y: y },
        // light red
        (0xFF, 0xC0, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 0,
            light: 0,
            block_index: None,
        },
        // red
        (0xFF, 0x00, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 0,
            light: 1,
            block_index: None,
        },
        // dark  red
        (0xC0, 0x00, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 0,
            light: 2,
            block_index: None,
        },
        // light yellow
        (0xFF, 0xFF, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 1,
            light: 0,
            block_index: None,
        },
        // yellow
        (0xFF, 0xFF, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 1,
            light: 1,
            block_index: None,
        },
        // dark  yellow
        (0xC0, 0xC0, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 1,
            light: 2,
            block_index: None,
        },
        // light green
        (0xC0, 0xFF, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 2,
            light: 0,
            block_index: None,
        },
        // green
        (0x00, 0xFF, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 2,
            light: 1,
            block_index: None,
        },
        // dark  green
        (0x00, 0xC0, 0x00) => Codel::Color {
            x: x,
            y: y,
            hue: 2,
            light: 2,
            block_index: None,
        },
        // light cyan
        (0xC0, 0xFF, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 3,
            light: 0,
            block_index: None,
        },
        // cyan
        (0x00, 0xFF, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 3,
            light: 1,
            block_index: None,
        },
        // dark  cyan
        (0x00, 0xC0, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 3,
            light: 2,
            block_index: None,
        },
        // light blue
        (0xC0, 0xC0, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 4,
            light: 0,
            block_index: None,
        },
        // blue
        (0x00, 0x00, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 4,
            light: 1,
            block_index: None,
        },
        // dark  blue
        (0x00, 0x00, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 4,
            light: 2,
            block_index: None,
        },
        // light magenta
        (0xFF, 0xC0, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 5,
            light: 0,
            block_index: None,
        },
        // magenta
        (0xFF, 0x00, 0xFF) => Codel::Color {
            x: x,
            y: y,
            hue: 5,
            light: 1,
            block_index: None,
        },
        // dark  magenta
        (0xC0, 0x00, 0xC0) => Codel::Color {
            x: x,
            y: y,
            hue: 5,
            light: 2,
            block_index: None,
        },
        (r, g, b) => {
            eprintln!("Parsed unknown codel color ({r}, {g}, {b}) / (#{r:02X}{g:02X}{b:02X}) at pos ({x},{y})", r=r, g=g, b=b, x=x, y=y);
            if unknown_white {
                Codel::White { x: x, y: y }
            } else {
                Codel::Black { x: x, y: y }
            }
        }
    }
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Interpreter<dp: {}, cc: {}, pos: {:?} alive: {}, steps: {}, pointer_toggles_without_move: {}, stack: {:?}>",
            self.dp, self.cc, self.current_position, self.alive, self.step_counter, self.toggled_pointers_without_move, self.stack
        )
    }
}
