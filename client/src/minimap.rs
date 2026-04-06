use common::maze::{Maze, NORTH, SOUTH, EAST, WEST};
use common::font::set_pixel;

const WALL_COLOR:  [u8; 4] = [255, 255, 255, 255];
const FLOOR_COLOR: [u8; 4] = [40,  40,  40,  255];

pub fn draw_minimap(frame: &mut [u8], width: usize, maze: &Maze, cell_size: usize, ox: usize, oy: usize) {
    for y in 0..maze.height {
        for x in 0..maze.width {
            let px = ox + x * cell_size;
            let py = oy + y * cell_size;

            fill_rect(frame, width, px, py, cell_size, cell_size, FLOOR_COLOR);

            if !maze.has_passage(x, y, NORTH) {
                fill_rect(frame, width, px, py, cell_size, 1, WALL_COLOR);
            }
            if !maze.has_passage(x, y, SOUTH) {
                fill_rect(frame, width, px, py + cell_size - 1, cell_size, 1, WALL_COLOR);
            }
            if !maze.has_passage(x, y, WEST) {
                fill_rect(frame, width, px, py, 1, cell_size, WALL_COLOR);
            }
            if !maze.has_passage(x, y, EAST) {
                fill_rect(frame, width, px + cell_size - 1, py, 1, cell_size, WALL_COLOR);
            }
        }
    }
}

fn fill_rect(frame: &mut [u8], width: usize, x: usize, y: usize, w: usize, h: usize, color: [u8; 4]) {
    for row in y..y + h {
        for col in x..x + w {
            set_pixel(frame, width, col, row, color);
        }
    }
}