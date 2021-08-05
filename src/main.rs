mod generator;
mod graphics;

use generator::{Coordinate, Maze};
use graphics::Pipeline;

impl Pipeline for Maze {
    fn init(&mut self) {
    }
    fn update(&mut self) {
        self.generate_step();
        self.update_path();
    }
    fn draw(&mut self, gfx: &graphics::Gfx) {
        gfx.draw_pixel_arr(&self.path, self.width as i16, self.height as i16);
    }
    fn quit(&self) {
    }
}

fn main() {
    //maze
    const WIDTH: i32 = 60;
    const HEIGHT: i32 = 60;

    let start = Coordinate { x: 0, y: 1 };
    let mut maze = generator::build(WIDTH, HEIGHT, start);

    //graphics
    let (screen_width, screen_height, bpp, size) = (800 / 2, 600 / 2, 32, 4);
    let mut gfx = graphics::build(screen_width, screen_height, bpp, size);
    gfx.run(&mut maze);

    //maze.generate_full();
    //maze.print_grid();
}
