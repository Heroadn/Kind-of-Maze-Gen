# Kind-of-Maze-gen

![rust](https://user-images.githubusercontent.com/36571620/128580141-60ab409f-eb81-475c-aae0-7f848fb797fb.png)

Abaixo exemplo de uso que pode ser encontrado em src/main.rs

``` sh
    const WIDTH: u32 = 40;
    const HEIGHT: u32 = 40;
    
    //where it should start generating 
    let start = Coordinate { x: 0, y: 1 };
    let mut maze: Maze = generator::build(WIDTH as i32, HEIGHT as i32, start);
    
    //generates part of the maze, can be used to see the algorithm working
    maze.generate_step();
    
    //generates all the maze
    maze.generate_full();
```

# Build

``` sh
Cargo build
```

# Run
``` sh
Cargo run
```
