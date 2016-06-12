/*  chip8.rs
 *  Provides an implementation/interpreter of the chip8 system */

pub struct Chip8 {
    pub fontset: [u8; 80],
    pub memory: [u8; 4096],
    pub r: [u8; 16],
    pub I: u16,
    pub DT: u8,
    pub ST: u8,
    pub PC: u16,
    pub SP: u8,
    pub stack: [u16; 16],
    pub keyboard: [bool; 16],
    pub display: [bool; 64*32],
}

impl Default for Chip8 {
    fn default() -> Chip8 {
        Chip8{
            fontset: [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80],
            memory: [0u8; 4096],
            r: [0u8; 16],
            I: 0u16,
            DT: 0u8,
            ST: 0u8,
            PC: 0u16,
            SP: 0u8,
            stack: [0u16; 16],
            keyboard: [false; 16],
            display: [false; 64*32],
        }
    }
}

impl Chip8 {
    //TODO: implement ROM loading and starting
    //TODO: implement all opcodes, timers
    pub fn emulate_cycle(self) {
        let mut x = 0u8;
        let mut y = 0u8;
        let mut height = 0u8;
        let mut pixel = 0u8;

        //Look at old incomplete C++ code for reference...
        //Fetch
        let mut opcode = (self.memory[self.PC as usize] as u16) << 8 | (self.memory[self.PC as usize + 1] as u16);
        //Decode/Execute
        
        match opcode & 0xF000 {
            0x0000 => {
                opcode;
            },
            _ => {
                println!("Can't find opcode: {}", opcode);
            }
        }
        //Timers 
    }
}