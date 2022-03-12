pub const KB: usize = 1024;
pub const MB: usize = 1024 * 1024;

pub const MAX_ROM_SIZE: usize = 8 * MB;
pub const WRAM_SIZE: usize = 128 * KB;
pub const SRAM_SIZE: usize = 512 * KB;
pub const VRAM_SIZE: usize = 512 * KB;
pub const ROM_SIZE: usize = (MAX_ROM_SIZE + 0x200 + 0x8000);
