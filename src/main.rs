mod cpu;
mod decoder;
mod opcode;
mod renderer;

use crate::cpu::Cpu;
use log::info;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use simplelog::{
    ColorChoice, CombinedLogger, Config, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
    WriteLogger,
};
use std::collections::HashSet;
use std::fs::File;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            ConfigBuilder::new()
                .set_time_level(LevelFilter::Off)
                .build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("chip8.log").unwrap(),
        ),
    ])
    .unwrap();

    // DSL --
    let sdl_context = sdl2::init().unwrap();
    let mut screen = renderer::new(&sdl_context);
    let mut cpu = Cpu::default();

    // CPU -- Loading fonts and rom
    cpu.load_fonts("./roms/fonts.ch8").unwrap();
    cpu.load_rom("./roms/invaders.ch8").unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let start = SystemTime::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    scancode: Some(scancode),
                    ..
                } => {
                    // info!("Key pressed {keycode} with scancode {scancode}");
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
                        _ => None,
                    };

                    if let Some(code) = key {
                        cpu.set_key_pressed(code);
                    } else {
                        cpu.set_key_released();
                    }
                }
                Event::KeyUp { .. } => {
                    cpu.set_key_released();
                }
                _ => {}
            }
        }
        let keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .map(|scancode| match scancode {
                Scancode::Num1 => 0x1,
                Scancode::Num2 => 0x2,
                Scancode::Num3 => 0x3,
                Scancode::Num4 => 0xC,
                Scancode::Q => 0x4,
                Scancode::W => 0x5,
                Scancode::E => 0x6,
                Scancode::R => 0xD,
                Scancode::A => 0x7,
                Scancode::S => 0x8,
                Scancode::D => 0x9,
                Scancode::F => 0xE,
                Scancode::Z => 0xA,
                Scancode::X => 0x0,
                Scancode::C => 0xB,
                Scancode::V => 0xF,
                _ => 0x0,
            })
            .collect::<HashSet<u8>>();
        cpu.set_keys_pressed(keys);

        cpu.tick_timers();
        cpu.tick();
        if cpu.should_render {
            screen.render(cpu.screen);
            cpu.should_render = false;
        }

        sleep(Duration::from_secs_f64(1.0 / 3000.0).saturating_sub(start.elapsed().unwrap()));
    }
}
