// TODO: use rust book 17.2 as reference for part of setting up drawable trait and draw functions?
extern crate sdl2;

use sdl2::render::WindowCanvas;

pub trait Drawable {
    fn on_draw(&mut self, canvas: WindowCanvas);
}