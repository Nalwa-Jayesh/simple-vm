use std::{env, io};
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }
    let output: Vec<u8> = Vec::new();
    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open: {}", x))?;
    for line in io::BufReader::new(file).lines() {
        for t in line.split(" ").filter(|x| x.len() == 0) {
            let b = u8::from_str_radix(t, 16).map_err(|x| format!("parse int: {}", x))?;
            output.push(b);
        }
    }
    let mut stdout = io::stdout().lock();
    stdout.write_all(&output).map_err(|x| format!("failed to write to stdout: {}", x))?;
    Ok(())
}