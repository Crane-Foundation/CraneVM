use crate::engine::memory;

use crate::structs::Opcode::{self};
use memory::Block;
use memory::Memory;
use std::io::Read;
// Registers are just stored in Memory struct

pub struct Engine {
    path: String,
    pub memory: Memory,
    pc: u16,
}

// Constructor
impl Engine {
    pub fn new(path: &str) -> Self {
        Engine {
            path: path.to_string(),
            memory: Memory::new(),
            pc: 0,
        }
    }
}

// Methods - Load File
impl Engine {
    //function to read a file in chunkcs of 4 bytes and return a Vector
    fn read_chunk(reader: &mut std::fs::File) -> [u8; 4] {
        let mut buffer = [0; 4];
        reader.read(&mut buffer).expect("Error reading file");
        buffer
    }
    //function to open a file and continually run read_chunk until eof
    pub fn load_file(&mut self) {
        let mut file = std::fs::File::open(&self.path).expect("File not found");
        let mut i = memory::PROGRAM_START;
        loop {
            let chunk = Engine::read_chunk(&mut file);
            if chunk == [0; 4] {
                break;
            }
            self.memory.write_block(Block::new(i, 4), &chunk);
            i += 4;
        }
        //print the program memory block
    }
}

type VmFunction = fn(&mut Engine, [u8; 3]);

// Methods - FDE Cycle
impl Engine {
    pub const FUNCS: [VmFunction; 38] = [
        Engine::add,
        Engine::sub,         //sub
        Engine::mul,         //mul
        Engine::placeholder, //div
        Engine::placeholder, //mod
        Engine::placeholder, //and
        Engine::placeholder, //or
        Engine::placeholder, //xor
        Engine::placeholder, //not
        Engine::placeholder, //shl
        Engine::placeholder, //shr
        Engine::placeholder, //cmp
        Engine::placeholder, //eq
        Engine::placeholder, // neq
        Engine::placeholder, // lt
        Engine::placeholder, // gt
        Engine::placeholder, // lte
        Engine::placeholder, // gte
        Engine::placeholder, // push
        Engine::placeholder, // pop
        Engine::placeholder, // dup
        Engine::placeholder, // swap
        Engine::placeholder, // jmp
        Engine::placeholder, // jmpt
        Engine::placeholder, // jmpf
        Engine::placeholder, // jz
        Engine::placeholder, // call
        Engine::placeholder, // ret
        Engine::placeholder, //load
        Engine::placeholder,
        Engine::store,
        Engine::mov,
        Engine::placeholder,
        Engine::placeholder,
        Engine::placeholder,
        Engine::placeholder,
        Engine::placeholder,
        Engine::halt,
    ];
    //decode the instruction - this returns the opcode and operands
    fn decode(&self, instruction: [u8; 4]) -> (Opcode, [u8; 3]) {
        let opcode = Opcode::from_char(instruction[0] as char).unwrap();
        let operands = [instruction[1], instruction[2], instruction[3]];
        (opcode, operands)
    }
    //execute the instruction
    fn execute(&mut self, opcode: Opcode, operands: [u8; 3]) {
        //execute a function based on the opcode, passing the operands
        Self::FUNCS[opcode as usize](self, operands);
        self.pc += 4;
    }
    //fetch the next instruction
    //decode the instruction - this just returns a buffer of 4 bytes
    fn fetch(&mut self) -> [u8; 4] {
        let from = memory::PROGRAM_START + self.pc as usize;
        let instruction = &self.memory.memory[from..from + 4];
        unsafe { *(instruction.as_ptr() as *const [u8; 4]) }
    }
    fn read_reg(&self, reg: u8) -> u16 {
        if self.sizeof_reg(reg) == 1 {
            self.memory.read_byte(reg as usize) as u16
        } else {
            let reg = self.memory.read_block(Block::new(reg as usize, 2));
            Self::bytes_to_u16([reg[0], reg[1]])
        }
    }

    //run the fetch-decode-execute cycle
    #[inline(never)]
    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch();
            let (opcode, operands) = self.decode(instruction);
            self.execute(opcode, operands);
        }
    }
    fn sizeof_reg(&self, reg: u8) -> usize {
        if reg < 16 {
            1
        } else {
            2
        }
    }
}

// Methods - Instructions
impl Engine {
    //add the operands
    fn add(&mut self, operands: [u8; 3]) {
        let reg = operands[0];
        let reg_val = self.read_reg(reg);
        let data = self.memory.read_block(Block::new(
            Engine::bytes_to_u16([operands[1], operands[2]]) as usize,
            2,
        ));
        let result = reg_val + Engine::bytes_to_u16([data[0], data[1]]);
        let result_bytes = Engine::u16_to_bytes(result);
        self.memory
            .write_block(Block::new(reg as usize, 2), &result_bytes);
    }

    //subtract the operands
    fn sub(&mut self, operands: [u8; 3]) {
        let reg = operands[0];
        let reg_val = self.read_reg(reg);
        let data = self.memory.read_block(Block::new(
            Engine::bytes_to_u16([operands[1], operands[2]]) as usize,
            2,
        ));
        let result = reg_val.wrapping_sub(Engine::bytes_to_u16([data[0], data[1]]));
        let result_bytes = Engine::u16_to_bytes(result);
        self.memory
            .write_block(Block::new(reg as usize, 2), &result_bytes);
    }

    fn mul(&mut self, operands: [u8; 3]) {
        let reg_value: u16 = self.read_reg(operands[0]);
        let data: &[u8] = self.memory.read_block(Block::new(
            Engine::bytes_to_u16([operands[1], operands[2]]) as usize,
            2,
        ));
        self.memory.write_block(
            Block::new(operands[0] as usize, 2),
            &Engine::u16_to_bytes(reg_value.wrapping_mul(Engine::bytes_to_u16([data[0], data[1]]))),
        )
    }

    fn store(&mut self, operands: [u8; 3]) {
        // Read memory (operands 2 and 3 are the memory address), store in register (operand 1)
        let reg = operands[0];
        let address = Engine::bytes_to_u16([operands[1], operands[2]]) as usize;
        //use move_within to move the data from the memory to the register (this is a bit of a cheat)
        self.memory
            .memory
            .copy_within(address..address + 2, memory::REG_START + reg as usize);
    }

    fn mov(&mut self, operands: [u8; 3]) {
        //move a value into a register
        let reg = operands[0];
        self.memory
            .write_block(Block::new(reg as usize, 2), &[operands[1], operands[2]]);
    }

    fn placeholder(&mut self, _operands: [u8; 3]) {
        println!("Test PLACEHOLDER");
    }

    //halt the engine
    fn halt(&mut self, operands: [u8; 3]) {
        println!("{:?}", self.memory.read_block(Block::new(16, 64)));
        std::process::exit(operands[2] as i32);
    }
}

// selection of functions to allow easy conversions between u16s and [u8; 2]s
impl Engine {
    fn u16_to_bytes(value: u16) -> [u8; 2] {
        [(value >> 8) as u8, value as u8]
    }
    fn bytes_to_u16(bytes: [u8; 2]) -> u16 {
        (bytes[0] as u16) << 8 | bytes[1] as u16
    }
}
