use std::fs::File;
use std::io::Read;

//pub struct Gameboy {}

//pub enum InstructionSet {
//    LD,
//}

pub struct Memory {
    ram: [u8; 65536], //0X0000 to 0xFFFF
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory { ram: [0; 65536] };
        mem
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn from_rom_file(&mut self, rom_file: &[u8]) {
        let mut i: u16 = 0x0000;
        for &byte in rom_file.iter() {
            //            println!("{:#X}", i);
            self.write_byte(i, byte);
            i += 1
        }
    }
}

#[derive(Debug)]
pub struct CPU {
    a: u8,
    b: u8,
    d: u8,
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
            d: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        };
        cpu
    }

    pub fn run_instruction(&mut self, mem: &mut Memory) {
        let instruction = mem.read_byte(self.pc);
        match instruction {
            // decode instructions

            // 1. LD n,nn
            // Put value nn into n.
            // Use with:
            // n = BC,DE,HL,SP
            // nn = 16 bit immediate value
            ld @ 0x01 | ld @ 0x11 | ld @ 0x21 | ld @ 0x31 => {
                // LD SP, d16 // put d16 into SP (Stack pointer)
                match ld {
                    0x31 => {
                        // the immediate 16 bit
                        let n1 = mem.read_byte(self.pc + 1) as u16;
                        let n2 = mem.read_byte(self.pc + 2) as u16;
                        // inverting position because it is BIG ENDIAN with bitwise operation
                        let d16: u16 = (n2 << 8) | n1;
                        self.sp = d16;
                        //                        println!("This is LD 0x31 with imediate value = {:#X}", d16);
                        self.pc += 3;
                    }
                    _ => panic!(
                        "Unreconized instruction {:#X} on pc {:#X}\n CPU STATE: {:?}",
                        instruction, self.pc, self
                    ),
                }
            }
            //         7. XOR n
            //            Description:
            //            Logical exclusive OR n with register A, result in A.
            //            Use with:
            //                n = A,B,C,D,E,H,L,(HL),#
            //            Flags affected:
            //            Z - Set if result is zero.
            //            N - Reset.
            //            H - Reset.
            //            C - Reset.
            //            @TODO AF
            _ => panic!(
                "Unreconized instruction {:#X} on pc {:#X}\n CPU STATE: {:?}",
                instruction, self.pc, self
            ),
        }
    }
}

fn main() {
    //    Read the rom file
    let mut f = File::open("ROMS/DMG_ROM.bin").unwrap();
    let mut rom_file = Vec::<u8>::new();
    f.read_to_end(&mut rom_file);

    // put the rom file into the memory ram
    let mut mem = Memory::new();
    mem.from_rom_file(&rom_file);

    // run make CPU run instructions over ram
    let mut cpu = CPU::new();
    loop {
        cpu.run_instruction(&mut mem);
    }

    //    for &byte in rom_file.iter() {
    //        //        println!("{:#X}", (byte as u16));
    //        mem.write_byte(0x0000, byte);
    //    }
}
