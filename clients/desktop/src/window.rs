use getset::{Getters, MutGetters};
use ironboy_core::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use sdl2::{
    IntegerOrSdlError, Sdl, VideoSubsystem,
    image::{self, InitFlag, LoadTexture},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::{Window, WindowBuildError},
};
use thiserror::Error;

const SCALE: u32 = 6;
const MAIN_WINDOW_TITLE: &str = "Iron Boy";
const MAIN_WINDOW_WIDTH: u32 = (VIEWPORT_WIDTH as u32) * SCALE;
const MAIN_WINDOW_HEIGHT: u32 = (VIEWPORT_HEIGHT as u32) * SCALE;

#[derive(Error, Debug)]
pub enum WindowError {
    #[error("Failed to initialize image context")]
    ImageInitError(String),
    #[error("Failed to create video subsystem: {0}")]
    VideoSubsystemError(String),
    #[error("Failed to create window")]
    WindowBuildError(#[from] WindowBuildError),
    #[error("Failed to create canvas from window")]
    CanvasBuildError(#[from] IntegerOrSdlError),
    #[error("Failed to load texture")]
    TextureLoadError(String),
}

#[derive(Getters, MutGetters)]
pub struct WindowManager {
    video_subsystem: VideoSubsystem,
    #[getset(get = "pub", get_mut = "pub")]
    main_canvas: Canvas<Window>,
}

impl WindowManager {
    pub fn new(sdl_context: &Sdl) -> Result<WindowManager, WindowError> {
        image::init(InitFlag::PNG).map_err(WindowError::ImageInitError)?;

        let video_subsystem = sdl_context.video().map_err(WindowError::VideoSubsystemError)?;
        let window = video_subsystem
            .window(MAIN_WINDOW_TITLE, MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT)
            .position_centered()
            .resizable()
            .opengl()
            .build()?;

        let main_canvas = window.into_canvas().present_vsync().accelerated().build()?;

        Ok(Self {
            video_subsystem,
            main_canvas,
        })
    }

    pub fn create_canvas(&mut self, title: &str, width: u32, height: u32, x: i32, y: i32) -> Result<Canvas<Window>, WindowError> {
        let window = self
            .video_subsystem
            .window(title, width, height)
            .position(x, y)
            .resizable()
            .opengl()
            .build()?;
        let canvas = window.into_canvas().present_vsync().accelerated().build()?;
        Ok(canvas)
    }

    pub fn render_screen(&mut self, data: &[(u8, u8, u8)]) {
        self.main_canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.main_canvas.clear();

        for x in 0..VIEWPORT_WIDTH {
            for y in 0..VIEWPORT_HEIGHT {
                let i = y * VIEWPORT_WIDTH + x;
                let color = data[i as usize];
                self.main_canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
                let rect = Rect::new(
                    (x as u32 * SCALE) as i32,
                    (y as u32 * SCALE) as i32,
                    SCALE + 4, // change this if you want line speration
                    SCALE + 4, // change this if you want line speration
                );
                self.main_canvas.fill_rect(rect).unwrap();
            }
        }

        self.main_canvas.present();
    }

    pub fn render_splash(&mut self) -> Result<(), WindowError> {
        let texture_creator = self.main_canvas.texture_creator();
        let texture = texture_creator
            .load_texture("media/ironboy_logo.png")
            .map_err(WindowError::TextureLoadError)?;

        self.main_canvas.set_draw_color(Color::RGB(0, 64, 255));
        self.main_canvas.clear();

        self.main_canvas.copy(&texture, None, None).unwrap();

        self.main_canvas.present();

        Ok(())
    }
}
