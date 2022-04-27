extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sdl2::EventPump;
use sdl2::render::WindowCanvas;
use crate::engine::StepStatus::{EXIT, NORMAL};
use crate::entities::StepUpdates;
use crate::universes::Bounds;

use crate::graphics::Drawable;

const WINDOW_TITLE: &str = "Pocket Multiverse";
// const WINDOW_DIMENSIONS: (u32, u32) = (1920, 1080);
// const WINDOW_DIMENSIONS: (u32, u32) = (1280, 1024);
const WINDOW_DIMENSIONS: (u32, u32) = (1920, 1024);

pub struct Engine {
    event_pump: EventPump,

    start_time: f32,
    current_time: f32,
    last_time: f32,

    // updatable: Vec<StepUpdates>,
    updatable: Vec<Box<dyn StepUpdates>>,
    drawable: Vec<Box<dyn Drawable>>,
    // drawable: Vec<Drawable>,

    running: bool,
    canvas: WindowCanvas,
}
pub enum StepStatus {
    NORMAL,
    PAUSE,
    EXIT,
}
impl Engine {
    pub fn new() -> Engine {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let (w, h) = WINDOW_DIMENSIONS;
        let window = video_subsystem.window(WINDOW_TITLE, w, h)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();


        let time = Engine::get_time();
        Engine {
            event_pump,
            start_time: time,
            current_time: time,
            last_time: time,
            updatable: vec![],
            drawable: vec![],
            running: true,
            canvas,
        }
    }

    pub fn start(&mut self) {
        loop {
            self.canvas.clear();

            let result = self.on_step();
            if matches!(result, StepStatus::EXIT) { return }

            // self.draw_all(self.canvas);

            self.canvas.present();
        }
    }

    fn get_time() -> f32 { SystemTime::now().duration_since(UNIX_EPOCH).expect("error").as_secs_f32() }

    pub fn on_step(&mut self) -> StepStatus {
        self.current_time = Engine::get_time();
        let delta_time = self.current_time - self.last_time;

        self.handle_input();

        if !self.running { return EXIT };

        for i in 0..self.updatable.len() {
            self.updatable[i].step(delta_time);
        }

        self.last_time = self.current_time;

        return NORMAL
    }
    
    pub fn handle_input(&mut self) {
        let mut exit = false;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    exit = true;
                    break
                },
                _ => {}
            }
        }
        if exit { self.exit(); }
    }

    pub fn draw_all(&mut self, canvas: &WindowCanvas) {
        for i in 0..self.drawable.len() {
            // self.drawable[i].on_draw(canvas)
        }
    }

    fn exit(&mut self) {
        self.running = false;
    }
}


