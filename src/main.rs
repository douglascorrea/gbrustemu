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
    let mut ppu = PPU::new();
    let mut mmu = MMU::new();
    mmu.from_rom_file(&rom_file);

    // run make CPU run instructions over ram
    //    println!("MMU BEFORE: {:?}", mmu);
    let mut cpu = CPU::new();
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
        let now = Instant::now();
        cpu.run_instruction(&mut mmu, &mut ppu);
        if ppu.is_lcd_enable(&mmu) {
            ppu.populate_background_buffer(&mmu);
            let current_viewport = ppu.transform_background_buffer_into_screen(&mmu);
            window.update_with_buffer(&current_viewport).unwrap();
        }
        let new_now = Instant::now();
        println!("{:?}", new_now.duration_since(now))
    }
}
