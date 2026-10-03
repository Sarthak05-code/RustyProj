#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
    Nop = 0x0,
    LoadI = 0x1,
    Add = 0x2,
    Sub = 0x3,
    Shl = 0x4,
    Jmp = 0x5,
    Print = 0x6,
    Store = 0x7, // RAM Store
    Load = 0x8,  // RAM Load
    Push = 0x9,  // Stack Push
    Pop = 0xA,   // Stack Pop
    Call = 0xB,  // Subroutine Call
    Ret = 0xC,   // Subroutine Return
    Halt = 0xF,
}

impl Opcode {
    fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x0 => Some(Opcode::Nop),
            0x1 => Some(Opcode::LoadI),
            0x2 => Some(Opcode::Add),
            0x3 => Some(Opcode::Sub),
            0x4 => Some(Opcode::Shl),
            0x5 => Some(Opcode::Jmp),
            0x6 => Some(Opcode::Print),
            0x7 => Some(Opcode::Store),
            0x8 => Some(Opcode::Load),
            0x9 => Some(Opcode::Push),
            0xA => Some(Opcode::Pop),
            0xB => Some(Opcode::Call),
            0xC => Some(Opcode::Ret),
            0xF => Some(Opcode::Halt),
            _ => None,
        }
    }
}

pub struct Machine {
    registers: [u16; 16], // R0..R15
    sp: u16,              // Dedicated Stack Pointer
    pc: usize,            // Program Counter
    memory: Vec<u16>,     // Instruction Code Memory
    ram: [u8; 65536],     // 64 KB Main RAM (Data & Call Stack)
    running: bool,
}

impl Machine {
    pub fn new(program: Vec<u16>) -> Self {
        Self {
            registers: [0; 16],
            sp: 0xFFFE, // Start stack near top of 64KB boundary
            pc: 0,
            memory: program,
            ram: [0; 65536],
            running: false,
        }
    }

    // Stack Helper: Push 16-bit word onto RAM stack (Big-Endian)
    fn push_u16(&mut self, val: u16) {
        self.sp = self.sp.wrapping_sub(2);
        let addr = self.sp as usize;
        self.ram[addr] = (val >> 8) as u8;
        self.ram[addr + 1] = (val & 0xFF) as u8;
    }

    // Stack Helper: Pop 16-bit word from RAM stack
    fn pop_u16(&mut self) -> u16 {
        let addr = self.sp as usize;
        let high = self.ram[addr] as u16;
        let low = self.ram[addr + 1] as u16;
        self.sp = self.sp.wrapping_add(2);
        (high << 8) | low
    }

    // Instruction Encoders
    pub fn make_load_i(dst: u8, immediate: u8) -> u16 {
        ((Opcode::LoadI as u16) << 12) | ((dst as u16 & 0x0F) << 8) | (immediate as u16 & 0xFF)
    }

    pub fn make_inst(op: Opcode, dst: u8, src1: u8, src2: u8) -> u16 {
        ((op as u16) << 12)
            | ((dst as u16 & 0x0F) << 8)
            | ((src1 as u16 & 0x0F) << 4)
            | (src2 as u16 & 0x0F)
    }

    fn decode(instruction: u16) -> (u8, usize, usize, usize) {
        let opcode = ((instruction >> 12) & 0x0F) as u8;
        let dst = ((instruction >> 8) & 0x0F) as usize;
        let src1 = ((instruction >> 4) & 0x0F) as usize;
        let src2 = (instruction & 0x0F) as usize;
        (opcode, dst, src1, src2)
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running && self.pc < self.memory.len() {
            let instruction = self.memory[self.pc];
            self.pc += 1;

            let (raw_opcode, dst, src1, src2) = Self::decode(instruction);
            let opcode = Opcode::from_u8(raw_opcode).unwrap_or(Opcode::Nop);

            match opcode {
                Opcode::Nop => {}

                Opcode::LoadI => {
                    let immediate = (instruction & 0xFF) as u16;
                    self.registers[dst] = immediate;
                }

                Opcode::Add => {
                    self.registers[dst] = self.registers[src1].wrapping_add(self.registers[src2]);
                }

                Opcode::Sub => {
                    self.registers[dst] = self.registers[src1].wrapping_sub(self.registers[src2]);
                }

                Opcode::Shl => {
                    self.registers[dst] = self.registers[src1] << self.registers[src2];
                }

                Opcode::Jmp => {
                    self.pc = self.registers[dst] as usize;
                }

                Opcode::Print => {
                    println!("[VM Print] R{}: {}", src1, self.registers[src1]);
                }

                // RAM Memory Operations
                Opcode::Store => {
                    // RAM[Reg[Src2]] = Reg[Src1]
                    let addr = self.registers[src2] as usize;
                    let val = self.registers[src1];
                    self.ram[addr] = (val >> 8) as u8;
                    self.ram[addr + 1] = (val & 0xFF) as u8;
                }

                Opcode::Load => {
                    // Reg[Dst] = RAM[Reg[Src1]]
                    let addr = self.registers[src1] as usize;
                    let high = self.ram[addr] as u16;
                    let low = self.ram[addr + 1] as u16;
                    self.registers[dst] = (high << 8) | low;
                }

                // Stack Operations
                Opcode::Push => {
                    let val = self.registers[src1];
                    self.push_u16(val);
                }

                Opcode::Pop => {
                    let val = self.pop_u16();
                    self.registers[dst] = val;
                }

                // Subroutine/Function Calls
                Opcode::Call => {
                    // Save return address (current PC) onto stack
                    let return_addr = self.pc as u16;
                    self.push_u16(return_addr);

                    // Jump to function target address stored in Reg[Dst]
                    self.pc = self.registers[dst] as usize;
                }

                Opcode::Ret => {
                    // Restore PC from stack top
                    let return_addr = self.pop_u16();
                    self.pc = return_addr as usize;
                }

                Opcode::Halt => {
                    self.running = false;
                }
            }
        }
    }
}

pub fn main() {
    // Demo: Calling a "Double" function
    // ----------------------------------
    // Function at address PC=7:
    //   Calculates R0 = R0 + R0, then RET
    //
    // Main Program (PC 0..6):
    //   1. LOADI R0, 21       (Input argument = 21)
    //   2. LOADI R1, 7        (R1 = Function target address)
    //   3. CALL  R1           (Push PC=3, Jump to PC=7)
    //   4. PRINT R0           (Should print 42)
    //   5. HALT

    let program = vec![
        /* 0 */ Machine::make_load_i(0, 21),
        /* 1 */ Machine::make_load_i(1, 7),
        /* 2 */ Machine::make_inst(Opcode::Call, 1, 0, 0),
        /* 3 */ Machine::make_inst(Opcode::Print, 0, 0, 0),
        /* 4 */ Machine::make_inst(Opcode::Halt, 0, 0, 0),
        /* 5 */ Machine::make_inst(Opcode::Nop, 0, 0, 0), // Padding
        /* 6 */ Machine::make_inst(Opcode::Nop, 0, 0, 0), // Padding
        /* 7 */ Machine::make_inst(Opcode::Add, 0, 0, 0), // Function: R0 = R0 + R0
        /* 8 */ Machine::make_inst(Opcode::Ret, 0, 0, 0), // Function: Return
    ];

    let mut vm = Machine::new(program);
    vm.run();
}
