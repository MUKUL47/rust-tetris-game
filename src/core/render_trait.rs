use sdl2::render::Canvas;

pub trait Entity {
    fn draw<'a>(ctx: &'a mut Canvas<sdl2::video::Window>);
    fn update();
}
