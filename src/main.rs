/**
* plan [x]
* draw outline, [x]
*
*
*
*/

const WIDTH: usize = 30;
const HEIGHT: usize = 13;
const WALL_HORZ: u8 = 1;
const WALL_VERT: u8 = 2;
const MAP_TILES: [&str; 5] = [" ", "―", "|", "=", "@"];
fn main() {
    let mut map: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    create_wall(&mut map);

    print_map(&mut map);
}

fn print_map(map: &mut [[u8; WIDTH]; HEIGHT]) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", MAP_TILES[map[i][j] as usize]);
        }
        print!("\n");
    }
}

fn create_wall(map: &mut [[u8; WIDTH]; HEIGHT]) {
    for i in 0..WIDTH {
        // top part
        map[0][i] = WALL_HORZ;
        // bottom part
        map[HEIGHT - 1][i] = WALL_HORZ;
    }
    for i in 1..HEIGHT - 1 {
        // left part
        map[i][0] = WALL_VERT;
        // right part
        map[i][WIDTH - 1] = WALL_VERT;
    }
}
