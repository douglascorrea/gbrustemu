use gbrustemu::cpu::CPU;
use gbrustemu::mmu::MMU;
use gbrustemu::ppu::{LIGHTEST_GREEN, PPU, SCREEN_HEIGHT, SCREEN_WIDTH};
use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    //    Read the rom file
    let mut f = File::open("ROMS/tetris.gb").unwrap();
    let mut rom_file = Vec::<u8>::new();
    f.read_to_end(&mut rom_file).unwrap();

    // put the rom file into the memory ram
    let mut mmu = MMU::new();
    mmu.from_rom_file(&rom_file);

    let mut cpu = CPU::new();
    let mut ppu = PPU::new();

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
            if mmu.dirty_viewport_flag || mmu.dirty_vram_flag {
                let current_viewport = ppu.get_viewport();
                window.update_with_buffer(current_viewport).unwrap();
            }
        }
    }
}
