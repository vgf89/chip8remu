/*  chip8.rs
 *  Provides an implementation/interpreter of the chip8 system */

use std::io;
use std::io::Error;
use std::io::prelude::*;
use std::fs::File;

pub struct Chip8 {
    pub fontset: [u8; 80],
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub I: u16,
    pub dt: u8,
    pub st: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub keyboard: [bool; 16],
    pub display: [[bool; 64]; 32],
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
            v: [0u8; 16],
            I: 0u16,
            dt: 0u8,
            st: 0u8,
            pc: 0u16,
            sp: 0u8,
            stack: [0u16; 16],
            keyboard: [false; 16],
            display: [[false; 64]; 32],
        }
    }
}

impl Chip8 {
    /** Loads a (static) file into memory, at least for now */
    pub fn load_rom(&mut self) -> Result<(), Error> {

        let mut f = try!(File::open("test.ch8"));
        println!("Hello!");
        let mut i = 0;
        for byte in f.bytes() {
            //print!("{}", byte.unwrap() as char);
            self.memory[i] = byte.unwrap() as u8;
            i += 1;
        }

        Ok(())
    }

    /** Runs a cycle on the chip8 */
    pub fn emulate_cycle(&mut self) {
        let mut x = 0u8;
        let mut y = 0u8;
        let mut height = 0u8;
        let mut pixel = 0u8;

        //Fetch
        let opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);
        //Decode/Execute
        
        /* opcodes:
        1NNN 2NNN 3XNN 4XNN 5XY0 6XNN 7XNN 8XY0 8XY1 8XY2 8XY3 8XY4 8XY5 8XY6 8XY7 8XYE 9XY0 ANNN BNNN
        CXNN SXYN EX9E EXA1 FX07 FX0A FX15 FX18 FX1E FX29 FX33 FX55 FX64 */


        

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0xFFFF {
                    0x00E0 => {
                        println!("00E0");
                        self.pc += 2;
                        //TODO
                    },
                    0x00EE => {
                        println!("00EE");
                        self.pc += 2;
                        //TODO
                    }
                    _ => { //0NNN
                        println!("0NNN");
                        self.pc += 2;
                        //TODO
                    }
                }
            },
            0x1000 => {
                println!("1NNN");
                self.pc += 2;
                //TODO
            },
            0x2000 => {
                println!("2NNN");
                self.pc += 2;
                //TODO
            },
            0x3000 => {
                println!("3XNN");
                self.pc += 2;
                //TODO
            },
            0x4000 => {
                println!("4XNN");
                self.pc += 2;
                //TODO
            },
            0x5000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        println!("5XY0");
                        self.pc += 2;
                        //TODO
                    },
                    _ => {
                        println!("Invalid opcode: {:X}", opcode);
                    }
                }
            },
            0x6000 => {
                println!("6XNN");
                self.pc += 2;
                //TODO
            },
            0x7000 => {
                println!("7XNN");
                self.pc += 2;
                //TODO
            },
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        println!("8XY0");
                        self.pc += 2;
                        //TODO
                    },
                    0x0001 => {
                        println!("8XY1");
                        self.pc += 2;
                        //TODO
                    },
                    0x0002 => {
                        println!("8XY2");
                        self.pc += 2;
                        //TODO
                    },
                    0x0003 => {
                        println!("8XY3");
                        self.pc += 2;
                        //TODO
                    },
                    0x0004 => {
                        println!("8XY4");
                        self.pc += 2;
                        //TODO
                    },
                    0x0005 => {
                        println!("8XY5");
                        self.pc += 2;
                        //TODO
                    },
                    0x0006 => {
                        println!("8XY6");
                        self.pc += 2;
                        //TODO
                    },
                    0x0007 => {
                        println!("8XY7");
                        self.pc += 2;
                        //TODO
                    },
                    0x000E => {
                        println!("8XYE");
                        self.pc += 2;
                        //TODO
                    },
                    _ => {
                        println!("Invalid opcode: {:X}", opcode);
                    }
                }
            },
            0x9000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        println!("9XY0");
                        self.pc += 2;
                        //TODO
                    },
                    _ => {
                        println!("Invalid opcode: {:X}", opcode);
                    }
                }

            },
            0xA000 => {
                println!("ANNN");
                self.pc += 2;
                //TODO
            },
            0xB000 => {
                println!("BNNN");
                self.pc += 2;
                //TODO
            },
            0xC000 => {
                println!("CXNN");
                self.pc += 2;
                //TODO
            },
            0xD000 => {
                println!("DXYN");
                self.pc += 2;
                //TODO
            },
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => {
                        println!("EX9E");
                        self.pc += 2;
                        //TODO
                    },
                    0x00A1 => {
                        println!("EXA1");
                        self.pc += 2;
                        //TODO
                    },
                    _ => {
                        println!("Invalid opcode: {:X}", opcode);
                        //TODO
                    }
                }
            }
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => {
                        println!("FX07");
                        self.pc += 2;
                        //TODO
                    },
                    0x000A => {
                        println!("FX0A");
                        self.pc += 2;
                        //TODO
                    },
                    0x0015 => {
                        println!("FX15");
                        self.pc += 2;
                        //TODO
                    },
                    0x0018 => {
                        println!("FX18");
                        self.pc += 2;
                        //TODO
                    },
                    0x001E => {
                        println!("FX1E");
                        self.pc += 2;
                        //TODO
                    },
                    0x0029 => {
                        println!("FX29");
                        self.pc += 2;
                        //TODO
                    },
                    0x0033 => {
                        println!("FX33");
                        self.pc += 2;
                        //TODO
                    },
                    0x0055 => {
                        println!("FX55");
                        self.pc += 2;
                        //TODO
                    },
                    0x0065 => {
                        println!("FX65");
                        self.pc += 2;
                        //TODO
                    },
                    _ => {
                        println!("Invalid opcode: {:X}", opcode);
                    }
                }
            },

            _ => {
                println!("Invalid opcode: {:X}", opcode);
            }
        }
        //Timers 
    }
}