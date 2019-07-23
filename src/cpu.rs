use crate::instruction::Instruction;
use crate::mmu::MMU;

use core::num::FpCategory::Infinite;
use std::fmt;

pub struct CPU {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU {{ A: {:#X}, B: {:#X}, C: {:#X}, D: {:#X}, E: {:#X}, H: {:#X}, L: {:#X} }} \nflags: {{ Z: {:?}, N: {:?}, H: {:?}, C: {:?} }}\n{{ pc: {:#X}, sp: {:#X} }}",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.get_z_flag(),
            self.get_n_flag(),
            self.get_h_flag(),
            self.get_c_flag(),
            self.pc,
            self.sp
        )
    }
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        };
        cpu
    }

    fn get_z_flag(&self) -> bool {
        let only_z_on_f = self.f & 0b1000_0000;
        let z_flag: bool = (only_z_on_f >> 7) == 1;
        z_flag
    }
    fn get_n_flag(&self) -> bool {
        let only_n_on_f = self.f & 0b0100_0000;
        let n_flag: bool = (only_n_on_f >> 7) == 1;
        n_flag
    }
    fn get_h_flag(&self) -> bool {
        let only_h_on_f = self.f & 0b0010_0000;
        let h_flag: bool = (only_h_on_f >> 7) == 1;
        h_flag
    }
    fn get_c_flag(&self) -> bool {
        let only_c_on_f = self.f & 0b0001_0000;
        let c_flag: bool = (only_c_on_f >> 7) == 1;
        c_flag
    }

    fn set_z_flag(&mut self) {
        self.f = self.f | 0b1000_0000;
    }

    fn reset_z_flag(&mut self) {
        self.f = self.f & 0b0111_1111;
    }
    fn set_n_flag(&mut self) {
        self.f = self.f | 0b0100_0000;
    }
    fn reset_n_flag(&mut self) {
        self.f = self.f & 0b1011_1111;
    }
    fn set_h_flag(&mut self) {
        self.f = self.f | 0b0010_0000;
    }
    fn reset_h_flag(&mut self) {
        self.f = self.f & 0b1101_1111;
    }
    fn set_c_flag(&mut self) {
        self.f = self.f | 0b0001_0000;
    }
    fn reset_c_flag(&mut self) {
        self.f = self.f & 0b1110_1111;
    }

    fn do_bit_opcode(&mut self, bit: bool) {
        if bit {
            self.set_z_flag();
        }
        self.reset_n_flag();
        self.set_h_flag();
        self.pc += 2;
    }

    fn decode(&mut self, byte: u8, mmu: &MMU) -> Instruction {
        // prepare some special variables
        // the immediate 16 bit
        let n1 = mmu.read_byte(self.pc + 1) as u16;
        let n2 = mmu.read_byte(self.pc + 2) as u16;
        // inverting position because it is BIG ENDIAN with bitwise operation
        let d16: u16 = (n2 << 8) | n1;

        // in case of prefix CB n1 is OPCODE
        let cb_opcode = n1;
        // in case of prefix CB n2 is n1
        let cb_n1 = n2;
        let cb_n2 = mmu.read_byte(self.pc + 3) as u16;
        // inverting position because it is BIG ENDIAN with bitwise operation
        let cb_d16: u16 = (cb_n2 << 8) | cb_n1;

        match byte {
            0x01 => Instruction::LdBc(d16),
            0x06 => Instruction::LdB(n1 as u8),
            0x0E => Instruction::LdC(n1 as u8),
            0x16 => Instruction::LdD(n1 as u8),
            0x1E => Instruction::LdE(n1 as u8),
            0x26 => Instruction::LdH(n1 as u8),
            0x2E => Instruction::LdL(n1 as u8),
            0x11 => Instruction::LdDe(d16),
            0x21 => Instruction::LdHl(d16),
            0x31 => Instruction::LdSp(d16),
            0x20 => Instruction::JrNz(n1 as i8),
            0x28 => Instruction::JrZ(n1 as i8),
            0x30 => Instruction::JrNc(n1 as i8),
            0x32 => Instruction::LddHlA,
            0x38 => Instruction::JrC(n1 as i8),
            0xAF => Instruction::XorA,
            0xA8 => Instruction::XorB,
            0xA9 => Instruction::XorC,
            0xAA => Instruction::XorD,
            0xAB => Instruction::XorE,
            0xAC => Instruction::XorH,
            0xAD => Instruction::XorL,
            0xAE => Instruction::XorHl,
            0xCB => match cb_opcode {
                0x7C => Instruction::BitbH(0b01000000),
                _ => panic!(
                    "DECODING CB PREFIX: Unreconized cb_opcode {:#X} on pc {:#X}\n CPU STATE: {:?}",
                    cb_opcode, self.pc as u16, self
                ),
            },
            0xEE => Instruction::Xor(n1 as u8),
            _ => panic!(
                "DECODING: Unreconized byte {:#X} on pc {:#X}\n CPU STATE: {:?}",
                byte, self.pc as u16, self
            ),
        }
    }
    fn execute(&mut self, instruction: &Instruction, mmu: &mut MMU) {
        match instruction {
            Instruction::LdSp(d16) => {
                self.sp = *d16;
                self.pc += 3;
            }
            Instruction::LdBc(d16) => {
                self.b = ((d16 & 0xFF00) >> 8) as u8;
                self.c = (d16 & 0x00FF) as u8;
                self.pc += 3;
            }
            Instruction::LdDe(d16) => {
                self.d = ((d16 & 0xFF00) >> 8) as u8;
                self.e = (d16 & 0x00FF) as u8;
                self.pc += 3;
            }
            Instruction::LdHl(d16) => {
                self.h = ((d16 & 0xFF00) >> 8) as u8;
                self.l = (d16 & 0x00FF) as u8;
                self.pc += 3;
            }
            Instruction::LddHlA => {
                let h16 = (self.h as u16) << 8;
                let mut hl: u16 = h16 | (self.l as u16);
                mmu.write_byte(hl, self.a);
                if hl == 0 {
                    //@TODO Check if this is right
                    self.set_h_flag();
                }
                hl = hl.wrapping_sub(1);
                self.h = ((hl & 0xFF00) >> 8) as u8;
                self.l = (hl & 0x00FF) as u8;
                self.pc += 1;
                if hl == 0 {
                    self.set_z_flag();
                }
                self.set_n_flag();
            }
            Instruction::XorA => {
                self.a ^= self.a;
                if self.a == 0 {
                    self.f = 0b10000000;
                }
                self.pc += 1;
            }
            Instruction::BitbA(bit_mask) => {
                let bit_test: u8 = self.a & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
            }
            Instruction::BitbB(bit_mask) => {
                let bit_test: u8 = self.b & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
            }
            Instruction::BitbC(bit_mask) => {
                let bit_test: u8 = self.c & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
            }
            Instruction::BitbE(bit_mask) => {
                let bit_test: u8 = self.e & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
            }
            Instruction::BitbH(bit_mask) => {
                let bit_test: u8 = self.h & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
            }
            Instruction::BitbL(bit_mask) => {
                let bit_test: u8 = self.l & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
            }
            Instruction::JrNz(n) => {
                self.sp = self.pc;
                self.pc = self.pc + 2;
                if !self.get_z_flag() {
                    self.pc = self.pc.wrapping_add(*n as u16);
                }
            }
            Instruction::JrZ(n) => {
                self.sp = self.pc;
                self.pc = self.pc + 2;
                if self.get_z_flag() {
                    self.pc = self.pc.wrapping_add(*n as u16);
                }
            }
            Instruction::JrNc(n) => {
                self.sp = self.pc;
                self.pc = self.pc + 2;
                if !self.get_c_flag() {
                    self.pc = self.pc.wrapping_add(*n as u16);
                }
            }
            Instruction::JrC(n) => {
                self.sp = self.pc;
                self.pc = self.pc + 2;
                if self.get_c_flag() {
                    self.pc = self.pc.wrapping_add(*n as u16);
                }
            }
            _ => panic!(
                "EXECUTING: Unreconized instruction {:?} on pc {:#X}\n CPU STATE: {:?}",
                instruction, self.pc, self
            ),
        }
    }

    pub fn run_instruction(&mut self, mmu: &mut MMU) {
        // fetch
        let byte = mmu.read_byte(self.pc);
        // decode
        let instruction = self.decode(byte, mmu);
        // execute
        self.execute(&instruction, mmu);
    }
}
