#![forbid(unsafe_code)]
#![feature(static_nobundle)]
mod generator;

use generator::{Coordinate, Maze};
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn draw_square(
    pos_x: i32,
    pos_y: i32,
    size: i32,
    screen_width: i32,
    rgba: &[u8],
    frame: &mut [u8],
) {
    for x in 0..size {
        for y in 0..size {
            let i = ((x + pos_x) + (y + pos_y) * screen_width) as usize;
            let frame_i = i * 4;
            frame[frame_i] = rgba[0]; // R
            frame[frame_i + 1] = rgba[1]; // G
            frame[frame_i + 2] = rgba[2]; // B
            frame[frame_i + 3] = rgba[3]; // A
        }
    }
}

fn draw(size: i32, screen_width: i32, maze: &Maze, frame: &mut [u8]) {
    for x in 0..maze.width {
        for y in 0..maze.height {
            let index = x + y * maze.width;
            let rgba = if maze.is_open(index as usize) {
                [0x00, 0x00, 0x00, 0xff]
            } else {
                [0xff, 0xff, 0xff, 0xff]
            };

            draw_square(x * size, y * size, size, screen_width, &rgba, frame);
        }
    }
}

fn main() {
    //maze
    const WIDTH: u32 = 40;
    const HEIGHT: u32 = 40;

    let start = Coordinate { x: 0, y: 1 };
    let mut maze: Maze = generator::build(WIDTH as i32, HEIGHT as i32, start);
    let size: u32 = 8;

    //graphics
    let screen_width: u32 = 40 * size;
    let screen_height: u32 = 40 * size;

    //loop
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(screen_width as f64, screen_height as f64);
        WindowBuilder::new()
            .with_title("Maze")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(screen_width, screen_height, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            //clear screen
            draw(size as i32, screen_width as i32, &maze, pixels.get_frame());

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            maze.generate_step();

            window.request_redraw();
        }
    });
}
