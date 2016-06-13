/*  chip8.rs
 *  Provides an implementation/interpreter of the chip8 system */

use std::io;
use std::io::Error;
use std::io::prelude::*;
use std::fs::File;

use std::num::Wrapping;

extern crate rand;
use rand::Rng;

pub struct Chip8 {
    pub fontset: [u8; 80],
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub i: u16,
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
            i: 0u16,
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

        let mut f = try!(File::open("test2.ch8"));
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


        let mut addr = 0u16;

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0xFFFF {
                    0x00E0 => {
                        //Clear display
                        println!("00E0");
                        self.display = [[false; 64]; 32];
                        self.pc += 2;
                    },
                    0x00EE => {
                        //Return from a subroutine
                        println!("00EE");
                        self.pc = self.stack[self.sp as usize];
                        self.sp -= 1;
                    },
                    _ => { //0NNN
                        //Ignore this, not needed
                        println!("0NNN");
                        self.pc += 2;
                    }
                }
            },
            0x1000 => {
                //Jump
                println!("1NNN");
                self.pc = opcode & 0x0FFF;
            },
            0x2000 => {
                //Call subroutine
                println!("2NNN");
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = opcode & 0x0FFF;
            },
            0x3000 => {
                //Skip next instruction if VX = NN
                println!("3XNN");
                //if self.v[(opcode & 0x0F00) >> 8 as usize] == (opcode & 0x00FF) as u8 {
                //let vx = self.v[((opcode & 0x0F00) >> 8) as usize];
                if self.v[((opcode & 0x0F00) >> 8) as usize] == (opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x4000 => {
                //Skip next instruction if VX != NN
                println!("4XNN");
                //if self.v[(opcode & 0x0F00) >> 8 as usize] == (opcode & 0x00FF) as u8 {
                //let vx = self.v[((opcode & 0x0F00) >> 8) as usize];
                if self.v[((opcode & 0x0F00) >> 8) as usize] != (opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x5000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        // Skip next instruction if VX = VY.
                        println!("5XY0");
                        let x = self.v[((opcode & 0x0F00) >> 8) as usize];
                        let y = self.v[((opcode & 0x00F0) >> 4) as usize];
                        if x == y {
                            self.pc += 4;
                        }
                    },
                    _ => {
                        println!("Invalid opcode: {:X}", opcode);
                    }
                }
            },
            0x6000 => {
                //Set VX = NN
                println!("6XNN");
                let x = opcode & 0x0F00 >> 8;
                let n = opcode & 0x00FF;
                self.v[x as usize] = n as u8;
                self.pc += 2;
            },
            0x7000 => {
                //Set VX = VX + NN.
                println!("7XNN");
                let x = opcode & 0x0F00 >> 8;
                let n = opcode & 0x00FF;
                self.v[x as usize] += n as u8;
                self.pc += 2;
            },
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        println!("8XY0");
                        //Set VX = VY.
                        let x = opcode & 0x0F00 >> 8;
                        let y = opcode & 0x00F0 >> 4;
                        self.v[x as usize] = self.v[y as usize];
                        self.pc += 2;
                    },
                    0x0001 => {
                        println!("8XY1");
                        //Set VX OR VY.
                        let x = opcode & 0x0F00 >> 8;
                        let y = opcode & 0x00F0 >> 4;
                        self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
                        self.pc += 2;
                    },
                    0x0002 => {
                        println!("8XY2");
                        //Set VX OR VY.
                        let x = opcode & 0x0F00 >> 8;
                        let y = opcode & 0x00F0 >> 4;
                        self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
                        self.pc += 2;
                    },
                    0x0003 => {
                        println!("8XY3");
                        //Set VX OR VY.
                        let x = opcode & 0x0F00 >> 8;
                        let y = opcode & 0x00F0 >> 4;
                        self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
                        self.pc += 2;
                    },
                    0x0004 => {
                        println!("8XY4");
                        //Add VX, VY, set VF=carry.
                        let x = opcode & 0x0F00 >> 8;
                        let y = opcode & 0x00F0 >> 4;
                        let addition: u16 = (self.v[x as usize] + self.v[y as usize]) as u16;
                        self.v[x as usize] = addition as u8;
                        self.v[15] = (addition & 0x0100 >> 8) as u8;
                        self.pc += 2;
                    },
                    0x0005 => {
                        //Sub VX, VY, set VF=(VX > VY)
                        println!("8XY5");
                        let x = opcode & 0x0F00 >> 8;
                        let y = opcode & 0x00F0 >> 4;
                        if self.v[x as usize] > self.v[y as usize] {
                            self.v[15] = 1u8;
                        } else {
                            self.v[15] = 0u8;
                        }
                        let sub = Wrapping(self.v[x as usize]) - Wrapping(self.v[y as usize]);
                        self.v[x as usize] = sub.0;
                        self.pc += 2;
                    },
                    0x0006 => {
                        // Vx >>= 1, VF=LSB
                        println!("8XY6");
                        let x = opcode & 0x0F00 >> 8;
                        let vx = self.v[x as usize];
                        if (vx & 1u8) == 1u8 {
                            self.v[15] = 1u8;
                        } else {
                            self.v[15] = 0u8;
                        }

                        self.v[x as usize] = vx >> 1;
                        self.pc += 2;
                    },
                    0x0007 => {
                        // VX = VY - VX, set VF=(VY > VX)
                        println!("8XY7");
                        let x = opcode & 0x0F00 >> 8;
                        let y = opcode & 0x00F0 >> 4;
                        if self.v[y as usize] > self.v[x as usize] {
                            self.v[15] = 1u8;
                        } else {
                            self.v[15] = 0u8;
                        }
                        let sub = Wrapping(self.v[y as usize]) - Wrapping(self.v[x as usize]);
                        self.v[x as usize] = sub.0;
                        self.pc += 2;
                    },
                    0x000E => {
                        // VX <<= 1, VF=MSB
                        println!("8XYE");
                        let x = opcode & 0x0F00 >> 8;
                        let vx = self.v[x as usize];
                        if (vx & 0x80) == 0x80 {
                            self.v[15] = 1u8;
                        } else {
                            self.v[15] = 0u8;
                        }

                        self.v[x as usize] = vx << 1;
                        self.pc += 2;
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
                        let x = opcode & 0x0F00 >> 8;
                        let y = opcode & 0x00F0 >> 4;
                        if self.v[x as usize] != self.v[y as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        println!("Invalid opcode: {:X}", opcode);
                    }
                }

            },
            0xA000 => {
                // Set I = NNN
                println!("ANNN");
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            },
            0xB000 => {
                // Jump to NNN + V0
                println!("BNNN");
                self.pc = (opcode & 0x0FFF) + (self.v[0] as u16);
            },
            0xC000 => {
                //Set VX = random byte AND NN
                println!("CXNN");
                let x = opcode & 0x0F00 >> 8;
                let n = opcode as u8;
                let mut rng = rand::thread_rng();
                self.v[x as usize] = n & rng.gen::<u8>();
                self.pc += 2;
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