use getset::{Getters, MutGetters};
use ironboy_core::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use sdl2::{
    Sdl, VideoSubsystem,
    image::{self, InitFlag, LoadTexture},
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::{self, Sdl2TtfContext},
    video::{Window, WindowContext},
};
use thiserror::Error;

const SCALE: u32 = 6;
const FPS_FONT_SIZE: u16 = 24;
const FPS_PADDING: i32 = 10;
const FONT_PATH: &str = "media/gbboot-alpm.ttf";
const SPLASH_PADDING: u32 = 20;
const SPLASH_PATH: &str = "media/ironboy_logo.png";

#[derive(Error, Debug)]
pub enum WindowError {
    #[error("Failed to create video subsystem: {0}")]
    VideoSubsystemError(String),
    #[error("Failed to initialize image context: {0}")]
    ImageInitError(String),
    #[error("Failed to initialize TTF context: {0}")]
    TtfInitError(String),
    #[error("Failed to create window: {0}")]
    WindowBuildError(#[from] sdl2::video::WindowBuildError),
    #[error("Failed to create canvas from window: {0}")]
    CanvasBuildError(#[from] sdl2::IntegerOrSdlError),
    #[error("There was a canvas error: {0}")]
    CanvasError(String),
    #[error("There was a texture error: {0}")]
    TextureError(String),
    #[error("Failed to load font: {0}")]
    FontLoadError(String),
    #[error("Failed to render text: {0}")]
    TextRenderError(String),
}

#[derive(Getters, MutGetters)]
pub struct WindowManager {
    video_subsystem: VideoSubsystem,
    #[getset(get = "pub", get_mut = "pub")]
    main_canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: Sdl2TtfContext,
}

impl WindowManager {
    pub fn new(sdl_context: &Sdl) -> Result<WindowManager, WindowError> {
        image::init(InitFlag::PNG).map_err(WindowError::ImageInitError)?;
        let ttf_context = ttf::init().map_err(WindowError::TtfInitError)?;

        let video_subsystem = sdl_context.video().map_err(WindowError::VideoSubsystemError)?;
        let window = video_subsystem
            .window("Iron Boy", (VIEWPORT_WIDTH as u32) * SCALE, (VIEWPORT_HEIGHT as u32) * SCALE)
            .position_centered()
            .resizable()
            .opengl()
            .build()?;

        let main_canvas = window.into_canvas().present_vsync().accelerated().build()?;
        let texture_creator = main_canvas.texture_creator();

        Ok(Self {
            video_subsystem,
            main_canvas,
            texture_creator,
            ttf_context,
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

    pub fn render_screen(&mut self, data: &[(u8, u8, u8)], fps: Option<f64>) -> Result<(), WindowError> {
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
                self.main_canvas.fill_rect(rect).map_err(WindowError::CanvasError)?;
            }
        }

        if let Some(fps_value) = fps {
            self.render_fps_overlay(fps_value)?;
        }

        self.main_canvas.present();
        Ok(())
    }

    fn render_fps_overlay(&mut self, fps: f64) -> Result<(), WindowError> {
        let font = self.ttf_context.load_font(FONT_PATH, FPS_FONT_SIZE).map_err(WindowError::FontLoadError)?;

        let fps_text = format!("{:.1} FPS", fps);
        let surface = font
            .render(&fps_text)
            .blended(Color::RGB(0, 255, 0))
            .map_err(|e| WindowError::TextRenderError(e.to_string()))?;

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| WindowError::TextureError(e.to_string()))?;

        let (window_width, window_height) = self.main_canvas.output_size().map_err(WindowError::CanvasError)?;
        let text_width = surface.width();
        let text_height = surface.height();

        let x = (window_width - text_width) as i32 - FPS_PADDING;
        let y = (window_height - text_height) as i32 - FPS_PADDING;

        let dst_rect = Rect::new(x, y, text_width, text_height);
        self.main_canvas.copy(&texture, None, dst_rect).map_err(WindowError::CanvasError)?;

        Ok(())
    }

    pub fn render_splash(&mut self) -> Result<(), WindowError> {
        let texture = self.texture_creator.load_texture(SPLASH_PATH).map_err(WindowError::TextureError)?;

        self.main_canvas.set_draw_color(Color::RGB(45, 45, 45));
        self.main_canvas.clear();

        let (window_width, window_height) = self.main_canvas.output_size().map_err(WindowError::CanvasError)?;
        let texture_query = texture.query();
        let texture_width = texture_query.width;
        let texture_height = texture_query.height;

        let scale_x = window_width as f32 / texture_width as f32;
        let scale_y = window_height as f32 / window_height as f32;
        let scale = scale_x.min(scale_y);

        let scaled_width = (texture_width as f32 * scale) as u32 - SPLASH_PADDING;
        let scaled_height = (texture_height as f32 * scale) as u32;

        let x = (window_width - scaled_width) / 2;
        let y = (window_height - scaled_height) / 2;

        let dst_rect = Rect::new(x as i32, y as i32, scaled_width, scaled_height);

        self.main_canvas.copy(&texture, None, dst_rect).map_err(WindowError::CanvasError)?;

        self.main_canvas.present();

        Ok(())
    }
}
