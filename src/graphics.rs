use sdl::event::{Event, Key};
use sdl::video::{Color, Surface, SurfaceFlag, VideoFlag};
use std::{thread, time};

pub struct Gfx {
    screen: Surface,
    width: i16,
    height: i16,
    bpp: i32,
    pixel_size: i16,
}

pub trait Pipeline {
    fn init(&mut self);
    fn update(&mut self);
    fn draw(&mut self, gfx: &Gfx);
    fn quit(&self);
}

pub fn build(width: i16, height: i16, bpp: i32, pixel_size: i16) -> Gfx {
    //Start SDL
    if sdl::init(&[sdl::InitFlag::Everything]) == false {
        panic!("Could not init SDL");
    }

    sdl::wm::set_caption("Game of life", "rust-sdl");

    let screen = match sdl::video::set_video_mode(
        width as isize,
        height as isize,
        bpp as isize,
        &[SurfaceFlag::HWSurface],
        &[VideoFlag::DoubleBuf],
    ) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err),
    };

    let gfx = Gfx {
        screen,
        width,
        height,
        bpp,
        pixel_size,
    };

    return gfx;
}

impl Gfx {
    pub fn run(&mut self, pipe: &mut impl Pipeline) {
        pipe.init();

        'main: loop {
            'event: loop {
                match sdl::event::poll_event() {
                    Event::Quit => break 'main,
                    Event::None => break 'event,
                    Event::Key(k, _, _, _) if k == Key::Escape => break 'main,
                    Event::Key(k, _, _, _) if k == Key::Num0 => {
                        let max_zoom = 80;

                        if self.pixel_size < max_zoom
                        {
                            self.pixel_size += 1;
                            println!("Zoom {}", self.pixel_size);
                        }
                    },
                    Event::Key(k, _, _, _) if k == Key::Num9 => {
                        if self.pixel_size > 1
                        {
                            self.pixel_size -= 1;
                            println!("Zoom {}", self.pixel_size);
                        }
                    }
                    _ => {}
                }
            }

            Gfx::clear_screen(&self);


            pipe.update();
            thread::sleep(time::Duration::from_millis(10));

            pipe.draw(&self);

            self.screen.flip();
        }

        pipe.quit();
        sdl::quit();
    }

    fn clear_screen(&self){
        let rect = sdl::Rect(0, 0, self.width as u16, self.height as u16);
        let color = sdl::video::Color::RGB(0, 0, 0);
        self.screen.fill_rect(Some(rect), color);
    }

    fn draw_square(&self, pixel: sdl::Rect, color: Color) {
        self.screen.fill_rect(Some(pixel), color);
    }

    pub fn draw_pixel(&self, x:i32, y: i32, r:u8, g:u8, b:u8) {
        let mut rect = sdl::Rect(0, 0, self.pixel_size as u16, self.pixel_size as u16);
        let color = sdl::video::Color::RGB(r, g, b);
        self.screen.fill_rect(Some(rect), color);
    }

    pub fn draw_pixel_arr(&self, pixels: &Vec<i8>, arr_width: i16, arr_height: i16) {
        let mut rect = sdl::Rect(0, 0, self.pixel_size as u16, self.pixel_size as u16);
        let color_pixel = sdl::video::Color::RGB(255, 255, 255);
        let color_background = sdl::video::Color::RGB(0, 0, 0);

        for x in 0..arr_width {
            for y in 0..arr_height {
                rect.x = x * self.pixel_size;
                rect.y = y * self.pixel_size;

                let index =  x + y * arr_width;
                let color = if pixels[index as usize] == 1 {
                    color_pixel
                } else {
                    color_background
                };

                Gfx::draw_square(self, rect, color);
            }
        }
    }
}
