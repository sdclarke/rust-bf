use std::env::args;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum RawInstruction {
    Increment,
    Decrement,
    LoopOpen,
    LoopClose,
    RightShift,
    LeftShift,
    Output,
    Input,
}

impl RawInstruction {
    fn from_char(c: char) -> Option<RawInstruction> {
        match c {
            '+' => Some(Self::Increment),
            '-' => Some(Self::Decrement),
            '[' => Some(Self::LoopOpen),
            ']' => Some(Self::LoopClose),
            '>' => Some(Self::RightShift),
            '<' => Some(Self::LeftShift),
            '.' => Some(Self::Output),
            ',' => Some(Self::Input),
            _ => None,
        }
    }
}

impl std::fmt::Display for RawInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Increment => write!(f, "Increment value at pointer"),
            Self::Decrement => write!(f, "Decrement value at pointer"),
            Self::LoopOpen => write!(f, "Start looping"),
            Self::LoopClose => write!(f, "Stop looping"),
            Self::RightShift => write!(f, "Increment pointer"),
            Self::LeftShift => write!(f, "Decrement pointer"),
            Self::Output => write!(f, "Output byte at pointer"),
            Self::Input => write!(f, "Read byte of input to location at pointer"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RawInstructionStruct {
    raw_instruction: RawInstruction,
    line: i64,
    column: i64,
}

impl RawInstructionStruct {
    fn new(raw_instruction: RawInstruction, line: i64, column: i64) -> Self {
        Self {
            raw_instruction,
            line,
            column,
        }
    }

    fn raw_instruction(&self) -> RawInstruction {
        self.raw_instruction
    }

    fn line(&self) -> i64 {
        self.line
    }

    fn column(&self) -> i64 {
        self.column
    }
}

fn parse_file(filename: &str) -> Result<Vec<RawInstructionStruct>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let bf = contents.trim_end().split('\n');

    let mut raw_instruction_vec = Vec::new();
    let mut line = 1;
    for s in bf {
        let mut column = 1;
        // For each character in a line
        for c in s.chars() {
            // If it is a bf char, create struct and add to Vec
            let raw_instruction = match RawInstruction::from_char(c) {
                Some(r) => r,
                None => {
                    // If not bf char, skip
                    column += 1;
                    continue;
                }
            };
            raw_instruction_vec.push(RawInstructionStruct::new(raw_instruction, line, column));
            column += 1;
        }
        line += 1;
    }
    Ok(raw_instruction_vec)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename which will be the second element of the args
    let filename = args().nth(1).ok_or("Expected filename")?;

    for r in parse_file(&filename)? {
        println!(
            "[{}:{}:{}] {}",
            filename,
            r.line(),
            r.column(),
            r.raw_instruction()
        );
    }

    Ok(())
}
