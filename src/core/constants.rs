pub mod util {
    pub static BLOCK_SIZE: u32 = 20;
    pub static WIN_WIDTH: u32 = 400;
    pub static WIN_HEIGHT: u32 = 600;
    pub static WIN_HEIGHT_OFFSET: u32 = (WIN_HEIGHT / BLOCK_SIZE) * BLOCK_SIZE;
    pub static WIN_WIDTH_OFFSET: u32 = (WIN_WIDTH / BLOCK_SIZE) * BLOCK_SIZE;
    pub static MID_WIDTH: i32 = (WIN_WIDTH / BLOCK_SIZE / 2 - 2) as i32;
}
