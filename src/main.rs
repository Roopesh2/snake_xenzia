use std::io::{self, Write};
use std::process::Command;
use std::{thread, time::Duration};

fn clear_screen() -> io::Result<()> {
    // Clear the console screen on Unix-like systems
    if cfg!(target_os = "windows") {
        // Clear the console screen on Windows
        Command::new("cmd").args(&["/c", "cls"]).status()?;
    } else {
        // Clear the console screen on Unix-like systems
        print!("{}[2J", 27 as char);
        io::stdout().flush()?;
    };

    Ok(())
}
/**
* Working:
* takes a bunch of sprites and flattens them to single matrix
* Flattening:
* ^ Snake
* ^ stuffs
* ^ Boundary
*
*/

const WIDTH: usize = 40;
const HEIGHT: usize = 20;
const W: isize = WIDTH as isize;
const H: isize = HEIGHT as isize;


type Map2D = [[u8; WIDTH]; HEIGHT];
type Sprite = Vec<(u8, usize, usize)>;

const EMPTY_MAP: Map2D = [[0; WIDTH]; HEIGHT];

const MAP_TILES: [&str; 9] = [" ", "═", "║", "╔", "╗", "╚", "╝", "█", "▒"];
const WALL_HZ: u8 = 1;
const WALL_VT: u8 = 2;
const WALL_TL: u8 = 3;
const WALL_TR: u8 = 4;
const WALL_BL: u8 = 5;
const WALL_BR: u8 = 6;
const SNAKE_BODY: u8 = 7;
const SNAKE_HEAD: u8 = 8;


fn main() {
    let mut snake: Sprite = Vec::new();
    let mut boundary: Map2D = EMPTY_MAP;
    let head_dir: (isize, isize) = (0, 1);
    for i in 0..5 {
        snake.push((SNAKE_BODY, HEIGHT / 2, WIDTH / 2 - 2 + i));
    }
    snake.push((SNAKE_HEAD, HEIGHT / 2, WIDTH / 2 - 2 + 5));
    // init game states
    loop {
        clear_screen().unwrap();
        create_wall(&mut boundary);
        let mut snake_map = sprite_to_map(&snake);
        project(&mut boundary, &mut snake_map);

        update_snake_position(&mut snake, head_dir);

        thread::sleep(Duration::from_millis(500));
    }
}

fn update_snake_position(snake: &mut Sprite, (dy, dx): (isize, isize)) {
    let head = snake.pop().unwrap().clone();
    let tail = snake.first().unwrap().clone();
    let mut head_i = head.1 as isize + dx;
    let mut head_j = head.2 as isize + dy;
    // loop around
    if head_i >= H - 1 {
        head_i = 1;
    }
    if head_i == 0 {
        head_i = H - 1;
    }
    if head_j >= W - 1 {
        head_j = 1;
    }
    if head_j == 0 {
        head_j = W - 1;
    }
    snake.push((tail.0, head.1, head.2));
    snake.push((head.0, head_i as usize, head_j as usize));
    snake.remove(0);
}

fn sprite_to_map(sprite: &Sprite) -> Map2D {
    let mut res: Map2D = EMPTY_MAP;
    for &(map_val, i, j) in sprite.iter() {
        res[i][j] = map_val;
    }
    return res;
}

fn project(boundary: &mut Map2D, snake: &mut Map2D) {
    let mut screen: Map2D = EMPTY_MAP;
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            // check if snake collides with bounday
            if (snake[i][j] > 0) && (boundary[i][j] > 0) {
                print!("collision\n");
            } else {
                screen[i][j] = boundary[i][j] + snake[i][j];
            }
        }
    }

    print_map(&mut screen);
}
fn print_map(boundary: &mut Map2D) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", MAP_TILES[boundary[i][j] as usize]);
        }
        print!("\n");
    }
}

fn create_wall(boundary: &mut Map2D) {
    for i in 0..WIDTH {
        // top part
        boundary[0][i] = WALL_HZ;
        // bottom part
        boundary[HEIGHT - 1][i] = WALL_HZ;
    }
    for i in 1..HEIGHT - 1 {
		// left part
        boundary[i][0] = WALL_VT;
        // right part
        boundary[i][WIDTH - 1] = WALL_VT;
    }
	boundary[HEIGHT - 1][0] = WALL_BL;
	boundary[HEIGHT - 1][WIDTH - 1] = WALL_BR;
	boundary[0][0] = WALL_TL;
	boundary[0][WIDTH - 1] = WALL_TR;
}
