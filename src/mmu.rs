use std::fmt;

pub struct MMU {
    ram: [u8; 65_536], //0X0000 to 0xFFFF
}
impl fmt::Debug for MMU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VRAM: \n\nOAM RAM: {:?}\n\nIO RAM: {:?}\n\nH RAM: {:?}\n\n",
            //            &self.ram[0x8000..0xA000],
            &self.ram[0xFE00..0xFEA0],
            &self.ram[0xFF00..0xFF80],
            &self.ram[0xFE80..0xFFFF],
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
