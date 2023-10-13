mod cpu;
mod renderer;
mod decoder;
mod opcode;

use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use log::info;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use simplelog::{Config, LevelFilter, WriteLogger};

fn main() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("chip8.log").unwrap(),
    ).unwrap();

    // DSL --
    let sdl_context = sdl2::init().unwrap();
    let mut screen = renderer::new(&sdl_context);
    let cpu = Arc::new(Mutex::new(cpu::new()));
    let cpu_clone = Arc::clone(&cpu);

    // CPU -- Loading fonts and rom
    let mut guard = cpu.lock().unwrap();
    guard.load_fonts("./roms/fonts.ch8").unwrap();
    guard.load_rom("./roms/invaders.ch8").unwrap();
    drop(guard);

    thread::spawn(move || {
        loop {
            let start = SystemTime::now();

            let mut timers_cpu = cpu_clone.lock().unwrap();
            timers_cpu.tick_timers();
            drop(timers_cpu);

            sleep(Duration::from_secs_f64(1.0 / 60.0).saturating_sub(start.elapsed().unwrap()));
        }
    });

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let start = SystemTime::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(keycode), scancode: Some(scancode), .. } => {
                    info!("Key pressed {keycode} with scancode {scancode}");
                    let key: Option<u16> = match scancode {
                        Scancode::Num1 => Some(0x1),
                        Scancode::Num2 => Some(0x2),
                        Scancode::Num3 => Some(0x3),
                        Scancode::Num4 => Some(0xC),
                        Scancode::Q => Some(0x4),
                        Scancode::W => Some(0x5),
                        Scancode::E => Some(0x6),
                        Scancode::R => Some(0xD),
                        Scancode::A => Some(0x7),
                        Scancode::S => Some(0x8),
                        Scancode::D => Some(0x9),
                        Scancode::F => Some(0xE),
                        Scancode::Z => Some(0xA),
                        Scancode::X => Some(0x0),
                        Scancode::C => Some(0xB),
                        Scancode::V => Some(0xF),
                        _ => None
                    };

                    let mut cpu_guard = cpu.lock().unwrap();
                    if let Some(code) = key {
                        cpu_guard.set_key_pressed(code);
                    } else {
                        cpu_guard.set_key_released();
                    }
                }
                Event::KeyUp { .. } => {
                    let mut cpu_guard = cpu.lock().unwrap();
                    cpu_guard.set_key_released();
                }
                _ => {}
            }
        }
        let mut cpu_guard = cpu.lock().unwrap();
        cpu_guard.tick();
        if cpu_guard.should_render {
            screen.render(cpu_guard.screen);
            cpu_guard.should_render = false;
        }
        drop(cpu_guard);

        sleep(Duration::from_secs_f64(1.0 / 300.0).saturating_sub(start.elapsed().unwrap()));
    }
}
