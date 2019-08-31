use crate::instruction::Instruction;
use crate::mmu::MMU;
use crate::ppu::PPU;

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
    ime: bool,
    last_t: usize,
    last_m: usize,
    debug: bool,
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CPU {{ A: {:#X}, B: {:#X}, C: {:#X}, D: {:#X}, E: {:#X}, H: {:#X}, L: {:#X} }} \nflags: {{ Z: {:?}, N: {:?}, H: {:?}, C: {:?} }}\n{{ pc: {:#X}, sp: {:#X} }}, ime: {:?}",
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
            self.sp,
            self.ime
        )
    }
}

impl CPU {
    pub fn new() -> CPU {
        let cpu = CPU {
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
            ime: false,
            last_t: 0,
            last_m: 0,
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
GVHBJ V FTCRDVH B  = mmu.read_byte(self.sp);
        let addr_016 = (addr_0 as u16) << 8;
        let addr: u16 = addr_016 | (addr_1 as u16);
        addr
    }

    fn do_bit_opcode(&mut self, reg_value: u8, bit_mask: u8) {
        let bit_test: u8 = reg_value & bit_mask;
        let bit = bit_test != bit_mask;
        if bit {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }
        self.reset_n_flag();
        self.set_h_flag();
        self.inc_pc_t(2, 8);
    }
    fn calc_half_carry_on_u16_sum(&self, value_a: u16, value_b: u16) -> bool {
        ((value_a & 0xFFF) + (value_b & 0xFFF)) & 0x1000 == 0x1000
    }

    fn calc_half_carry_on_u16_sub(&self, value_a: u16, value_b: u16) -> bool {
        (value_a & 0xFFF) < (value_b & 0xFFF)
    }

    fn calc_half_carry_on_u8_sum(&self, value_a: u8, value_b: u8) -> bool {
        ((value_a & 0xF) + (value_b & 0xF)) & 0x10 == 0x10
    }

    fn calc_half_carry_on_u8_sub(&self, value_a: u8, value_b: u8) -> bool {
        (value_a & 0xF) < (value_b & 0xF)
    }

    fn do_inc_d16(&mut self, register_value: u16) -> u16 {
        // Checking the Half Carry bit
        if self.calc_half_carry_on_u16_sum(register_value, 1) {
            self.set_h_flag();
        } else {
            self.reset_h_flag();
        }

        let new_register_value = register_value.wrapping_add(1);

        self.pc += 1;
        self.t += 4;
        self.m += 1;
        // set the flags
        if new_register_value == 0 {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }
        self.reset_n_flag();
        new_register_value
    }
    fn do_add(&mut self, register_value_a: u8, register_value_b: u8) -> u8 {
        // Checking the Half Carry bit
        if self.calc_half_carry_on_u8_sum(register_value_a, register_value_b) {
            self.set_h_flag();
        } else {
            self.reset_h_flag();
        }

        let new_register_value_a = register_value_a.wrapping_add(register_value_b);

        self.pc += 1;
        self.t += 4;
        self.m += 1;
        // set the flags
        if new_register_value_a == 0 {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }
        self.reset_n_flag();
        new_register_value_a
    }
    fn do_inc_n(&mut self, register_value: u8) -> u8 {
        self.do_add(register_value, 1)
    }
    fn do_sub(&mut self, register_value_a: u8, register_value_b: u8) -> u8 {
        // Checking the Half Carry bit
        if self.calc_half_carry_on_u8_sub(register_value_a, register_value_b) {
            self.reset_h_flag();
        } else {
            self.set_h_flag();
        }

        let new_register_value_a = register_value_a.wrapping_sub(register_value_b);

        self.pc += 1;
        self.t += 4;
        self.m += 1;
        // set the flags
        if new_register_value_a == 0 {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }
        self.reset_n_flag();
        new_register_value_a
    }
    fn do_dec_n(&mut self, register_value: u8) -> u8 {
        self.do_sub(register_value, 1)
    }

    fn do_rl_n(&mut self, register_value: u8) -> u8 {
        let old_c_flag = self.get_c_flag();
        let c_flag: bool = (0b1000_0000 & register_value) != 0;
        if c_flag {
            self.set_c_flag();
        } else {
            self.reset_c_flag();
        }
        // actually rotating
        let mut new_register_value = register_value << 1;
        new_register_value = new_register_value & 0b1111_1110;
        if old_c_flag {
            new_register_value += 1;
        }
        //handling flags
        if new_register_value == 0 {
            self.set_z_flag();
        } else {
            self.reset_z_flag();
        }
        self.reset_n_flag();
        self.reset_h_flag();

        self.pc += 2;
        self.t += 8;
        self.t += 2;
        new_register_value
    }

    fn do_jump(&mut self, jump: bool, n1: i8) {
        self.pc = self.pc + 2;
        if jump {
            self.pc = self.pc.wrapping_add(n1 as u16);
            self.t += 12;
        } else {
            self.t += 8;
        }

    }

    fn h_l_to_hl(&self) -> u16 {
        let h16 = (self.h as u16) << 8;
        h16 | (self.l as u16)
    }

    fn hl_to_h_l(&mut self, hl: u16) {
        self.h = ((hl & 0xFF00) >> 8) as u8;
        self.l = (hl & 0x00FF) as u8;
    }

    fn b_c_to_bc(&self) -> u16 {
        let b16 = (self.b as u16) << 8;
        b16 | (self.c as u16)
    }

    fn bc_to_b_c(&mut self, bc: u16) {
        self.b = ((bc & 0xFF00) >> 8) as u8;
        self.c = (bc & 0x00FF) as u8;
    }

    fn d_e_to_de(&self) -> u16 {
        let d16 = (self.d as u16) << 8;
        d16 | (self.e as u16)
    }

    fn de_to_d_e(&mut self, de: u16) {
        self.d = ((de & 0xFF00) >> 8) as u8;
        self.e = (de & 0x00FF) as u8;
    }

    fn a_f_to_af(&self) -> u16 {
        let a16 = (self.a as u16) << 8;
        a16 | (self.a as u16)
    }

    fn af_to_a_f(&mut self, af: u16) {
        self.a = ((af & 0xFF00) >> 8) as u8;
        self.f = (af & 0x00FF) as u8;
    }

    fn decode(&mut self, byte: u8, mmu: &MMU) -> Instruction {

        match byte {
            0x2A => Instruction::LdiAHl,
            0xAF => Instruction::XorA,
            0xA8 => Instruction::XorB,
            0xA9 => Instruction::XorC,
            0xAA => Instruction::XorD,
            0xAB => Instruction::XorE,
            0xAC => Instruction::XorH,
            0xAD => Instruction::XorL,
            0xAE => Instruction::XorHl,
            0xD6 => Instruction::Sub(n1 as u8),
            0xC6 => Instruction::AddA(n1 as u8),
            0xC4 => Instruction::CallNz(d16),
            0xD4 => Instruction::CallNc(d16),
            0xCC => Instruction::CallZ(d16),
            0xDC => Instruction::CallC(d16),
            0xCD => Instruction::Call(d16),
            0xD5 => Instruction::PushDe,
            0xE5 => Instruction::PushHl,
            0xF1 => Instruction::PopAf,
            0xC1 => Instruction::PopBc,
            0xD1 => Instruction::PopDe,
            0xE1 => Instruction::PopHl,
            0x17 => Instruction::RLA,
            0xBF => Instruction::CpA,
            0xB8 => Instruction::CpB,
            0xB9 => Instruction::CpC,
            0xBA => Instruction::CpD,
            0xBB => Instruction::CpE,
            0xBC => Instruction::CpH,
            0xBD => Instruction::CpL,
            0xBE => Instruction::CpHl,
            0xFE => Instruction::Cp(n1 as u8),
            0xCB => match cb_opcode {
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

    fn set_register(&mut self, register_name: &str, register_value: u8) {
        match register_name {
            "a" => {
                self.a = register_value;
            }
            "b" => {
                self.b = register_value;
            }
            "c" => {
                self.c = register_value;
            }
            "d" => {
                self.d = register_value;
            }
            "e" => {
                self.e = register_value;
            }
            "f" => {
                self.f = register_value;
            }
            "h" => {
                self.h = register_value;
            }
            "l" => {
                self.l = register_value;
            }
            _ => {
                panic!(
                    "set_register: Unrecognized register to set {:?}",
                    &register_name
                );
            }
        }
    }
    fn get_register(&self, register_name: &str) -> u8 {
        match register_name {
            "a" => self.a,
            "b" => self.b,
            "c" => self.c,
            "d" => self.d,
            "e" => self.e,
            "f" => self.f,
            "h" => self.h,
            "l" => self.l,
            _ => {
                panic!(
                    "set_register: Unrecognized register to set {:?}",
                    &register_name
                );
            }
        }
    }

    fn do_ld_reg_to_reg(&mut self, to: &str, from: &str) {
        self.set_register(to, self.get_register(from));
        self.inc_pc_t(1, 4);
    }

    fn inc_pc_t(&mut self, pc: u16, t: usize) {
        self.pc += pc;
        self.t += t;
    }

    fn execute(&mut self, byte: u8, mmu: &mut MMU) {
        let n1 = mmu.read_byte(self.pc + 1) as u16;
        let n2 = mmu.read_byte(self.pc + 2) as u16;
        let d16: u16 = (n2 << 8) | n1;

        let cb_opcode = n1;
        let cb_n1 = n2;
        let cb_n2 = mmu.read_byte(self.pc + 3) as u16;
        let cb_d16: u16 = (cb_n2 << 8) | cb_n1;
        match byte {
            0x00 => {
                self.inc_pc_t(1,4);
            },
            0x01 => {
                self.b = ((d16 & 0xFF00) >> 8) as u8;
                self.c = (d16 & 0x00FF) as u8;
                self.inc_pc_t(3,12);
            },
            0x04 => self.b = self.do_inc_n(self.b),
            0x06 => {
                self.b = n1 as u8;
                self.inc_pc_t(2,8);
            }
            0x0C => self.c = self.do_inc_n(self.c),
            0x0E => {
                self.c = n1 as u8;
                self.inc_pc_t(2,8);
            }
            0x11 => {
                self.d = ((d16 & 0xFF00) >> 8) as u8;
                self.e = (d16 & 0x00FF) as u8;
                self.inc_pc_t(3,12);
            },
            0x14 => self.d = self.do_inc_n(self.d),
            0x16 => {
                self.d = n1 as u8;
                self.inc_pc_t(2,8);
            },
            0x18 => self.do_jump(true,n1 as i8),
            0x1A => {
                let d16 = (self.d as u16) << 8;
                let de: u16 = d16 | (self.e as u16);
                self.a = mmu.read_byte(de);
                self.inc_pc_t(2, 12);
            },
            0x1C => self.e = self.do_inc_n(self.e),
            0x1E => {
                self.e = n1 as u8;
                self.inc_pc_t(2,8);
            }
            0x20 => self.do_jump(!self.get_z_flag(),n1 as i8),
            0x21 => {
                self.h = ((d16 & 0xFF00) >> 8) as u8;
                self.l = (d16 & 0x00FF) as u8;
                self.inc_pc_t(3,12);
            }
            0x22 => {
                let mut hl = self.h_l_to_hl();
                mmu.write_byte(hl, self.a);
                hl = hl.wrapping_add(1);
                self.hl_to_h_l(hl);
                self.inc_pc_t(1,8);
            },
            0x26 => {
                self.h = n1 as u8;
                self.inc_pc_t(2,8);
            }
            0x28 => self.do_jump(self.get_z_flag(),n1 as i8),
            0x2A => {
                let mut hl = self.h_l_to_hl();
                self.a = mmu.read_byte(hl);
                hl = hl.wrapping_add(1);
                self.hl_to_h_l(hl);
                self.inc_pc_t(1,8);
            },
            0x2E => {
                self.l = n1 as u8;
                self.inc_pc_t(2,8);
            }
            0x30 => self.do_jump(!self.get_c_flag(),n1 as i8),
            0x31 => {
                self.sp = d16;
                self.inc_pc_t(3,12);
            }
            0x32 => {
                let mut hl = self.h_l_to_hl();
                mmu.write_byte(hl, self.a);
                hl = hl.wrapping_sub(1);
                self.h = ((hl & 0xFF00) >> 8) as u8;
                self.l = (hl & 0x00FF) as u8;
                self.inc_pc_t(1,8);
            }
            0x36 => {
                let hl = self.h_l_to_hl();
                mmu.write_byte(hl, n1 as u8);
                self.inc_pc_t(2, 12);
            }
            0x38 => self.do_jump(self.get_c_flag(),n1 as i8),
            0x3C => self.a = self.do_inc_n(self.a),
            0x3E => {
                self.a = n1 as u8;
                self.inc_pc_t(2,8);
            }
            0x47 => self.do_ld_reg_to_reg("b", "a"),
            0x4F => self.do_ld_reg_to_reg("c", "a"),
            0x57 => self.do_ld_reg_to_reg("d", "a"),
            0x5F => self.do_ld_reg_to_reg("e", "a"),
            0x67 => self.do_ld_reg_to_reg("h", "a"),
            0x6f => self.do_ld_reg_to_reg("l", "a"),
            0x78 => self.do_ld_reg_to_reg("a", "b"),
            0x77 => {
                let hl = self.h_l_to_hl();
                mmu.write_byte(hl, self.a);
                self.inc_pc_t(1, 8);
            }
            0x79 => self.do_ld_reg_to_reg("a", "c"),
            0x7A => self.do_ld_reg_to_reg("a", "d"),
            0x7B => self.do_ld_reg_to_reg("a", "e"),
            0x7C => self.do_ld_reg_to_reg("a", "h"),
            0x7D => self.do_ld_reg_to_reg("a", "l"),
            0x7F => self.do_ld_reg_to_reg("a", "a"),
            0xAF => {
                self.a ^= self.a;
                if self.a == 0 {
                    self.set_z_flag();
                }
                self.inc_pc_t(1,4)
            },
            0xC3 => {
                self.pc = d16;
                self.t += 16;
            }
            0xE0 => {
                let addr: u16 = 0xFF00 + n1;
                mmu.write_byte(addr, self.a);
                self.inc_pc_t(2,12)
            }
            0xE2 => {
                let addr: u16 = 0xFF00 + self.c as u16;
                mmu.write_byte(addr, self.a);
                self.inc_pc_t(1,8)
            }
            0xEA => {
                mmu.write_byte(d16, self.a);
                self.inc_pc_t(3, 16);
            }
            0xF0 => {
                let addr: u16 = 0xFF00 + n1 as u16;
                self.a = mmu.read_byte(addr);
                self.inc_pc_t(2,12)
            }
            0xF3 => {
                self.ime = false;
                self.inc_pc_t(1,4);
            },
            0xFB => {
                self.ime = true;
                self.inc_pc_t(1,4);
            }
            0x24 => self.h = self.do_inc_n(self.h),
            0x2C => self.l = self.do_inc_n(self.l),
            0x34 => self.hl_to_h_l(self.do_inc_d16(self.h_l_to_hl())),
            0x23  => {
                let mut hl= self.h_l_to_hl();
                hl = hl.wrapping_add(1);
                self.hl_to_h_l(hl);
                self.inc_pc_t(1, 8);
            }
            0x03 => {
                let mut bc = self.b_c_to_bc();
                bc = bc.wrapping_add(1);
                self.bc_to_b_c(bc);
                self.inc_pc_t(1, 8);
            }
            0x13 => {
                let mut de = self.d_e_to_de();
                de = de.wrapping_add(1);
                self.de_to_d_e(de);
                self.inc_pc_t(1, 8);
            }
            0x3D => self.a = self.do_dec_n(self.a),
            0x05  => self.b = self.do_dec_n(self.b),
            0x0D  => self.c = self.do_dec_n(self.c),
            0x15  => self.d = self.do_dec_n(self.d),
            0x1D  => self.e = self.do_dec_n(self.e),
            0x25 => self.h = self.do_dec_n(self.h),
            0x2D => self.l = self.do_dec_n(self.l),
            0x97 => self.a = self.do_sub(self.a, self.a),
            0x90 => self.a = self.do_sub(self.a, self.b),
            0x91 => self.a = self.do_sub(self.a, self.c),
            0x92 => self.a = self.do_sub(self.a, self.d),
            0x93 => self.a = self.do_sub(self.a, self.e),
            0x94 => self.a = self.do_sub(self.a, self.h),
            0x95 => self.a = self.do_sub(self.a, self.l),
            0x87 => self.a = self.do_add(self.a, self.a),
            0xCB => match cb_opcode {
                0x40 => self.do_bit_opcode(self.b, 0b0000_0001),
                0x41 => self.do_bit_opcode(self.c, 0b0000_0001),
                0x42 => self.do_bit_opcode(self.d, 0b0000_0001),
                0x43 => self.do_bit_opcode(self.e, 0b0000_0001),
                0x44 => self.do_bit_opcode(self.h, 0b0000_0001),
                0x45 => self.do_bit_opcode(self.l, 0b0000_0001),
                0x47 => self.do_bit_opcode(self.a, 0b0000_0001),
                0x48 => self.do_bit_opcode(self.b, 0b0000_0010),
                0x49 => self.do_bit_opcode(self.c, 0b0000_0010),
                0x4A => self.do_bit_opcode(self.d, 0b0000_0010),
                0x4B => self.do_bit_opcode(self.e, 0b0000_0010),
                0x4C => self.do_bit_opcode(self.h, 0b0000_0010),
                0x4D => self.do_bit_opcode(self.l, 0b0000_0010),
                0x4F => self.do_bit_opcode(self.a, 0b0000_0010),
                0x50 => self.do_bit_opcode(self.b, 0b0000_0100),
                0x51 => self.do_bit_opcode(self.c, 0b0000_0100),
                0x52 => self.do_bit_opcode(self.d, 0b0000_0100),
                0x53 => self.do_bit_opcode(self.e, 0b0000_0100),
                0x54 => self.do_bit_opcode(self.h, 0b0000_0100),
                0x55 => self.do_bit_opcode(self.l, 0b0000_0100),
                0x57 => self.do_bit_opcode(self.a, 0b0000_0100),
                0x58 => self.do_bit_opcode(self.b, 0b0000_1000),
                0x59 => self.do_bit_opcode(self.c, 0b0000_1000),
                0x5A => self.do_bit_opcode(self.d, 0b0000_1000),
                0x5B => self.do_bit_opcode(self.e, 0b0000_1000),
                0x5C => self.do_bit_opcode(self.h, 0b0000_1000),
                0x5D => self.do_bit_opcode(self.l, 0b0000_1000),
                0x5F => self.do_bit_opcode(self.a, 0b0000_1000),
                0x7c => self.do_bit_opcode(self.h, 0b1000_0000),
                0x17 => self.a = self.do_rl_n(self.a),
                0x10 => self.b = self.do_rl_n(self.b),
                0x11 => self.c = self.do_rl_n(self.c),
                0x12 => self.d = self.do_rl_n(self.d),
                0x13 => self.e = self.do_rl_n(self.e),
                0x14 => self.h = self.do_rl_n(self.e),
                0x15 => self.l = self.do_rl_n(self.l),
                _ => panic!(
                    "DECODING CB PREFIX: Unreconized MMU STATE: {:?}\ncb_opcode {:#X} on pc {:#X}\n CPU STATE: {:?}",
                    mmu, cb_opcode, self.pc as u16, self
                ),
            },
            0x80  => self.a = self.do_add(self.a, self.b),
            0x81 => self.a = self.do_add(self.a, self.c),
            0x82 => self.a = self.do_add(self.a, self.d),
            0x83 => self.a = self.do_add(self.a, self.e),
            0x84 => self.a = self.do_add(self.a, self.h),
            0x85 => self.a = self.do_add(self.a, self.l),
            0x86 => {
                let hl = self.h_l_to_hl();
                self.a = self.do_add(self.a, mmu.read_byte(hl));
            }
            0xCD  => {
                self.pc += 3;
                self.push_to_stack(mmu, self.pc);
                self.pc = d16;
                self.t += 24;
            }
            0xC9 => {
                self.pc = self.pop_from_stack(mmu);
                self.t += 16;
            }
            0xF5 => {
                let af = self.a_f_to_af();
                self.push_to_stack(mmu, af);
                self.inc_pc_t(1, 16)
            }
            0xC5 => {
                let bc = self.b_c_to_bc();
                self.push_to_stack(mmu, bc);
                self.inc_pc_t(1, 16)
            }
            }
            Instruction::PushDe => {
                if self.debug { println!("Push DE"); }
                let d16 = (self.d as u16) << 8;
                let de: u16 = d16 | (self.e as u16);
                self.push_to_stack(mmu, de);
                self.pc += 1;
                self.t += 16;
                self.m += 4;
            }
            Instruction::PushHl => {
                if self.debug { println!("Push HL"); }
                let h16 = (self.h as u16) << 8;
                let hl: u16 = h16 | (self.l as u16);
                self.push_to_stack(mmu, hl);
                self.pc += 1;
                self.t += 16;
                self.m += 4;
            }
            Instruction::PopAf => {
                if self.debug { println!("Pop AF"); }
                let addr: u16 = self.pop_from_stack(mmu);
                self.a = ((addr & 0xFF00) >> 8) as u8;
                self.f = (addr & 0x00FF) as u8;
                self.pc += 1;
                self.t += 12;
                self.m += 3;
            }
            Instruction::PopDe => {
                if self.debug { println!("Pop DE"); }
                let addr: u16 = self.pop_from_stack(mmu);
                self.d = ((addr & 0xFF00) >> 8) as u8;
                self.e = (addr & 0x00FF) as u8;
                self.pc += 1;
                self.t += 12;
                self.m += 3;
            }
            Instruction::PopHl => {
                if self.debug { println!("Pop HL"); }
                let addr: u16 = self.pop_from_stack(mmu);
                self.h = ((addr & 0xFF00) >> 8) as u8;
                self.l = (addr & 0x00FF) as u8;
                self.pc += 1;
                self.t += 12;
                self.m += 3;
            }
            Instruction::PopBc => {
                if self.debug { println!("Pop BC"); }
                let addr: u16 = self.pop_from_stack(mmu);
                self.b = ((addr & 0xFF00) >> 8) as u8;
                self.c = (addr & 0x00FF) as u8;
                self.pc += 1;
                self.t += 12;
                self.m += 3;
            }
            Instruction::RLA => {
                if self.debug { println!("RLA"); }
                self.a = self.do_rl_n(self.a);
                self.pc -= 1;
                self.t -= 4;
                self.m -= 1;
            }
            Instruction::CpA => {
                if self.debug { println!("CP A") };
                let _ = self.do_sub(self.a, self.a);
            }
            Instruction::CpB => {
                if self.debug { println!("CP B") };
                let _ = self.do_sub(self.a, self.b);
            }
            Instruction::CpC => {
                if self.debug { println!("CP C") };
                let _ = self.do_sub(self.a, self.c);
            }
            Instruction::CpD => {
                if self.debug { println!("CP D") };
                let _ = self.do_sub(self.a, self.d);
            }
            Instruction::CpE => {
                if self.debug { println!("CP E") };
                let _ = self.do_sub(self.a, self.e);
            }
            Instruction::CpH => {
                if self.debug { println!("CP H") };
                let _ = self.do_sub(self.a, self.h);
            }
            Instruction::CpL => {
                if self.debug { println!("CP L") };
                let _ = self.do_sub(self.a, self.l);
            }
            Instruction::CpHl => {
                if self.debug { println!("CP HL") };
                let h16 = (self.h as u16) << 8;
                let hl: u16 = h16 | (self.l as u16);
                let _ = self.do_sub(self.a, mmu.read_byte(hl));
                self.t += 4;
                self.m += 1;
            }
            Instruction::Cp(n) => {
                if self.debug { println!("CP n: {:#X}", n) };
                let ly = mmu.read_byte(0xFF44);
                let _ = self.do_sub(self.a, *n);
                self.pc += 1;
                self.t += 4;
                self.m += 1;
            }
            _ => panic!(
                "\n MEM STATE: {:?} \nCPU STATE: {:?}\nEXECUTING: Unreconized instruction {:?} on pc {:#X}",
                mmu, self, instruction, self.pc
            ),
        }
    }

    pub fn run_instruction(&mut self, mmu: &mut MMU, ppu: &mut PPU) {
        self.last_m = self.m;
        self.last_t = self.t;

        // fetch
        let byte = mmu.read_byte(self.pc);
        // decode
        let instruction = self.decode(byte, mmu);
        // execute
        self.execute(byte, mmu);

        let current_instruction_t_clocks_passed = self.t - self.last_t;
        ppu.step(current_instruction_t_clocks_passed, mmu);
        //        if self.pc == 0x00E8 {
        //            let bg_tile_set = ppu.get_bg_tile_set(mmu);
        //            let mut i = 0;
        //            //            while i < bg_tile_set.len() {
        //            let tile = ppu.get_tile(mmu, 33168);
        //            //                println!("TILE ADDR: {:?}", (0x8000 + i) as u16);
        //            println!("TILE: {:?}", tile);
        //            ppu.transform_tile_to_minifb_tile(&mmu, tile);
        //            i += 16;
        //            //            }
        //            panic!("BGP Palette: {:b}", ppu.get_bgp(&mmu));
        //        }
    }
}
