use cranevm::structs::*;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

// This basically is a 1:1 translation of asm -> cb
fn cb_from_asm(asm: String) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    //read the file line by line, split by spaces, and parse the instruction and operands
    for line in asm.lines() {
        let mut parts = line.split_whitespace();
        let opcode = opcode_from_string(parts.next().unwrap_or("NOP"));
        let operands = parts.map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        //convert operands to [u8;3]
        let ops = operands.try_into().unwrap();
        instructions.push(Instruction {
            opcode,
            operands: ops,
        });
    }
    instructions
}

fn opcode_from_string(opcode: &str) -> Opcode {
    match opcode {
        "NOP" => Opcode::NOP,
        "HALT" => Opcode::HALT,
        "PUSH" => Opcode::PUSH,
        "POP" => Opcode::POP,
        "SWAP" => Opcode::SWAP,
        "ADD" => Opcode::ADD,
        "SUB" => Opcode::SUB,
        "MUL" => Opcode::MUL,
        "DIV" => Opcode::DIV,
        "MOD" => Opcode::MOD,
        "INC" => Opcode::INC,
        "DEC" => Opcode::DEC,
        "AND" => Opcode::AND,
        "OR" => Opcode::OR,
        "XOR" => Opcode::XOR,
        "NOT" => Opcode::NOT,
        "SHL" => Opcode::SHL,
        "SHR" => Opcode::SHR,
        "EQ" => Opcode::EQ,
        "JMP" => Opcode::JMP,
        "MOV" => Opcode::MOV,
        "STORE" => Opcode::STORE,
        _ => panic!("Invalid opcode {}", opcode),
    }
}

fn asm_from_cb(instructions: Vec<Instruction>) -> String {
    let mut asm = String::new();
    for instruction in instructions {
        asm.push_str(&format!(
            "{:?} {}\n",
            instruction.opcode,
            instruction
                .operands
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ));
    }
    asm
}

fn write(instructions: Vec<Instruction>, file: &str) {
    let asm = asm_from_cb(instructions);
    let mut file = File::create(file).expect("Unable to create file");
    file.write_all(asm.as_bytes())
        .expect("Unable to write data");
}

fn read(file: &str) -> Vec<Instruction> {
    let mut file = File::open(file).expect("Unable to open file");
    let mut asm = String::new();
    file.read_to_string(&mut asm).expect("Unable to read data");
    cb_from_asm(asm)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input file> [-o <output file>]", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = if args.len() > 3 && args[2] == "-o" {
        &args[3]
    } else {
        "output.asm"
    };

    let instructions = read(input_file);
    write(instructions, output_file);
}
