use std::fmt;

#[derive(Debug)]
pub enum DirectionPointer {
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
pub enum CodelChooser {
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

#[derive(Debug)]
pub enum Counters {
    DirectionPointer,
    CodelChooser,
}
