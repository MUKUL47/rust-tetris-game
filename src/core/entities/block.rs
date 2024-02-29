use std::vec;
use rand::Rng;
use sdl2::{ keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas };
#[path = "../render_trait.rs"]
pub mod render_trait;
#[path = "../constants.rs"]
mod constants;
use render_trait::Entity;
use constants::util::*;
#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}
#[derive(Debug)]
enum Piece {
    I,
    J,
    L,
    S,
    Z,
    T,
    O,
}
const PIECES: [Piece; 7] = [Piece::I, Piece::J, Piece::L, Piece::S, Piece::Z, Piece::T, Piece::O];

#[derive(Clone, Debug)]
pub struct Block<'a> {
    active_piece: &'a Piece,
    active_piece_coordinates: Vec<Coordinate>,
    settled_pieces: std::collections::HashMap<String, Coordinate>,
}

impl<'a> Block<'a> {
    pub fn new() -> Self {
        let mut b = Block {
            active_piece: Block::get_random_piece(),
            active_piece_coordinates: vec![],
            settled_pieces: std::collections::HashMap::new(),
        };
        b.active_piece_coordinates = b.get_piece_coordinates(&b.active_piece);
        b
    }

    fn initialize_next_piece(&mut self) {
        while self.active_piece_coordinates.len() > 0 {
            let c = self.active_piece_coordinates.pop().unwrap();
            self.settled_pieces.insert(Block::get_coords_hash(c), c.clone());
        }
        self.active_piece = Block::get_random_piece();
        self.active_piece_coordinates = self.get_piece_coordinates(&self.active_piece);
    }

    fn get_coords_hash(c: Coordinate) -> String {
        c.x.to_string() + "" + &c.y.to_string()
    }

    fn get_random_piece() -> &'a Piece {
        let idx = rand::thread_rng().gen_range(0..7);
        &PIECES[idx]
    }

    fn get_piece_coordinates(&self, p: &Piece) -> Vec<Coordinate> {
        match p {
            Piece::I =>
                vec![
                    Coordinate { x: MID_WIDTH, y: 0 },
                    Coordinate { x: MID_WIDTH + 1, y: 0 },
                    Coordinate { x: MID_WIDTH + 2, y: 0 },
                    Coordinate { x: MID_WIDTH + 3, y: 0 }
                ],
            Piece::J =>
                vec![
                    Coordinate { x: MID_WIDTH, y: 0 },
                    Coordinate { x: MID_WIDTH, y: 1 },
                    Coordinate { x: MID_WIDTH + 1, y: 1 },
                    Coordinate { x: MID_WIDTH + 2, y: 1 }
                ],
            Piece::L =>
                vec![
                    Coordinate { x: MID_WIDTH, y: 0 },
                    Coordinate { x: MID_WIDTH - 2, y: 1 },
                    Coordinate { x: MID_WIDTH - 1, y: 1 },
                    Coordinate { x: MID_WIDTH, y: 1 }
                ],
            Piece::O =>
                vec![
                    Coordinate { x: MID_WIDTH, y: 0 },
                    Coordinate { x: MID_WIDTH + 1, y: 0 },
                    Coordinate { x: MID_WIDTH, y: 1 },
                    Coordinate { x: MID_WIDTH + 1, y: 1 }
                ],
            Piece::S =>
                vec![
                    Coordinate { x: MID_WIDTH, y: 0 },
                    Coordinate { x: MID_WIDTH + 1, y: 0 },
                    Coordinate { x: MID_WIDTH - 1, y: 1 },
                    Coordinate { x: MID_WIDTH, y: 1 }
                ],
            Piece::T =>
                vec![
                    Coordinate { x: MID_WIDTH, y: 0 },
                    Coordinate { x: MID_WIDTH - 1, y: 1 },
                    Coordinate { x: MID_WIDTH, y: 1 },
                    Coordinate { x: MID_WIDTH + 1, y: 1 }
                ],
            Piece::Z =>
                vec![
                    Coordinate { x: MID_WIDTH, y: 0 },
                    Coordinate { x: MID_WIDTH + 1, y: 0 },
                    Coordinate { x: MID_WIDTH + 1, y: 1 },
                    Coordinate { x: MID_WIDTH + 2, y: 1 }
                ],
        }
    }
    pub fn draw<'b>(&self, ctx: &'b mut Canvas<sdl2::video::Window>) {
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
        ctx.set_draw_color(Color::RGB(0, 100, 0));
        for c in self.settled_pieces.iter() {
            ctx.fill_rect(
                Rect::new(
                    c.1.x * (BLOCK_SIZE as i32),
                    c.1.y * (BLOCK_SIZE as i32),
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
        if self.is_y_blocked() {
            self.initialize_next_piece();
            return;
        }
        for c in self.active_piece_coordinates.iter_mut() {
            c.y = c.y + 1;
        }
    }

    fn is_y_blocked(&self) -> bool {
        match
            self.active_piece_coordinates.iter().find(|c| {
                let target: i32 = WIN_HEIGHT_OFFSET as i32;
                let c_y = (c.y + 1) * (BLOCK_SIZE as i32);
                if c_y >= target {
                    return true;
                }
                return self.settled_pieces.contains_key(
                    &(c.x.to_string() + "" + &(c.y + 1).to_string())
                );
            })
        {
            Some(_) => true,
            _ => false,
        }
    }

    fn is_x_blocked(&self, dir: i32) -> bool {
        match
            self.active_piece_coordinates.iter().find(|c| {
                let c_x: i32 = (c.x + dir) * (BLOCK_SIZE as i32);
                if
                    (dir == -1 && c_x < 0) ||
                    (dir == 1 && (c.x + 1) * (BLOCK_SIZE as i32) >= (WIN_WIDTH_OFFSET as i32))
                {
                    return true;
                }
                return self.settled_pieces.contains_key(
                    &((c.x + dir).to_string() + "" + &c.y.to_string())
                );
            })
        {
            Some(_) => true,
            _ => false,
        }
    }

    pub fn on_key(&mut self, key: Keycode) {
        match key {
            Keycode::Left | Keycode::Right => self.block_x_move(key),
            Keycode::Up => self.rotate_active_block(),
            _ => {}
        }
    }

    fn block_x_move(&mut self, key: Keycode) {
        let mut dir = 1;
        match key {
            Keycode::Left => {
                dir = -1;
            }
            _ => {}
        }
        if self.is_x_blocked(dir) {
            return;
        }
        for c in self.active_piece_coordinates.iter_mut() {
            c.x = c.x + dir;
        }
    }

    //https://github.com/MUKUL47/Tetris/blob/66cccc6ab951fa85457a4d2851c7df3824037c08/block.js#L56
    fn rotate_active_block(&mut self) {
        //not working as expected adjust rotation offset
        for piece in self.active_piece_coordinates.iter_mut() {
            let mut x = piece.x - MID_WIDTH - 1;
            let mut y = piece.y - 1;
            let x_1 = -y;
            let y_1 = x;
            x = x_1 + MID_WIDTH - 1;
            y = y_1 + 1;
            piece.x = x;
            piece.y = y;
        }
    }
}
