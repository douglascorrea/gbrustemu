use crate::{Instruction, MMU};

#[derive(Debug)]
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
            ld @ 0x01 | ld @ 0x11 | ld @ 0x21 | ld @ 0x31 => match ld {
                0x01 => Instruction::LdBc(d16),
                0x11 => Instruction::LdDe(d16),
                0x21 => Instruction::LdHl(d16),
                0x31 => Instruction::LdSp(d16),
                _ => panic!(
                    "DECODING LD: Unreconized byte {:#X} on pc {:#X}\n CPU STATE: {:?}",
                    byte, self.pc, self
                ),
            },
            0x32 => Instruction::LddHlA,
            0xAF => Instruction::XorA,
            0xA8 => Instruction::XorB,
            0xA9 => Instruction::XorC,
            0xAA => Instruction::XorD,
            0xAB => Instruction::XorE,
            0xAC => Instruction::XorH,
            0xAD => Instruction::XorL,
            0xAE => Instruction::XorHl,
            0xCB => match cb_opcode {
                0x7C => Instruction::BitbH(7),
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
                hl -= 1;
                self.h = ((hl & 0xFF00) >> 8) as u8;
                self.l = (hl & 0x00FF) as u8;
                self.pc += 1;
            }
            Instruction::XorA => {
                self.a ^= self.a;
                if self.a == 0 {
                    self.f = 0b10000000;
                }
                self.pc += 1;
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
