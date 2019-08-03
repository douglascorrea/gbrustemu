use gbrustemu::cpu::CPU;
use gbrustemu::mmu::MMU;
use gbrustemu::ppu::{LIGHTEST_GREEN, PPU};
use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::Read;

//const WIDTH: usize = 160;
//const HEIGHT: usize = 144;
const WIDTH: usize = 256;
const HEIGHT: usize = 256;

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

    let mut screen = vec![LIGHTEST_GREEN; WIDTH * HEIGHT];
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
        if ppu.is_lcd_enable(&mmu) {
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            //            let tile = ppu.get_tile(&mmu, 33168);
            //            //
            //            let minifb_tile = ppu.transform_tile_to_minifb_tile(&mmu, tile);
            //            println!("{:?}", minifb_tile);
            ppu.populate_background_buffer(&mmu);
            let background_buffer = ppu.get_background_buffer();
            // ********
            //

            //            for (m, minifb_tile) in background_buffer.iter().enumerate() {
            for (m, pixel) in background_buffer.iter().enumerate() {
                screen[m] = *pixel;
            }
            //                        }
            //            let lcdc = ppu.get_lcdc(&mmu);
            //            println!("LCDC: {:b}", lcdc);
            //            println!("MMU STATE {:?}", mmu);

            //9800-9BFF
            window.update_with_buffer(&screen).unwrap();
        }
    }
    //    loop {
    //        cpu.run_instruction(&mut mmu, &mut ppu);
    //    }
}
