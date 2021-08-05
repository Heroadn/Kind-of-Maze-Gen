# Kind-of-Maze-gen
abaixo exemplo de uso que pode ser encontrado em src/main.rs

``` sh
    const WIDTH: i32 = 60;
    const HEIGHT: i32 = 60;

    let start = Coordinate { x: 0, y: 1 };
    let mut maze = generator::build(WIDTH, HEIGHT, start);

    //graphics
    let (screen_width, screen_height, bpp, size) = (800 / 2, 600 / 2, 32, 4);
    let mut gfx = graphics::build(screen_width, screen_height, bpp, size);
    gfx.run(&mut maze);
```
