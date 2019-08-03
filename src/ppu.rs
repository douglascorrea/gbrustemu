use crate::mmu::MMU;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

pub const DARKEST_GREEN: u32 = 0xFF0F380F;
pub const DARK_GREEN: u32 = 0xFF306230;
pub const LIGHT_GREEN: u32 = 0xFF8BAC0F;
pub const LIGHTEST_GREEN: u32 = 0xFF9BBC0F;

pub struct PPU {
    mode: u8,
    mode_clock: usize,
    background_buffer: Vec<u32>,
}

impl PPU {
    pub fn new() -> PPU {
        let ppu = PPU {
            mode: 0,
            background_buffer: vec![LIGHTEST_GREEN; WIDTH * HEIGHT],
            mode_clock: 0,
        };
        ppu
    }

    pub fn get_lcdc(&self, mmu: &MMU) -> u8 {
        mmu.read_byte(0xFF40)
    }

    pub fn get_bgp(&self, mmu: &MMU) -> u8 {
        mmu.read_byte(0xFF47)
    }

    pub fn is_lcd_enable(&self, mmu: &MMU) -> bool {
        (self.get_lcdc(mmu) & 0b1000_0000) != 0
    }

    pub fn get_tile_set(&self, mmu: &MMU) -> [[u8; 16]; 256] {
        // @TODO check LCDC
        let mut tile_set = [[0; 16]; 256];

        for i in 0..256 {
            tile_set[i] = self.get_tile(&mmu, (0x8000 + (i * 16)) as u16);
        }
        tile_set
    }

    pub fn get_tile_map(&self, mmu: &MMU) -> [u8; 1_024] {
        let mut tile_map: [u8; 1024] = [0; 1_024];

        for i in 0..1_024 {
            tile_map[i] = mmu.read_byte((0x9800 + i) as u16);
        }
        tile_map
    }

    pub fn get_tile(&self, mmu: &MMU, first_tile_byte_addr: u16) -> [u8; 16] {
        let mut tile = [0; 16];
        for i in 0..16 {
            tile[i] = mmu.read_byte(first_tile_byte_addr + i as u16);
        }
        tile
    }

    pub fn populate_background_buffer(&mut self, mmu: &MMU) {
        // get the tile set
        let tile_set = self.get_tile_set(mmu);
        // get the tile map
        let tile_map = self.get_tile_map(mmu);
        // populate the background_buffer accordingly to tile_map AND tranform tile to minifb tile
        // in the process
        for (i, tile_map_item) in tile_map.iter().enumerate() {
            let tile = tile_set[*tile_map_item as usize];
            let minifb_tile = self.transform_tile_to_minifb_tile(mmu, tile);
            for (j, pixel) in minifb_tile.iter().enumerate() {
                self.background_buffer[j + (i * 64) as usize] = *pixel;
            }
        }
    }

    pub fn get_background_buffer(&self) -> &Vec<u32> {
        &self.background_buffer
    }

    pub fn transform_pair_into_bgp_palette(&self, mmu: &MMU, pixel_pair: u8) -> u8 {
        let bgp_palette = self.get_bgp(&mmu);
        //        println!("bgp_palette: {:?}", bgp_palette);
        match pixel_pair {
            0b00 => bgp_palette & 0b0000_0011,
            0b01 => (bgp_palette & 0b0000_1100) >> 2,
            0b10 => (bgp_palette & 0b0011_0000) >> 4,
            0b11 => (bgp_palette & 0b1100_0000) >> 4,
            _ => bgp_palette & 0b0000_0011,
        }
    }

    pub fn transform_from_bgp_to_minifb_color(&self, bgp_palette: u8) -> u32 {
        match bgp_palette {
            0b00 => LIGHTEST_GREEN,
            0b01 => LIGHT_GREEN,
            0b10 => DARK_GREEN,
            0b11 => DARKEST_GREEN,
            _ => LIGHTEST_GREEN,
        }
    }

    pub fn transform_tile_to_minifb_tile(&self, mmu: &MMU, tile: [u8; 16]) -> Vec<u32> {
        let mut minifb_tile = vec![0; 64];
        for i in (0..tile.len()).step_by(2) {
            let pixel_part_1 = tile[i];
            let pixel_part_2 = tile[i + 1];
            for j in 0..8 {
                let bit_part_1 = pixel_part_1 & (1 << j) != 0;
                let bit_part_2 = pixel_part_2 & (1 << j) != 0;
                let pair = ((bit_part_1 as u8) << 1) | (bit_part_2 as u8);
                // TRANSFORM THIS PAIR INTO BGP PALETTE
                let bgp_palette = self.transform_pair_into_bgp_palette(&mmu, pair);
                // TRANSFORM INTO MINIFB COLOR
                let minifb = self.transform_from_bgp_to_minifb_color(bgp_palette);

                //                minifb_tile[i / 2][7 - j] = minifb;
                minifb_tile[(i / 2 * 8) + (7 - j) as usize] = minifb;
            }
        }
        minifb_tile
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
                // this happen on HBLANK
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
