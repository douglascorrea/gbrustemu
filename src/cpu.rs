use crate::instruction::Instruction;
use crate::mmu::MMU;

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
    t: usize,
    m: usize,
    debug: bool,
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
            t: 0,
            m: 0,
            debug: false,
        };
        cpu
    }

    pub fn set_debug_flag(&mut self) {
        self.debug = true;
    }
    pub fn reset_debug_flag(&mut self) {
        self.debug = false;
    }

    fn get_flag(&self, bit_mask: u8) -> bool {
        (self.f & bit_mask) != 0
    }
    fn get_z_flag(&self) -> bool {
        self.get_flag(0b1000_0000)
    }
    fn get_n_flag(&self) -> bool {
        self.get_flag(0b0100_0000)
    }
    fn get_h_flag(&self) -> bool {
        self.get_flag(0b0010_0000)
    }
    fn get_c_flag(&self) -> bool {
        self.get_flag(0b0001_0000)
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

    pub fn push_to_stack(&mut self, mmu: &mut MMU, addr: u16) {
        // write the address being pushed to the current stack pointer
        let addr_0: u8 = ((addr & 0xFF00) >> 8) as u8;
        let addr_1: u8 = (addr & 0x00FF) as u8;
        mmu.write_byte(self.sp, addr_1);
        self.sp -= 1;
        mmu.write_byte(self.sp, addr_0);
        self.sp -= 1;
    }

    pub fn pop_from_stack(&mut self, mmu: &MMU) -> u16 {
        self.sp += 1;
        let addr_0 = mmu.read_byte(self.sp);
        self.sp += 1;
        let addr_1 = mmu.read_byte(self.sp);
        let addr_016 = (addr_0 as u16) << 8;
        let addr: u16 = addr_016 | (addr_1 as u16);
        addr
    }

    fn do_bit_opcode(&mut self, bit: bool) {
        if bit {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }
        self.reset_n_flag();
        self.set_h_flag();
        self.pc += 2;
    }

    fn decode(&mut self, byte: u8, mmu: &MMU) -> Instruction {
        if self.debug {
            println!("Decoding PC: {:#X}", self.pc);
        }
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
            0x3E => Instruction::LdA(n1 as u8),
            0x06 => Instruction::LdB(n1 as u8),
            0x0E => Instruction::LdC(n1 as u8),
            0x16 => Instruction::LdD(n1 as u8),
            0x1E => Instruction::LdE(n1 as u8),
            0x26 => Instruction::LdH(n1 as u8),
            0x2E => Instruction::LdL(n1 as u8),
            0x7F => Instruction::LdAa,
            0x47 => Instruction::LdBa,
            0x57 => Instruction::LdDa,
            0x67 => Instruction::LdHa,
            0x77 => Instruction::LdHlA,
            0x4F => Instruction::LdCa,
            0x5F => Instruction::LdEa,
            0x6F => Instruction::LdLa,
            0x1A => Instruction::LdADE,
            0x0A => Instruction::LdABC,
            0x11 => Instruction::LdDe(d16),
            0x21 => Instruction::LdHl(d16),
            0x31 => Instruction::LdSp(d16),
            0xE0 => Instruction::LdFf00U8a(n1 as u8),
            0xF0 => Instruction::LdAFf00U8(n1 as u8),
            0xE2 => Instruction::LdFf00Ca,
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
            0x3C => Instruction::IncA,
            0x04 => Instruction::IncB,
            0x0C => Instruction::IncC,
            0x14 => Instruction::IncD,
            0x1C => Instruction::IncE,
            0x24 => Instruction::IncH,
            0x2C => Instruction::IncL,
            0xC4 => Instruction::CallNz(d16),
            0xD4 => Instruction::CallNc(d16),
            0xCC => Instruction::CallZ(d16),
            0xDC => Instruction::CallC(d16),
            0xCD => Instruction::Call(d16),
            0xF5 => Instruction::PushAf,
            0xC5 => Instruction::PushBc,
            0xD5 => Instruction::PushDe,
            0xE5 => Instruction::PushHl,
            0xF1 => Instruction::PopAf,
            0xC1 => Instruction::PopBc,
            0xD1 => Instruction::PopDe,
            0xE1 => Instruction::PopHl,
            0xCB => match cb_opcode {
                0x40 => Instruction::BitbB(0b0000_0001),
                0x41 => Instruction::BitbC(0b0000_0001),
                0x42 => Instruction::BitbD(0b0000_0001),
                0x43 => Instruction::BitbE(0b0000_0001),
                0x44 => Instruction::BitbH(0b0000_0001),
                0x45 => Instruction::BitbL(0b0000_0001),
                0x46 => Instruction::BitbHL(0b0000_0001),
                0x47 => Instruction::BitbA(0b0000_0001),
                0x48 => Instruction::BitbB(0b0000_0010),
                0x49 => Instruction::BitbC(0b0000_0010),
                0x4A => Instruction::BitbD(0b0000_0010),
                0x4B => Instruction::BitbE(0b0000_0010),
                0x4C => Instruction::BitbH(0b0000_0010),
                0x4D => Instruction::BitbL(0b0000_0010),
                0x4E => Instruction::BitbHL(0b0000_0010),
                0x4F => Instruction::BitbA(0b0000_0010),
                0x50 => Instruction::BitbB(0b0000_0100),
                0x51 => Instruction::BitbC(0b0000_0100),
                0x52 => Instruction::BitbD(0b0000_0100),
                0x53 => Instruction::BitbE(0b0000_0100),
                0x54 => Instruction::BitbH(0b0000_0100),
                0x55 => Instruction::BitbL(0b0000_0100),
                0x56 => Instruction::BitbHL(0b0000_0100),
                0x57 => Instruction::BitbA(0b0000_0100),
                0x58 => Instruction::BitbB(0b0000_1000),
                0x59 => Instruction::BitbC(0b0000_1000),
                0x5A => Instruction::BitbD(0b0000_1000),
                0x5B => Instruction::BitbE(0b0000_1000),
                0x5C => Instruction::BitbH(0b0000_1000),
                0x5D => Instruction::BitbL(0b0000_1000),
                0x5E => Instruction::BitbHL(0b0000_1000),
                0x5F => Instruction::BitbA(0b0000_1000),
                0x7C => Instruction::BitbH(0b1000_0000),
                0x17 => Instruction::RlA,
                0x10 => Instruction::RlB,
                0x11 => Instruction::RlC,
                0x12 => Instruction::RlD,
                0x13 => Instruction::RlE,
                0x14 => Instruction::RlH,
                0x15 => Instruction::RlL,
                0x16 => Instruction::RlHl,
                _ => panic!(
                    "DECODING CB PREFIX: Unreconized MMU STATE: {:?}\ncb_opcode {:#X} on pc {:#X}\n CPU STATE: {:?}",
                    mmu, cb_opcode, self.pc as u16, self
                ),
            },
            0xEE => Instruction::Xor(n1 as u8),
            _ => panic!(
                "\nMMU STATE: {:?} \nCPU STATE: {:?}\nDECODING: Unreconized byte {:#X} on pc {:#X}",
                mmu, self, byte, self.pc
            ),
        }
    }
    fn execute(&mut self, instruction: &Instruction, mmu: &mut MMU) {
        if self.debug {
            println!("Executing PC: {:#X}", self.pc);
        }
        match instruction {
            Instruction::LdA(n) => {
                if self.debug {
                    println!("LD A, n: {:#X}", n);
                }
                self.a = *n;
                self.pc += 2;
                self.t += 8;
                self.m += 2;
            },
            Instruction::LdB(n) => {
                if self.debug {
                    println!("LD B, n: {:#X}", n);
                }
                self.b = *n;
                self.pc += 2;
                self.t += 8;
                self.m += 2;
            },
            Instruction::LdC(n) => {
                if self.debug {
                    println!("LD C, n: {:#X}", n);
                }
                self.c = *n;
                self.pc += 2;
                self.t += 8;
                self.m += 2;
            },
            Instruction::LdD(n) => {
                if self.debug {
                    println!("LD D, n: {:#X}", n);
                }
                self.d = *n;
                self.pc += 2;
                self.t += 8;
                self.m += 2;
            },
            Instruction::LdE(n) => {
                if self.debug {
                    println!("LD E, n: {:#X}", n);
                }
                self.e = *n;
                self.pc += 2;
                self.t += 8;
                self.m += 2;
            },
            Instruction::LdH(n) => {
                if self.debug {
                    println!("LD H, n: {:#X}", n);
                }
                self.h = *n;
                self.pc += 2;
                self.t += 8;
                self.m += 2;
            },
            Instruction::LdL(n) => {
                if self.debug {
                    println!("LD L, n: {:#X}", n);
                }
                self.l = *n;
                self.pc += 2;
                self.t += 8;
                self.m += 2;
            },
            Instruction::LdSp(d16) => {
                if self.debug {
                    println!("LD SP, d16: {:#X}", d16);
                }
                self.sp = *d16;
                self.pc += 3;
                self.t += 12;
                self.m += 3;
            },
            Instruction::LdBc(d16) => {
                if self.debug {
                    println!("LD BC, d16: {:#X}", d16);
                }
                self.b = ((d16 & 0xFF00) >> 8) as u8;
                self.c = (d16 & 0x00FF) as u8;
                self.pc += 3;
                self.t += 12;
                self.m += 3;
            },
            Instruction::LdDe(d16) => {
                if self.debug {
                    println!("LD DE, d16: {:#X}", d16);
                }
                self.d = ((d16 & 0xFF00) >> 8) as u8;
                self.e = (d16 & 0x00FF) as u8;
                self.pc += 3;
                self.t += 12;
                self.m += 3;
            },
            Instruction::LdHl(d16) => {
                if self.debug {
                    println!(
                        "LD HL before, d16: {:#X} H: {:#X}, L: {:#X}",
                        d16, self.h, self.l
                    );
                }
                self.h = ((d16 & 0xFF00) >> 8) as u8;
                self.l = (d16 & 0x00FF) as u8;
                if self.debug {
                    println!("LD HL after, H: {:#X}, L: {:#X}", self.h, self.l);
                }
                self.pc += 3;
                self.t += 12;
                self.m += 3;
            },
            Instruction::LdAa => {
                if self.debug { println!("LD A,A"); }
                self.a = self.a;
                self.pc += 1;
                self.t += 4;
                self.m += 1;
            },
            Instruction::LdBa => {
                if self.debug { println!("LD B,A"); }
                self.a = self.b;
                self.pc += 1;
                self.t += 4;
                self.m += 1;
            },
            Instruction::LdCa => {
                if self.debug { println!("LD C,A"); }
                self.a = self.c;
                self.pc += 1;
                self.t += 4;
                self.m += 1;
            }
            Instruction::LdDa => {
                if self.debug { println!("LD D,A"); }
                self.a = self.d;
                self.pc += 1;
                self.t += 4;
                self.m += 1;
            },
            Instruction::LdEa => {
                if self.debug { println!("LD E,A"); }
                self.a = self.e;
                self.pc += 1;
                self.t += 4;
                self.m += 1;
            },
            Instruction::LdHa => {
                if self.debug { println!("LD H,A"); }
                self.a = self.h;
                self.pc += 1;
                self.t += 4;
                self.m += 1;
            },
            Instruction::LdLa => {
                if self.debug { println!("LD L,A"); }
                self.a = self.l;
                self.pc += 1;
                self.t += 4;
                self.m += 1;
            },
            Instruction::LdADE => {
                if self.debug {
                    println!(
                        "LD,A,(DE) before, A: {:#X} D: {:#X}, E: {:#X}",
                        self.a, self.d, self.e
                    );
                }
                let d16 = (self.d as u16) << 8;
                let de: u16 = d16 | (self.e as u16);
                self.a = mmu.read_byte(d16);
                self.pc += 1;
                self.t += 8;
                self.m += 2;
            },
            Instruction::LdHlA => {
                if self.debug {
                    println!(
                        "LD (HL) A before, A: {:#X} H: {:#X}, L: {:#X}",
                        self.a, self.h, self.l
                    );
                }
                let h16 = (self.h as u16) << 8;
                let hl: u16 = h16 | (self.l as u16);
                mmu.write_byte(hl, self.a);
                self.pc += 1;
                self.t += 8;
                self.m += 2;
            },
            Instruction::LddHlA => {
                if self.debug {
                    println!(
                        "LD (HL-) A before, A: {:#X} H: {:#X}, L: {:#X}",
                        self.a, self.h, self.l
                    );
                }
                let h16 = (self.h as u16) << 8;
                let mut hl: u16 = h16 | (self.l as u16);
                mmu.write_byte(hl, self.a);
                hl = hl.wrapping_sub(1);
                self.h = ((hl & 0xFF00) >> 8) as u8;
                self.l = (hl & 0x00FF) as u8;
                self.pc += 1;
                self.t += 8;
                self.m += 2;
                if self.debug {
                    println!(
                        "LD (HL-) A after, A: {:#X} H: {:#X}, L: {:#X}",
                        self.a, self.h, self.l
                    );
                }
            },
            Instruction::LdFf00U8a(n) => {
                if self.debug {
                    println!("LD (FF00+u8) A n(u8): {:#X}", n);
                }
                let addr: u16 = 0xFF00 + *n as u16;
                mmu.write_byte(addr, self.a);
                self.pc += 2;
                self.t += 12;
                self.m += 3;
            },
            Instruction::LdFf00Ca => {
                if self.debug {
                    println!("LD (C) A");
                }
                let addr: u16 = 0xFF00 + self.c as u16;
                mmu.write_byte(addr, self.a);
                self.pc += 1;
                self.t += 8;
                self.m += 2;
            },
            Instruction::XorA => {
                if self.debug {
                    println!("XorA, before A: {:#X}, Z: {:?}", self.a, self.get_z_flag());
                }
                self.a ^= self.a;
                if self.a == 0 {
                    self.set_z_flag();
                }
                if self.debug {
                    println!("XorA, after A: {:#X}, Z: {:?}", self.a, self.get_z_flag());
                }
                self.pc += 1;
                self.t += 4;
                self.m += 1;
            },
            Instruction::BitbA(bit_mask) => {
                if self.debug {
                    println!("Bit b,A b: {:b}", bit_mask);
                }
                let bit_test: u8 = self.a & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
                self.t += 8;
                self.m += 2;
            },
            Instruction::BitbB(bit_mask) => {
                if self.debug {
                    println!("Bit b,B b: {:b}", bit_mask);
                }
                let bit_test: u8 = self.b & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
                self.t += 8;
                self.m += 2;
            },
            Instruction::BitbC(bit_mask) => {
                if self.debug {
                    println!("Bit b,C b: {:b}", bit_mask);
                }
                let bit_test: u8 = self.c & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
                self.t += 8;
                self.m += 2;
            },
            Instruction::BitbD(bit_mask) => {
                if self.debug {
                    println!("Bit b,D b: {:b}", bit_mask);
                }
                let bit_test: u8 = self.d & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
                self.t += 8;
                self.m += 2;
            },
            Instruction::BitbE(bit_mask) => {
                if self.debug {
                    println!("Bit b,E b: {:b}", bit_mask);
                }
                let bit_test: u8 = self.e & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
                self.t += 8;
                self.m += 2;
            }
            Instruction::BitbH(bit_mask) => {
                if self.debug {
                    println!(
                        "BIT n,H - before, b: {:b}, H: {:#X}, Z: {:?}, N: {:?}, H(bit): {:?}, C: {:?}",
                        bit_mask,
                        self.h,
                        self.get_z_flag(),
                        self.get_n_flag(),
                        self.get_h_flag(),
                        self.get_c_flag()
                    );
                }
                let bit_test: u8 = self.h & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
                if self.debug {
                    println!(
                        "BIT n,H - after, b: {:b}, H: {:#X}, Z: {:?}, N: {:?}, H(bit): {:?}, C: {:?}",
                        bit_mask,
                        self.h,
                        self.get_z_flag(),
                        self.get_n_flag(),
                        self.get_h_flag(),
                        self.get_c_flag()
                    );
                }
                self.t += 8;
                self.m += 2;
            }
            Instruction::BitbL(bit_mask) => {
                if self.debug {
                    println!("Bit b,L b: {:b}", bit_mask);
                }
                let bit_test: u8 = self.l & *bit_mask;
                self.do_bit_opcode(*bit_mask != bit_test);
                self.t += 8;
                self.m += 2;
            }
            Instruction::JrNz(n) => {
                if self.debug {
                    println!("JR NZ n: {:#X}", n);
                }
                self.pc = self.pc + 2;
                if self.debug {
                    println!(
                        "JR NZ, n - before, n: {:#X}, Z: {:?}, PC: {:#X}",
                        n,
                        self.get_z_flag(),
                        self.pc
                    );
                }
                if !self.get_z_flag() {
                    self.pc = self.pc.wrapping_add(*n as u16);
                    self.t += 12;
                    self.m += 3;
                } else {
                    self.t += 8;
                    self.m += 2;
                }
                if self.debug {
                    println!(
                        "JR NZ, n - before, n: {:#X}, Z: {:?}, PC: {:#X}",
                        n,
                        self.get_z_flag(),
                        self.pc
                    );
                }
            }
            Instruction::JrZ(n) => {
                if self.debug {
                    println!("JR Z n: {:#X}", n);
                }
                self.pc = self.pc + 2;
                if self.get_z_flag() {
                    self.pc = self.pc.wrapping_add(*n as u16);
                    self.t += 12;
                    self.m += 3;
                } else {
                    self.t += 8;
                    self.m += 2;
                }
            }
            Instruction::JrNc(n) => {
                if self.debug {
                    println!("JR NC n: {:#X}", n);
                }
                self.pc = self.pc + 2;
                if !self.get_c_flag() {
                    self.pc = self.pc.wrapping_add(*n as u16);
                    self.t += 12;
                    self.m += 3;
                } else {
                    self.t += 8;
                    self.m += 2;
                }
            }
            Instruction::JrC(n) => {
                if self.debug {
                    println!("JR C n: {:#X}", n);
                }
                self.pc = self.pc + 2;
                if self.get_c_flag() {
                    self.pc = self.pc.wrapping_add(*n as u16);
                    self.t += 12;
                    self.m += 3;
                } else {
                    self.t += 8;
                    self.m += 2;
                }
            },
            Instruction::IncA => {
                if self.debug {
                    println!("INC A");
                }
                let was_less_than_15 = self.a < 15;
                // 0b0000_XXXX
                self.a = self.a.wrapping_add(1);
                let is_bigger_than_15 = self.a > 15;
                // 0b000X_XXXX
                self.pc += 1;
                self.t += 4;
                self.t += 1;
                // set the flags
                if self.a == 0 {
                    self.set_z_flag();
                }
                if was_less_than_15 && is_bigger_than_15 {
                    self.set_h_flag()
                }
                self.reset_n_flag();
            },
            Instruction::IncB => {
                if self.debug { println!("INC B"); }
                let was_less_than_15 = self.b < 15;
                // 0b0000_XXXX
                self.b = self.b.wrapping_add(1);
                let is_bigger_than_15 = self.b > 15;
                // 0b000X_XXXX
                self.pc += 1;
                self.t += 4;
                self.t += 1;
                // set the flags
                if self.b == 0 {
                    self.set_z_flag();
                }
                if was_less_than_15 && is_bigger_than_15 {
                    self.set_h_flag()
                }
                self.reset_n_flag();
            },
            Instruction::IncC => {
                if self.debug {
                    println!("INC C");
                }
                let was_less_than_15 = self.c < 15;
                // 0b0000_XXXX
                self.c = self.c.wrapping_add(1);
                let is_bigger_than_15 = self.c > 15;
                // 0b000X_XXXX
                self.pc += 1;
                self.t += 4;
                self.t += 1;
                // set the flags
                if self.c == 0 {
                    self.set_z_flag();
                }
                if was_less_than_15 && is_bigger_than_15 {
                    self.set_h_flag()
                }
                self.reset_n_flag();
            },
            Instruction::IncD => {
                if self.debug {
                    println!("INC D");
                }
                let was_less_than_15 = self.d < 15;
                // 0b0000_XXXX
                self.d = self.d.wrapping_add(1);
                let is_bigger_than_15 = self.d > 15;
                // 0b000X_XXXX
                self.pc += 1;
                self.t += 4;
                self.t += 1;
                // set the flags
                if self.d == 0 {
                    self.set_z_flag();
                }
                if was_less_than_15 && is_bigger_than_15 {
                    self.set_h_flag()
                }
                self.reset_n_flag();
            },
            Instruction::IncE => {
                if self.debug {
                    println!("INC E");
                }
                let was_less_than_15 = self.e < 15;
                // 0b0000_XXXX
                self.e = self.e.wrapping_add(1);
                let is_bigger_than_15 = self.e > 15;
                // 0b000X_XXXX
                self.pc += 1;
                self.t += 4;
                self.t += 1;
                // set the flags
                if self.e == 0 {
                    self.set_z_flag();
                }
                if was_less_than_15 && is_bigger_than_15 {
                    self.set_h_flag()
                }
                self.reset_n_flag();
            },
            Instruction::IncH => {
                if self.debug {
                    println!("INC H");
                }
                let was_less_than_15 = self.h < 15;
                // 0b0000_XXXX
                self.h = self.h.wrapping_add(1);
                let is_bigger_than_15 = self.h > 15;
                // 0b000X_XXXX
                self.pc += 1;
                self.t += 4;
                self.t += 1;
                // set the flags
                if self.h == 0 {
                    self.set_z_flag();
                }
                if was_less_than_15 && is_bigger_than_15 {
                    self.set_h_flag()
                }
                self.reset_n_flag();
            },
            Instruction::IncL => {
                if self.debug {
                    println!("INC L");
                }
                let was_less_than_15 = self.l < 15;
                // 0b0000_XXXX
                self.l = self.l.wrapping_add(1);
                let is_bigger_than_15 = self.l > 15;
                // 0b000X_XXXX
                self.pc += 1;
                self.t += 4;
                self.t += 1;
                // set the flags
                if self.l == 0 {
                    self.set_z_flag();
                }
                if was_less_than_15 && is_bigger_than_15 {
                    self.set_h_flag()
                }
                self.reset_n_flag();
            },
            Instruction::Call(d16) => {
                if self.debug { println!("Call d16: {:#X}", d16); }
                self.pc += 3;
                self.push_to_stack(mmu, self.pc);
                self.pc = *d16;
                self.t += 24;
                self.t += 6;
            },
            Instruction::PushAf => {
                if self.debug { println!("Push AF"); }
                let a16 = (self.a as u16) << 8;
                let af: u16 = a16 | (self.f as u16);
                self.push_to_stack(mmu, af);
                self.pc += 1;
                self.t += 16;
                self.t += 4;
            },
            Instruction::PushBc => {
                if self.debug { println!("Push BC"); }
                let b16 = (self.b as u16) << 8;
                let bc: u16 = b16 | (self.c as u16);
                self.push_to_stack(mmu, bc);
                self.pc += 1;
                self.t += 16;
                self.t += 4;
            },
            Instruction::PushDe => {
                if self.debug { println!("Push DE"); }
                let d16 = (self.d as u16) << 8;
                let de: u16 = d16 | (self.e as u16);
                self.push_to_stack(mmu, de);
                self.pc += 1;
                self.t += 16;
                self.t += 4;
            },
            Instruction::PushHl => {
                if self.debug { println!("Push HL"); }
                let h16 = (self.h as u16) << 8;
                let hl: u16 = h16 | (self.l as u16);
                self.push_to_stack(mmu, hl);
                self.pc += 1;
                self.t += 16;
                self.t += 4;
            },
            Instruction::PopAf => {
                if self.debug { println!("Pop AF"); }
                let addr: u16 = self.pop_from_stack(mmu);
                self.a = ((addr & 0xFF00) >> 8) as u8;
                self.f = (addr & 0x00FF) as u8;
                self.pc += 1;
                self.t += 12;
                self.t += 3;
            },
            Instruction::PopDe => {
                if self.debug { println!("Pop DE"); }
                let addr: u16 = self.pop_from_stack(mmu);
                self.d = ((addr & 0xFF00) >> 8) as u8;
                self.e = (addr & 0x00FF) as u8;
                self.pc += 1;
                self.t += 12;
                self.t += 3;
            },
            Instruction::PopHl => {
                if self.debug { println!("Pop HL"); }
                let addr: u16 = self.pop_from_stack(mmu);
                self.h = ((addr & 0xFF00) >> 8) as u8;
                self.l = (addr & 0x00FF) as u8;
                self.pc += 1;
                self.t += 12;
                self.t += 3;
            },
            Instruction::PopBc => {
                if self.debug { println!("Pop BC"); }
                let addr: u16 = self.pop_from_stack(mmu);
                self.b = ((addr & 0xFF00) >> 8) as u8;
                self.c = (addr & 0x00FF) as u8;
                self.pc += 1;
                self.t += 12;
                self.t += 3;
            },
//            Instruction::RlA => {
//                if self.debug { println!("RL A"); }
//
//                //
//
//
//                self.pc += 2;
//                self.t += 8;
//                self.t += 2;
//            },
            _ => panic!(
                "\n MEM STATE: {:?} \nCPU STATE: {:?}\nEXECUTING: Unreconized instruction {:?} on pc {:#X}",
                mmu, self, instruction, self.pc
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
