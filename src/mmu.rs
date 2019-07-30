use std::fmt;

pub struct MMU {
    ram: [u8; 65_536], //0X0000 to 0xFFFF
}
impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "$FF40 - LCDC: {:b}, \
             $FF41 - STAT: {:b}, \
             $FF42 - SCY: {:#X}, \
             $FF43 - SCX: {:#X}, \
             $FF44 - LY: {:b}, \
             $FF45 - LYC: {:b}, \
             $FF47 - BGP: {:b}, \
             $90: {:b}",
            //             VRAM Tile Data - Block 1: {:?}\n\
            //             VRAM Tile Data - Block 2: {:?}\n\
            //             VRAM Tile Data - Block 3: {:?}\n",
            &self.ram[0xFF40],
            &self.ram[0xFF41],
            &self.ram[0xFF42],
            &self.ram[0xFF43],
            &self.ram[0xFF44],
            &self.ram[0xFF45],
            &self.ram[0xFF47],
            &self.ram[0x0090],
            //            &self.ram[0x8000..0x8800],
            //            &self.ram[0x8800..0x9000],
            //            &self.ram[0x9000..0x9800],
            //            "VRAM: {:?}\n\nOAM RAM: {:?}\n\nIO RAM: {:?}\n\nH RAM: {:?}\n\n",
            //            &self.ram[0x8000..0xA001],
            //            &self.ram[0xFE00..0xFEA1],
            //            &self.ram[0xFF00..0xFF81],
            //            &self.ram[0xFE80..],
        )
    }
}

impl MMU {
    pub fn new() -> MMU {
        let mut mem = MMU { ram: [0; 65_536] };
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
