use std::fmt;

use crate::block_exit::BlockExit;
use crate::counters::{CodelChooser, DirectionPointer};

#[derive(Debug)]
pub struct Block {
    pub codel_coordinates: Vec<(usize, usize)>,
    pub hue: u8,
    pub light: u8,
    pub block_exit: Option<BlockExit>,
}

impl Block {
    pub fn exit_coordinates(
        &self,
        dp: &DirectionPointer,
        cc: &CodelChooser,
    ) -> Option<(usize, usize)> {
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

    pub fn size(&self) -> usize {
        self.codel_coordinates.len()
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
