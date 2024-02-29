use core::fmt;
use std::path;
use crate::Canvas;

#[path = "./constants.rs"]
mod constants;
#[path = "./render_trait.rs"]
pub mod render_trait;
#[path = "./entities/mod.rs"]
mod entities;

use constants::util::BLOCK_SIZE;
use sdl2::{ keyboard::Keycode, pixels::Color, sys::{ KeyCode, Window } };
use entities::grid::Grid;

use self::entities::block::Block;
pub use self::entities::grid::render_trait::Entity;

pub struct Tetris<'a> {
    ctx: &'a mut Canvas<sdl2::video::Window>,
    tetris_block: Block<'a>,
}
impl<'a> Tetris<'a> {
    pub fn new(ctx: &'a mut Canvas<sdl2::video::Window>) -> Self {
        Tetris {
            ctx,
            tetris_block: Block::new(),
        }
    }
    pub fn draw(&mut self) {
        Grid::draw(self.ctx);
        self.tetris_block.draw(self.ctx);
    }

    pub fn reset(&mut self) {
        self.ctx.set_draw_color(Color::RGB(0, 0, 0));
        self.ctx.present();
        self.ctx.clear();
    }

    pub fn on_key(&mut self, key: Keycode) {
        self.tetris_block.on_key(key);
    }
    pub fn update(&mut self) {
        self.tetris_block.update();
    }
}
