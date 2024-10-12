use std::{env, io};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;

use simplevm::{Instruction, OpCode, Register};

fn parse_numeric(s: &str) -> Result<u8, String> {
    if s.len() == 0 {
        return Err("String has no length".to_string());
    }
    let fst = s.chars().nth(0).unwrap();
    let (num, radix) = match fst {
        '$' => (&s[1..], 16),
        '%' => (&s[1..], 2),
        _ => (s, 10)
    };
    u8::from_str_radix(num, radix).map_err(|x| format!("{}", x))
}

fn parse_register(s: &str) -> Result<Register, String> {
    match s {
        "A" => Ok(Register::A),
        "B" => Ok(Register::B),
        "C" => Ok(Register::C),
        _ => Err(format!("invalid register: {}", s)),
    }
}

fn assert_length(parts: &Vec<&str>, n: usize) -> Result<(), String> {
    if parts.len() == n {
        Ok(())
    } else {
        Err(format!("expected {} parts, got {}", parts.len(), n))
    }
}

fn handle_line(parts: Vec<&str>) -> Result<Instruction, String> {
    let opcode = OpCode::from_str(parts[0]).ok_or(format!("unknown opcode: {}", parts[0]))?;
    match opcode {
        OpCode::Nop => Ok(Instruction::Nop),
        OpCode::Push => {
            assert_length(&parts, 2)?;
            Ok(Instruction::Push(parse_numeric(parts[1])?))
        },
        OpCode::AddStack => {
            assert_length(&parts, 1)?;
            Ok(Instruction::AddStack)
        },
        OpCode::AddRegister => {
            assert_length(&parts, 3)?;
            Ok(Instruction::AddRegister(
                parse_register(parts[1])?,
                parse_register(parts[2])?
            ))
        },
        OpCode::PopRegister => {
            assert_length(&parts, 2)?;
            Ok(Instruction::PopRegister(parse_register(parts[1])?))
        },
        OpCode::PushRegister => {
            assert_length(&parts, 2)?;
            Ok(Instruction::PushRegister(parse_register(parts[1])?))
        },
        OpCode::Signal => {
            assert_length(&parts, 2)?;
            Ok(Instruction::Signal(parse_numeric(parts[1])?))
        }
        // _ => Err(format!("unimplemented opcode: {:?}", opcode))
    }
}

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }
    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open: {}", x))?;
    let mut output: Vec<u8> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let inner_line = line.map_err(|_x| "foo")?;
        if inner_line.len() == 0 {
            continue;
        }
        if inner_line.chars().nth(0).unwrap() == ';' {
            continue;
        }
        let parts: Vec<_> = inner_line.split(" ").filter(|x| x.len() > 0).collect();
        if parts.len() == 0 {
            continue;
        }
        let instruction = handle_line(parts)?;
        let raw_instruction: u16 = instruction.encode_u16();
        output.push((raw_instruction&0xff) as u8);
        output.push((raw_instruction>>8) as u8);
    }
    let mut stdout = io::stdout().lock();
    stdout.write_all(&output).map_err(|x| format!("failed to write to stdout: {}", x))?;
    Ok(())
}