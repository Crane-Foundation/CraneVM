// I want a contiguous block of memory that I can write to, read from

//constant sizes

pub const DATA_SIZE: usize = 1024 * 2; // registers, constants, etc
pub const STACK_SIZE: usize = 1024 * 4; // 4KB
pub const PROGRAM_SIZE: usize = 1024 * 4; // 8KB stores the program
pub const HEAP_SIZE: usize = 1024 * 16; // 16KB

pub const REG_START: usize = 0;
pub const REG_SIZE: usize = 2 * 16 * 2 + 2 * 8 * 2; // 16 8-bit registers and 8 16-bit registers
pub const PC: usize = REG_SIZE + Register::Pc as usize;
pub const PROGRAM_START: usize = DATA_SIZE + STACK_SIZE;
pub const RETURN_STACK_START: usize = DATA_SIZE + REG_SIZE;
pub const CONSTANTS_8: usize = DATA_SIZE + STACK_SIZE + PROGRAM_SIZE; // 256 8-bit constants
pub const CONSTANTS_16: usize = CONSTANTS_8 + 1; // 256 16-bit constants
pub const CONSTANTS_END: usize = CONSTANTS_16 + 1 + 255;
//memory layout
pub struct Memory {
    pub memory: [u8; STACK_SIZE + PROGRAM_SIZE + HEAP_SIZE + DATA_SIZE],
    pub stack: Block,
    pub program: Block,
    pub heap: Block,
    pub data: Block,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; STACK_SIZE + PROGRAM_SIZE + HEAP_SIZE + DATA_SIZE],
            data: Block::new(0, DATA_SIZE - 1),
            stack: Block::new(DATA_SIZE, STACK_SIZE),
            program: Block::new(DATA_SIZE + STACK_SIZE, PROGRAM_SIZE),
            heap: Block::new(DATA_SIZE + STACK_SIZE + PROGRAM_SIZE, HEAP_SIZE),
        }
    }
    //read a byte from memory
    pub fn read_byte(&self, address: usize) -> u8 {
        self.memory[address]
    }
    //write a byte to memory
    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }
    //Stack
    pub fn push(&mut self, value: u8) {
        self.stack.size -= 1;
        self.write_byte(self.stack.size, value);
    }
    pub fn pop(&mut self) -> u8 {
        let value = self.read_byte(self.stack.size);
        self.stack.size += 1;
        value
    }
    pub fn swap(&mut self, value: u8) -> u8 {
        let old = self.pop();
        self.push(value);
        old
    }
    pub fn write_block(&mut self, block: Block, data: &[u8]) {
        self.memory[block.start..block.start + block.size].copy_from_slice(data)
    }
    //write block of memory
    // pub fn write_block(&mut self, block: Block, data: &[u8]) {
    //     for (i, &byte) in data.iter().enumerate() {
    //         self.write_byte(block.start + i, byte);
    //     }
    // }

    //read block of memory, do give a slice back, no vectors
    pub fn read_block(&self, block: Block) -> &[u8] {
        &self.memory[block.start..block.start + block.size]
    }
}

#[repr(u8)]
pub enum Register {
    // 8 bit
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    // Function
    Ax,
    Bx,
    Cx,
    Dx,
    Si,
    Di,
    Bp,
    Sp,
    Pc,
    // 16 bit
    //Data
    Er0,
    Er1,
    Er2,
    Er3,
    Er4,
    Er5,
    Er6,
    Er7,
    // Function
    Eax,
    Ebx,
    Ecx,
    Edx,
    Esi,
    Edi,
    Ebp,
    Esp,
}
impl From<u8> for Register {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

pub struct Block {
    pub start: usize,
    pub size: usize,
}

impl Block {
    pub fn new(start: usize, size: usize) -> Self {
        Block { start, size }
    }
}

//page struct, just an intermediary between the memory and files
pub struct Page {
    pub data: [u8; STACK_SIZE + PROGRAM_SIZE + HEAP_SIZE + DATA_SIZE],
    pub program_id: usize,
}
