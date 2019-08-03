use std::fmt;

pub struct MMU {
    ram: [u8; 65_536], //0X0000 to 0xFFFF
}
impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "$FF40 - LCDC: {:b}, \n\
             $FF41 - STAT: {:b}, \n\
             $FF42 - SCY: {:#X}, \n\
             $FF43 - SCX: {:#X}, \n\
             $FF44 - LY: {:?}, \n\
             $FF45 - LYC: {:?}, \n\
             $FF46 - DMA: {:#X}, \n\
             $FF47 - BGP: {:b}, \n\
             $FF48 - OBP0: {:b}, \n\
             $FF49 - OBP1: {:b}, \n\
             $FF4A - WY: {:#X}, \n\
             $FF4B - WY: {:#X}, \n
             BG Tile Data: {:?}\n\
             BG Tile Map: {:?}\n\
             ",
            &self.ram[0xFF40],
            &self.ram[0xFF41],
            &self.ram[0xFF42],
            &self.ram[0xFF43],
            &self.ram[0xFF44],
            &self.ram[0xFF45],
            &self.ram[0xFF46],
            &self.ram[0xFF47],
            &self.ram[0xFF48],
            &self.ram[0xFF49],
            &self.ram[0xFF4A],
            &self.ram[0xFF4B],
            &self.ram[0x8000..0x87FF],
            &self.ram[0x9800..0x9BFF],
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
        let mmu = MMU { ram: [0; 65_536] };
        mmu
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
