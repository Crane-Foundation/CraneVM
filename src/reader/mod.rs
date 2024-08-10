//this will read a file in groups of 4 bytes and return a vector of Instruction

use crate::structs::Instruction;
use std::fs::File;
use std::io::Read;

pub fn read_file(file_name: &str) -> Vec<Instruction> {
    //open the file
    let mut file = File::open(file_name).expect("File not found");
    //create a vector to store the instructions
    let mut instructions = Vec::new();
    //create a buffer to store the bytes
    let mut buffer = [0; 4];
    //loop through the file
    loop {
        //read 4 bytes into the buffer
        let bytes_read = file.read(&mut buffer).expect("Error reading file");
        //if we read 0 bytes, we are done
        if bytes_read == 0 {
            break;
        }
        //create an instruction from the buffer
        let instruction = Instruction::new(buffer[0].into(), [buffer[1], buffer[2], buffer[3]]);
        //push the instruction to the vector
        instructions.push(instruction);
    }
    //return the vector of instructions
    instructions
}
