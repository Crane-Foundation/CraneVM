#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    // Arithmetic Operations
    ADD, // Addition
    SUB, // Subtraction
    MUL, // Multiplication
    DIV, // Division
    MOD, // Modulus
    INC, // Increment
    DEC, // Decrement

    // Logical Operations
    AND, // Logical AND
    OR,  // Logical OR
    XOR, // Logical XOR
    NOT, // Logical NOT
    SHL, // Shift Left
    SHR, // Shift Right

    // Comparison Operations
    EQ,  // Compare Equal
    NEQ, // Compare Not Equal
    LT,  // Compare Less Than
    GT,  // Compare Greater Than
    LTE, // Compare Less Than or Equal
    GTE, // Compare Greater Than or Equal

    // Stack Manipulation
    PUSH, // Push to stack
    POP,  // Pop from stack
    DUP,  // Duplicate top of stack
    SWAP, // Swap top two elements of stack

    // Control Flow
    JMP,  // Unconditional Jump
    JMPT, // Jump if true
    JMPF, // Jump if false
    JZ,   // Jump if zero
    CALL, // Call subroutine
    RET,  // Return from subroutine

    // Memory Operations
    LOAD,  // Load from memory
    STORE, // Store to memory

    // Register Operations
    MOV, // Move data between registers

    // Input/Output Operations
    IN,  // Input
    OUT, // Output

    // Function Operations
    LAB, // Declare a label

    // System Operations
    SYS, // System call

    // Miscellaneous
    NOP,  // No Operation
    HALT, // Halt execution
}

impl Opcode {
    pub fn from_char(c: char) -> Option<Opcode> {
        unsafe {
            //do a transmute to convert the char to the enum variant
            Some(std::mem::transmute(c as u8))
        }
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe {
            //do a transmute to convert the u8 to the enum variant
            std::mem::transmute(value)
        }
    }
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,    // 1 byte for the opcode
    pub operands: [u8; 3], // 3 bytes for the operands
}

impl Instruction {
    pub fn new(opcode: Opcode, operands: [u8; 3]) -> Self {
        Instruction { opcode, operands }
    }
    pub fn stringify(&self) -> String {
        //create a string of length 4
        let mut s = String::with_capacity(4);
        //push the opcode to the string
        s.push_str(&format!("{:?}", self.opcode));
        //push the operands to the string
        for operand in self.operands.iter() {
            s.push_str(&format!("{}", *operand as char));
        }
        s
    }
}

//Macros to build instructions
#[macro_export]
macro_rules! instruction {
    ($opcode:ident, $op1:expr, $op2:expr, $op3:expr) => {
        crate::structs::Instruction::new(crate::structs::Opcode::$opcode, [$op1, $op2, $op3])
    };
    ($opcode:ident, $op1:expr, $op2:expr) => {
        //op2 will be split into high and low bytes
        crate::structs::Instruction::new(
            crate::structs::Opcode::$opcode,
            [$op1 as u8, ($op2 >> 8u8) as u8, ($op2 & 0xFF) as u8],
        )
    };
    ($opcode:ident, $op1:expr) => {
        //op1 will be split into high and low bytes
        crate::structs::Instruction::new(
            crate::structs::Opcode::$opcode,
            [
                ($op1 >> 16u8) as u8,
                ($op1 >> 8u8) as u8,
                ($op1 & 0xFF) as u8,
            ],
        )
    };
    ($opcode:ident) => {
        crate::structs::Instruction::new(crate::structs::Opcode::$opcode, [0, 0, 0])
    };
    () => {
        crate::structs::Instruction::new(crate::structs::Opcode::NOP, [0, 0, 0])
    };
}
