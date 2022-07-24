use macroquad::prelude::*;

use crate::tile::{Tile, TileType};

pub struct World {
    pub data: [[Tile; 100]; 100]
}

impl World {
    pub fn new() -> World {
        let grass_template = Tile {
            tipo: TileType::Grass,
        };
        let wall_template = Tile {
            tipo: TileType::Wall,
        };
        let mut world = World {
            data: [[grass_template.clone(); 100]; 100],
        };
        let mut x = 30;
        let mut y = 30;
        while x < 36 {
            while y < 35 {
                world.data[x][y] = wall_template.clone();
                y += 1;
            }
            y = 30;
            x += 1;
        }
        world
    }
}

pub fn screen_to_tiles(screen_coords: (f32, f32)) -> (usize, usize) {
    return (screen_coords.0 as usize / 40, screen_coords.1 as usize / 40)
}

pub fn tiles_to_screen(tile_coords: (usize, usize)) -> (f32, f32) {
    return (tile_coords.0 as f32 * -40.0, tile_coords.1 as f32 * -40.0)
}
