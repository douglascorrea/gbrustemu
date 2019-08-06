use gbrustemu::cpu::CPU;
use gbrustemu::mmu::MMU;
use gbrustemu::ppu::{LIGHTEST_GREEN, PPU, SCREEN_HEIGHT, SCREEN_WIDTH};
use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::Read;

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

    let mut screen = vec![LIGHTEST_GREEN; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.run_instruction(&mut mmu, &mut ppu);
        if ppu.is_lcd_enable(&mmu) {
            ppu.populate_background_buffer(&mmu);
            let background_buffer = ppu.get_background_buffer();
            let current_viewport = ppu.transform_background_buffer_into_screen(&mmu);
            for (m, pixel) in current_viewport.iter().enumerate() {
                screen[m] = *pixel;
            }
            window.update_with_buffer(&screen).unwrap();
        }
    }
}
