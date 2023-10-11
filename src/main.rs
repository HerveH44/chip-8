mod cpu;
mod screen;
mod decoder;
mod opcode;

use std::fs::File;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use simplelog::{Config, LevelFilter, WriteLogger};

fn main() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("chip8.log").unwrap(),
    ).unwrap();

    // DSL --
    let sdl_context = sdl2::init().unwrap();
    let mut screen = screen::new(&sdl_context);
    let mut cpu = cpu::new(&mut screen);
    cpu.load_fonts("./roms/fonts.ch8").unwrap();
    cpu.load_rom("./roms/ibm.ch8").unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let start = SystemTime::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        cpu.tick();

        sleep(Duration::from_secs_f64(1.0 / 3000.0).saturating_sub(start.elapsed().unwrap()));
    }
}
