use macroquad::prelude::*;

use crate::tile::{Tile, TileType};

pub struct World {
    pub data: [[Tile; 20]; 20]
}

impl World {
    pub fn new() -> World {
        let tile_template = Tile {
            tipo: TileType::Turf,
        };
        World {
            data: [[tile_template.clone(); 20]; 20],
        }
    }
}

pub fn screen_to_tiles(screen_coords: (f32, f32)) -> (usize, usize) {
    return (screen_coords.0 as usize / 40, screen_coords.1 as usize / 40)
}
