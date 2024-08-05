// Take a vector of Structs/Instruction and write it to a file

use crate::structs::Instruction;
use std::fs::File;
use std::io::Write;

pub fn write_file(file_name: &str, instructions: Vec<Instruction>) {
    //open the file
    let mut file = File::create(file_name).expect("Error creating file");
    //loop through the instructions
    for instruction in instructions {
        //write the opcode to the file
        file.write(&[instruction.opcode as u8])
            .expect("Error writing file");
        //write the operands to the file
        file.write(&instruction.operands)
            .expect("Error writing file");
    }
}
