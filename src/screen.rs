use std::ops::BitXor;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

const GRID_X_SIZE: u32 = 64;
const GRID_Y_SIZE: u32 = 32;
const DOT_SIZE_IN_PXS: u32 = 20;

pub struct Screen {
    pub canvas: WindowCanvas,
    pub screen: [[bool; 64]; 32],
}

impl Screen {

    pub fn clear(&mut self) {
        for (_, row) in self.screen.iter_mut().enumerate() {
            for (_, col) in row.iter_mut().enumerate() {
                *col = false;
            }
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, value: bool) -> bool {
        let current_sprite_value = self.screen[y][x];
        let new_sprite_value = current_sprite_value.bitxor(value);

        self.screen[y][x] = new_sprite_value;

        if current_sprite_value == true && new_sprite_value == false {
            return true;
        }
        false
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::YELLOW);
        self.screen.iter().enumerate().for_each(|(index, line)| {
            line.iter().enumerate().for_each(|(sprite_index, sprite)| {
                if *sprite {
                    self.canvas.fill_rect(Rect::new(
                        (sprite_index * DOT_SIZE_IN_PXS as usize) as i32,
                        (index * DOT_SIZE_IN_PXS as usize) as i32,
                        DOT_SIZE_IN_PXS,
                        DOT_SIZE_IN_PXS,
                    )).unwrap();
                }
            })
        });
        self.canvas.present()
    }

}

pub fn new(sdl_context: &Sdl) -> Screen {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(
        "rust-sdl2 demo",
        GRID_X_SIZE * DOT_SIZE_IN_PXS,
        GRID_Y_SIZE * DOT_SIZE_IN_PXS
    )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    Screen {
        canvas,
        screen: [[false; 64]; 32]
    }
}
