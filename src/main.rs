use std::collections::hash_map::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!();
    }

    let file_name = args.pop().expect("pop file name");

    let mut address_map: HashMap<String, u16> = HashMap::new();

    for (i, line) in BufReader::new(File::open(&file_name)?)
        .by_ref()
        .lines()
        .enumerate()
    {
        if let Some((val, _)) = line?.split_once(':') {
            address_map.insert(val.into(), i as u16);
        }
    }

    for (i, line) in BufReader::new(File::open(file_name)?).lines().enumerate() {
        let line = line?;

        let code: String = if let Some((code, _)) = line.split_once(';') {
            code.to_string()
        } else {
            line
        };

        let code: String = if let Some((_, without_address_code)) = code.split_once(':') {
            without_address_code.to_string()
        } else {
            code
        };

        let mut iter = code.split_whitespace();

        let mut output: u16 = match iter.next().expect("Mnemonic or number") {
            "HALT" => 0x0000,
            "PUSHI" => {
                0x1000
                    + iter
                        .next()
                        .expect("PUSHI expects number")
                        .parse::<u16>()
                        .expect("number")
            }
            "PUSH" => 0x2000,
            "POP" => 0x3000,
            "JMP" => 0x4000,
            "JZ" => 0x5000,
            "JNZ" => 0x6000,
            "IN" => {
                0xD000
                    + iter
                        .next()
                        .expect("IN expects number")
                        .parse::<u16>()
                        .expect("number")
            }
            "OUT" => 0xE000,
            "ADD" => 0xF000,
            "SUB" => 0xF001,
            "MUL" => 0xF002,
            "SHL" => 0xF003,
            "SHR" => 0xF004,
            "BAND" => 0xF005,
            "BOR" => 0xF006,
            "BXOR" => 0xF007,
            "AND" => 0xF008,
            "OR" => 0xF009,
            "EQ" => 0xF00A,
            "NE" => 0xF00B,
            "GE" => 0xF00C,
            "LE" => 0xF00D,
            "GT" => 0xF00E,
            "LT" => 0xF00F,
            "NEG" => 0xF010,
            "BNOT" => 0xF011,
            "NOT" => 0xF012,
            n => n.parse::<u16>().expect("Expect number"),
        };

        if let Some(n) = iter.next() {
            output += address_map[n];
        }

        println!("{:>03X}:{:>04X}", i, output);
    }

    Ok(())
}
