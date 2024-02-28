mod core;
extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::Canvas;
use std::time::Duration;
use crate::core::render_trait;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tetris", core::constants::util::WIN_WIDTH, core::constants::util::WIN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas: Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut tetris = core::tetris::Tetris::new(&mut canvas);
    'main_loop: loop {
        tetris.draw();
        tetris.update();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop;
                }
                Event::KeyDown { keycode, .. } => {
                    tetris.on_key(keycode.unwrap());
                }
                _ => {}
            }
        }
        std::thread::sleep(Duration::new(0, 15_000_0000));
    }
}
