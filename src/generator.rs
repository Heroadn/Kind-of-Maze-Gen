use rand::seq::SliceRandom;

impl Default for Node {
    fn default() -> Node {
        Node {
            elem_type: Type::Cell,
            visited: false,
        }
    }
}

//blocks separate cells from blocks like B|C|B|C|B|C|B
#[derive(Debug, Clone, Copy)]
pub enum Type {
    Cell,
    Block(bool),
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    visited: bool,
    elem_type: Type,
}

pub struct Maze {
    grid: Vec<Node>,
    stack: Vec<Coordinate>,
    //width and height of the grid, and path
    pub width: i32,
    pub height: i32,
    ///It's a representation of the grid i8
    pub path: Vec<i8>,
    ///tell where he is in the generation process
    pub cursor: Coordinate,
    //Where to start the generation
    pub start: Coordinate,
}

pub fn build(width: i32, height: i32, start: Coordinate) -> Maze {
    let mut maze = Maze {
        grid: vec![Default::default(); (width * height) as usize],
        stack: Vec::new(),
        path: vec![Default::default(); (width * height) as usize],
        width: width,
        height: height,
        cursor: start,
        start: start,
    };

    //startpoint gen
    let index = start.x + (start.y * maze.width);
    maze.grid[index as usize].visited = true;
    maze.stack.push(start);

    //put cells and blocks
    Maze::fill_grid(maze.width, maze.height, &mut maze.grid);

    return maze;
}

impl Maze {
    ///
    fn is_out_bounds(x: i32, y: i32, width: i32, height: i32) -> bool {
        return y < 0 || x < 0 || x >= width || y >= height;
    }

    ///
    fn search_unvisited_coordinates(
        cor: &Coordinate,
        width: i32,
        height: i32,
        grid: &[Node],
    ) -> Vec<Coordinate> {
        let mut choices: Vec<Coordinate> = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                let x = cor.x + i;
                let y = cor.y + j;
                let index = x + (y * width);
                //is_edge, is_middle
                if Maze::is_out_bounds(x, y, width, height) || i != 0 && j != 0 || i == j {
                    continue;
                }
                if grid[index as usize].visited == false {
                    choices.push(Coordinate { x, y });
                }
            }
        }
        return choices;
    }

    ///
    fn remove_wall(source: Node, target: &mut Node) {
        if let Type::Cell = source.elem_type {
            match target.elem_type {
                Type::Block(_is_open) => target.elem_type = Type::Block(false),
                _ => {}
            };
        }
    }

    pub fn update_path(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = x + (y * self.width);
                let (path, wall) = (1, 0);

                match self.grid[index as usize].elem_type {
                    Type::Cell => self.path[index as usize] = path,
                    Type::Block(is_open) => {
                        if is_open {
                            self.path[index as usize] = wall;
                        } else {
                            self.path[index as usize] = path;
                        }
                    }
                }
            }
        }
    }

    ///
    pub fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = x + (y * self.width);
                let (path, wall) = (" ", "#");

                if self.start.x == x && self.start.y == y {
                    print!("x");
                }

                match self.grid[index as usize].elem_type {
                    Type::Cell => print!("{}", path),
                    Type::Block(is_open) => {
                        if is_open {
                            print!("{}", wall)
                        } else {
                            print!("{}", path)
                        }
                    }
                }
            }
            println!("");
        }
    }

    ///
    fn fill_grid(width: i32, height: i32, grid: &mut [Node]) {
        for y in 0..height {
            for x in 0..width {
                let index = x + (y * width);
                grid[index as usize].elem_type = Type::Block(true);
                if x % 2 == 0 && y % 2 != 0 {
                    grid[index as usize].elem_type = Type::Cell;
                }
            }
        }
    }
    /// generate all the maze
    pub fn generate_full(&mut self) {
        while self.stack.len() > 0 {
            Maze::generate_step(self);
        }
    }
    /// just generate a portion of it
    pub fn generate_step(&mut self) {
        //search neighbours
        let choices =
            Maze::search_unvisited_coordinates(&self.cursor, self.width, self.height, &self.grid);

        //choose the branch
        let choice = choices.choose(&mut rand::thread_rng());

        //backtrack if theres no unvisited nodes
        if let None = choice {
            match self.stack.pop() {
                Some(cor) => {
                    self.cursor.x = cor.x;
                    self.cursor.y = cor.y;
                }
                _ => {}
            }
        }

        //check if not stuck
        if let Some(adj) = choice {
            let now_index = adj.x + (adj.y * self.width);
            let old_index = self.cursor.x + (self.cursor.y * self.width);

            //mark as visited
            self.grid[now_index as usize].visited = true;

            //push to the stack
            self.stack.push(Coordinate { x: adj.x, y: adj.y });

            //remove wall
            Maze::remove_wall(
                self.grid[old_index as usize],
                &mut self.grid[now_index as usize],
            );

            //changing target to neighbour
            self.cursor.x = adj.x;
            self.cursor.y = adj.y;
        }
    }
}
