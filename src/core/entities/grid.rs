use std::path;
use sdl2::{ pixels::Color, rect::Rect, render::Canvas };
#[path = "../render_trait.rs"]
pub mod render_trait;
#[path = "../constants.rs"]
mod constants;

use render_trait::Entity;
use constants::util::*;
pub struct Grid;
impl Entity for Grid {
    fn update() {}
    fn draw<'a>(ctx: &'a mut Canvas<sdl2::video::Window>) {
        ctx.set_draw_color(Color::RGB(0, 50, 0));
        for i in 0..=WIN_WIDTH_OFFSET - 1 {
            for j in 0..=WIN_HEIGHT_OFFSET - 1 {
                ctx.draw_rect(
                    Rect::new(
                        (i * BLOCK_SIZE) as i32,
                        (j * BLOCK_SIZE) as i32,
                        BLOCK_SIZE as u32,
                        BLOCK_SIZE as u32
                    )
                ).unwrap();
            }
        }
    }
}
