use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub const GRID_X_SIZE: usize = 64;
pub const GRID_Y_SIZE: usize = 32;
const DOT_SIZE_IN_PXS: u32 = 10;

const BACKGROUND_COLOR: Color = Color::RGB(134, 84, 3);
const FONT_COLOR: Color = Color::RGB(253, 195, 10);


pub struct Renderer {
    pub canvas: WindowCanvas,
}

impl Renderer {

    pub fn render(&mut self, screen: [[bool; GRID_X_SIZE]; GRID_Y_SIZE]) {
        self.canvas.set_draw_color(BACKGROUND_COLOR);
        self.canvas.clear();

        self.canvas.set_draw_color(FONT_COLOR);
        screen.iter().enumerate().for_each(|(index, line)| {
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

pub fn new(sdl_context: &Sdl) -> Renderer {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(
        "rust-sdl2 demo",
        (GRID_X_SIZE) as u32 * DOT_SIZE_IN_PXS,
        (GRID_Y_SIZE) as u32 * DOT_SIZE_IN_PXS
    )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    Renderer {
        canvas,
    }
}
