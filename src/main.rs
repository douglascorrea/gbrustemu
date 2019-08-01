use gbrustemu::cpu::CPU;
use gbrustemu::mmu::MMU;
use gbrustemu::ppu::PPU;
use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::Read;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    //    Read the rom file
    let mut f = File::open("ROMS/DMG_ROM.bin").unwrap();
    let mut rom_file = Vec::<u8>::new();
    f.read_to_end(&mut rom_file).unwrap();

    // put the rom file into the memory ram
    let mut mmu = MMU::new();
    mmu.from_rom_file(&rom_file);

    // run make CPU run instructions over ram
    //    println!("MMU BEFORE: {:?}", mmu);
    let mut cpu = CPU::new();
    let mut ppu = PPU::new();
    //    cpu.set_debug_flag();

    let mut screen = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.run_instruction(&mut mmu, &mut ppu);
        //        if ppu.is_lcd_enable() {
        //            for i in screen.iter_mut() {
        //                *i = 0; // write something more funny here!
        //            }
        //
        //            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        //            window.update_with_buffer(&screen).unwrap();
        //        }
    }
    //    loop {
    //        cpu.run_instruction(&mut mmu, &mut ppu);
    //    }
}
