use std::vec;
use sdl2::{ pixels::Color, rect::Rect, render::Canvas };
#[path = "../render_trait.rs"]
pub mod render_trait;
#[path = "../constants.rs"]
mod constants;
use render_trait::Entity;
use constants::util::*;
struct Coordinate {
    x: i32,
    y: i32,
}
enum Piece {
    I,
    J,
    L,
    S,
    Z,
    T,
}
pub struct Block {
    active_piece: Piece,
    active_piece_coordinates: Vec<Coordinate>,
    settled_pieces: Vec<Coordinate>,
}

impl Block {
    pub fn new() -> Self {
        let mut b = Block {
            active_piece: Block::get_random_piece(),
            active_piece_coordinates: vec![],
            settled_pieces: vec![],
        };
        b.active_piece_coordinates = b.get_piece_coordinates(&b.active_piece);
        b
    }

    fn get_random_piece() -> Piece {
        // todo!("get_random_piece");
        Piece::I
    }

    fn get_piece_coordinates(&self, p: &Piece) -> Vec<Coordinate> {
        match p {
            _ =>
                vec![
                    Coordinate { x: 10, y: 0 },
                    Coordinate { x: 10, y: 1 },
                    Coordinate { x: 10, y: 2 }
                ],
        }
    }
    pub fn draw<'a>(&self, ctx: &'a mut Canvas<sdl2::video::Window>) {
        ctx.set_draw_color(Color::RGB(0, 255, 0));
        for c in self.active_piece_coordinates.iter() {
            ctx.fill_rect(
                Rect::new(
                    c.x * (BLOCK_SIZE as i32),
                    c.y * (BLOCK_SIZE as i32),
                    BLOCK_SIZE as u32,
                    BLOCK_SIZE as u32
                )
            ).unwrap();
        }
    }
    pub fn update(&mut self) {
        self.move_active_piece()
    }

    fn move_active_piece(&mut self) {
        for c in self.active_piece_coordinates.iter_mut() {
            c.y = c.y + 1;
        }
    }
}
