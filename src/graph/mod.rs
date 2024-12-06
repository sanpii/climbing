pub mod bar;
pub mod kind;
pub mod pie;
pub mod score;
pub mod wall;

pub const SIZE: (u32, u32) = (740 * 2, 540 * 2);
pub const LEN: usize = SIZE.0 as usize * SIZE.1 as usize * 3;
