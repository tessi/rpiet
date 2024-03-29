use std::char;
use std::io::Read;

use crate::counters::{CodelChooser, DirectionPointer};

#[derive(Debug)]
pub enum Command {
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

impl Command {
    pub fn from(diffs: (u8, u8)) -> Option<Command> {
        match diffs {
            (0, 1) => Some(Command::Add),
            (0, 2) => Some(Command::Divide),
            (0, 3) => Some(Command::Greater),
            (0, 4) => Some(Command::Duplicate),
            (0, 5) => Some(Command::InChar),
            (1, 0) => Some(Command::Push),
            (1, 1) => Some(Command::Subtract),
            (1, 2) => Some(Command::Mod),
            (1, 3) => Some(Command::Pointer),
            (1, 4) => Some(Command::Roll),
            (1, 5) => Some(Command::OutNumber),
            (2, 0) => Some(Command::Pop),
            (2, 1) => Some(Command::Multiply),
            (2, 2) => Some(Command::Not),
            (2, 3) => Some(Command::Switch),
            (2, 4) => Some(Command::InNumber),
            (2, 5) => Some(Command::OutChar),
            _ => None,
        }
    }

    pub fn execute(
        &self,
        stack: &mut Vec<i64>,
        dp: &mut DirectionPointer,
        cc: &mut CodelChooser,
        block_size: usize,
        verbose_logging: bool,
    ) {
        match self {
            Command::Push => {
                if verbose_logging {
                    eprintln!("execute PUSH({})", block_size);
                }
                stack.push(block_size as i64)
            }
            Command::Pop => {
                if stack.pop().is_some() {
                    if verbose_logging {
                        eprintln!("execute POP");
                    }
                } else {
                    if verbose_logging {
                        eprintln!("skip executing POP due to empty stack");
                    }
                }
            }
            Command::Duplicate => {
                if let Some(&last) = stack.last() {
                    if verbose_logging {
                        eprintln!("execute DUPLICATE");
                    }
                    stack.push(last)
                } else {
                    if verbose_logging {
                        eprintln!("skip executing DUPLICATE due to empty stack");
                    }
                }
            }
            Command::Add => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if verbose_logging {
                        eprintln!("execute ADD({}, {})", b, a);
                    }
                    stack.push(a + b);
                } else {
                    if verbose_logging {
                        eprintln!("skip executing ADD due to not enough values on the stack");
                    }
                }
            }
            Command::Subtract => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if verbose_logging {
                        eprintln!("execute SUBTRACT({}, {})", b, a);
                    }
                    stack.push(b - a);
                } else {
                    if verbose_logging {
                        eprintln!("skip executing SUBTRACT due to not enough values on the stack");
                    }
                }
            }
            Command::Multiply => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if verbose_logging {
                        eprintln!("execute MULTIPLY({}, {})", b, a);
                    }
                    stack.push(a * b);
                } else {
                    if verbose_logging {
                        eprintln!("skip executing MULTIPLY due to not enough values on the stack");
                    }
                }
            }
            Command::Divide => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if a == 0 {
                        if verbose_logging {
                            eprintln!(
                                "skip executing DIVIDE due to not being able to divide by zero"
                            );
                        }
                    } else {
                        if verbose_logging {
                            eprintln!("execute DIVIDE({}, {})", b, a);
                        }
                        stack.push(b / a);
                    }
                } else {
                    if verbose_logging {
                        eprintln!("skip executing DIVIDE due to not enough values on the stack");
                    }
                }
            }
            Command::Mod => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if a == 0 {
                        if verbose_logging {
                            eprintln!("skip executing MOD due to not being able to modulo by zero");
                        }
                    } else {
                        if verbose_logging {
                            eprintln!("execute MOD({}, {})", b, a);
                        }
                        stack.push(b.rem_euclid(a));
                    }
                } else {
                    if verbose_logging {
                        eprintln!("skip executing MOD due to not enough values on the stack");
                    }
                }
            }
            Command::Not => {
                if let Some(a) = stack.pop() {
                    if verbose_logging {
                        eprintln!("execute NOT({})", a);
                    }
                    if a == 0 {
                        stack.push(1);
                    } else {
                        stack.push(0);
                    }
                } else {
                    if verbose_logging {
                        eprintln!("skip executing NOT due to empty stack");
                    }
                }
            }
            Command::Greater => {
                if stack.len() >= 2 {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    if verbose_logging {
                        eprintln!("execute GREATER({}, {})", b, a);
                    }
                    if b > a {
                        stack.push(1);
                    } else {
                        stack.push(0);
                    }
                } else {
                    if verbose_logging {
                        eprintln!("skip executing GREATER due to not enough values on the stack");
                    }
                }
            }
            Command::Pointer => {
                if let Some(a) = stack.pop() {
                    if verbose_logging {
                        eprintln!("execute POINTER({})", a);
                    }
                    match a % 4 {
                        3 => turn_direction_pointer(dp, 3),
                        2 => turn_direction_pointer(dp, 2),
                        1 => turn_direction_pointer(dp, 1),
                        -1 => turn_direction_pointer(dp, 3),
                        -2 => turn_direction_pointer(dp, 2),
                        -3 => turn_direction_pointer(dp, 1),
                        _ => (),
                    }
                } else {
                    if verbose_logging {
                        eprintln!("skip executing POINTER due to empty stack");
                    }
                }
            }
            Command::Switch => {
                if let Some(a) = stack.pop() {
                    if verbose_logging {
                        eprintln!("execute SWITCH({})", a);
                    }
                    if a % 2 == 1 {
                        *cc = match cc {
                            CodelChooser::Left => CodelChooser::Right,
                            CodelChooser::Right => CodelChooser::Left,
                        }
                    }
                } else {
                    if verbose_logging {
                        eprintln!("skip executing SWITCH due to empty stack");
                    }
                }
            }
            Command::Roll => {
                if stack.len() >= 2 {
                    let rolls = stack.pop().unwrap();
                    let depth = stack.pop().unwrap();
                    if depth <= 0 {
                        if verbose_logging {
                            eprintln!("skip executing ROLL must be a positive roll depth");
                        }
                        stack.push(depth);
                        stack.push(rolls);
                    } else {
                        if stack.len() < depth as usize {
                            if verbose_logging {
                                eprintln!(
                                    "skip executing ROLL due to not enough values on the stack"
                                );
                            }
                            stack.push(depth);
                            stack.push(rolls);
                        } else {
                            if verbose_logging {
                                eprintln!("execute ROLL({}, {})", depth, rolls);
                            }
                            let rolls = rolls % depth;
                            let mut substack: Vec<_> =
                                stack.drain(stack.len() - (depth as usize)..).collect();
                            if rolls > 0 {
                                substack.rotate_right(rolls as usize);
                            } else {
                                substack.rotate_left((rolls * -1) as usize);
                            }
                            stack.append(&mut substack);
                        }
                    }
                } else {
                    if verbose_logging {
                        eprintln!("skip executing ROLL due to not enough values on the stack");
                    }
                }
            }
            Command::OutNumber => {
                if let Some(last) = stack.pop() {
                    if verbose_logging {
                        eprintln!("execute OUT_NUM({})", last);
                    }
                    print!("{}", last);
                } else {
                    if verbose_logging {
                        eprintln!("skip executing OUT_NUM due to empty stack");
                    }
                }
            }
            Command::OutChar => {
                if let Some(last) = stack.pop() {
                    if last >= 0 && last <= (u32::max_value() as i64) {
                        let c = char::from_u32(last as u32);
                        if let Some(c) = c {
                            if verbose_logging {
                                eprintln!("execute OUT_CHAR({} -> {})", last, c);
                            }
                            print!("{}", c);
                        } else {
                            if verbose_logging {
                                eprintln!("skip executing OUT_CHAR due invalid char");
                            }
                            stack.push(last)
                        }
                    } else {
                        if verbose_logging {
                            eprintln!("skip executing OUT_CHAR due invalid char");
                        }
                        stack.push(last)
                    }
                } else {
                    if verbose_logging {
                        eprintln!("skip executing OUT_CHAR due to empty stack");
                    }
                }
            }
            Command::InNumber => {
                let mut buffer = String::new();
                if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
                    if let Ok(num) = buffer.trim().parse::<i64>() {
                        if verbose_logging {
                            eprintln!("executed IN_NUM({})", num);
                        }
                        stack.push(num);
                    } else {
                        if verbose_logging {
                            eprintln!("skip executing IN_NUM() because input could not be parse as a number");
                        }
                    }
                } else {
                    if verbose_logging {
                        eprintln!("skip executing IN_NUM() because input could read");
                    }
                }
            }
            Command::InChar => {
                let input: Option<u8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok());

                match input {
                    Some(byte) => {
                        if verbose_logging {
                            eprintln!(
                                "executed IN_CHAR({} -> {})",
                                char::from_u32(byte as u32).unwrap(),
                                byte as i64
                            );
                        }
                        stack.push(byte as i64);
                    }
                    None => {
                        if verbose_logging {
                            eprintln!("skip executing IN_CHAR() because input was empty or could not be read");
                        }
                    }
                }
            }
        }
    }
}

fn turn_direction_pointer(dp: &mut DirectionPointer, turns: u32) {
    if turns == 0 {
        return;
    }
    *dp = match dp {
        DirectionPointer::Right => DirectionPointer::Down,
        DirectionPointer::Down => DirectionPointer::Left,
        DirectionPointer::Left => DirectionPointer::Up,
        DirectionPointer::Up => DirectionPointer::Right,
    };
    turn_direction_pointer(dp, turns - 1)
}
