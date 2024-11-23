use ironboy_core::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

const SCALE: u32 = 6;
const WINDOW_WIDTH: u32 = (VIEWPORT_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (VIEWPORT_HEIGHT as u32) * SCALE;

pub fn create_canvas(sdl: &sdl2::Sdl) -> Canvas<Window> {
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Iron Boy", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();
    window.into_canvas().present_vsync().accelerated().build().unwrap()
}

pub fn render_screen(canvas: &mut Canvas<Window>, data: &[(u8, u8, u8)]) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for x in 0..VIEWPORT_WIDTH {
        for y in 0..VIEWPORT_HEIGHT {
            let i = y * VIEWPORT_WIDTH + x;
            let color = data[i as usize];
            canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
            let rect = Rect::new(
                (x as u32 * SCALE) as i32,
                (y as u32 * SCALE) as i32,
                SCALE + 4, // change this if you want line speration
                SCALE + 4, // change this if you want line speration
            );
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}
