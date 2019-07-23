use gbrustemu::cpu::CPU;
use gbrustemu::mmu::MMU;
use std::fs::File;
use std::io::Read;

fn main() {
    //    Read the rom file
    let mut f = File::open("ROMS/DMG_ROM.bin").unwrap();
    let mut rom_file = Vec::<u8>::new();
    f.read_to_end(&mut rom_file);

    // put the rom file into the memory ram
    let mut mmu = MMU::new();
    mmu.from_rom_file(&rom_file);

    // run make CPU run instructions over ram
    //    println!("MMU BEFORE: {:?}", mmu);
    let mut cpu = CPU::new();
    //    cpu.set_debug_flag();
    loop {
        cpu.run_instruction(&mut mmu);
    }
}
