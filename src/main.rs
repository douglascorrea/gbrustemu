use std::fs::File;
use std::io::Read;

fn main() {
//    Read the boot-rom file
    let mut f = File::open("ROMS/DMG_ROM.bin").unwrap();
    let mut rom_file = Vec::<u8>::new();

    f.read_to_end(&mut rom_file);
    for &byte in rom_file.iter() {
        println!("{:#X}", (byte as u16));
    }

}
