use macroquad::prelude::*;

use crate::{tile::{Tile, TileType, TileSet}, camera::Camera};

pub struct World {
    data: [[Tile; 20]; 20]
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

pub fn draw_world(camera: &Camera, world: &World, tileset: &TileSet) {
    let mut x = 0;
    let mut y = 0;
    while x < 20 {
        while y < 20 {
            if !camera.is_tile_visible((x,y)) {
                y+=1;
                continue
            }
            draw_texture(tileset.imgs[match world.data[x][y].tipo {
                TileType::Turf => {
                    0
                }
                _ => {
                    0
                }
            }].unwrap(),(x * 40) as f32 + camera.corner.0,
                        (y * 40) as f32 + camera.corner.1,WHITE);
            y += 1;
        }
        y = 0;
        x += 1;
    }
}

pub fn screen_to_tiles(screen_coords: (f32, f32)) -> (usize, usize) {
    return (screen_coords.0 as usize / 40, screen_coords.1 as usize / 40)

}
