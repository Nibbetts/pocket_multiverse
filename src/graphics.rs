// TODO: use rust book 17.2 as reference for part of setting up drawable trait and draw functions?
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::WindowCanvas;
use crate::engine::{Engine, StepStatus};
use crate::entities::StepUpdates;

const WINDOW_TITLE: &str = "Pocket Multiverse";
// const WINDOW_DIMENSIONS: (u32, u32) = (1920, 1080);
const WINDOW_DIMENSIONS: (u32, u32) = (1280, 1024);

pub fn test() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (w, h) = WINDOW_DIMENSIONS;
    let window = video_subsystem.window(WINDOW_TITLE, w, h)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut engine = Engine::new(event_pump);

    let mut i = 0;
    loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        let result = engine.on_step();
        if matches!(result, StepStatus::EXIT) { return }

        // engine.draw_all(canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub trait Drawable {
    fn on_draw(&mut self, canvas: WindowCanvas);
}