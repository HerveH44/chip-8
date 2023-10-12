mod cpu;
mod screen;
mod decoder;
mod opcode;

use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
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
    let mut cpu = Arc::new(Mutex::new(cpu::new(&mut screen)));

    // CPU -- Loading fonts and rom
    let mut guard = cpu.lock().unwrap();
    guard.load_fonts("./roms/fonts.ch8").unwrap();
    guard.load_rom("./roms/4-flags.ch8").unwrap();
    drop(guard);

    let cpu_clone = Arc::clone(&cpu);
    thread::spawn(move || {
        loop {
            let start = SystemTime::now();
            let mut timers_cpu = cpu_clone.lock().unwrap();
            timers_cpu.tick_timers();
            drop(timers_cpu);
            sleep(Duration::from_secs_f64(1.0 / 60.0).saturating_sub(start.elapsed()?));
        }
    });

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
