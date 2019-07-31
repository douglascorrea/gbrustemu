use crate::mmu::MMU;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

#[derive(Debug)]
pub struct PPU {
    mode: u8,
    mode_clock: usize,
    //    buffer: Vec<u32>,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            mode: 0,
            //            buffer: vec![0; WIDTH * HEIGHT],
            mode_clock: 0,
        }
    }

    pub fn step(&mut self, cpu_clocks_passed: usize, mmu: &mut MMU) {
        let lcdc: u8 = mmu.read_byte(0xFF40);
        let is_lcd_enable = (lcdc & 0b1000_0000) != 0;
        if is_lcd_enable {
            // increment our internal clock
            self.mode_clock += cpu_clocks_passed;
            // check which mode we are
            let mut ly: u8 = mmu.read_byte(0xFF44);
            if self.mode_clock > 456 && self.mode != 1 {
                ly = ly.wrapping_add(1);
                mmu.write_byte(0xFF44, ly);
                if ly <= 144 {
                    self.mode_clock = 0;
                }
            }

            match self.mode_clock {
                t if t <= 80 => self.mode = 2,
                t if t <= 252 => self.mode = 3,
                t if t <= 456 => self.mode = 0,
                t if t <= 4560 => self.mode = 1,
                t if t > 4560 => {
                    self.mode = 2;
                    self.mode_clock = 0;
                    if ly > 154 {
                        mmu.write_byte(0xFF44, 0);
                    }
                }
                _ => panic!("Not handled mode_clock"),
            }

            // change the appropriated PPU register (LY, LYC, STAT)
            // @TODO Check LYC behavior
            let lyc = mmu.read_byte(0xFF45);
            let stat_bit_0_to_2: u8 = match ly == lyc {
                true => 0b100 | self.mode,
                false => self.mode,
            } as u8;
            let mut current_stat = mmu.read_byte(0xFF41);
            current_stat = current_stat & 0b11111000;
            current_stat = current_stat | stat_bit_0_to_2;
            // set STAT register
            mmu.write_byte(0xFF41, current_stat);
        }
    }
}
