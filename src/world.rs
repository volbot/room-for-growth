use macroquad::prelude::*;

use crate::{tile::{Tile, TileType}, person::Person};

use noise::{Fbm, NoiseFn};

pub struct World {
    pub data: [[Tile; 100]; 100],
    pub people: Vec<Person>,
    noise: Fbm,
}

impl World {
    pub fn new() -> World {
        let grass_template = Tile {
            tipo: TileType::Grass,
        };
        let mut world = World {
            data: [[grass_template.clone(); 100]; 100],
            people: Vec::new(),
            noise: Fbm::new(),
        };
        let mut x = 0;
        let mut y = 0;
        while x < 100 {
            while y < 100 {
                let val = 100.0*world.noise.get([(x*10) as f64, (y*10) as f64]);
                if val > 20.0 {
                    world.data[x][y].tipo = TileType::Wall;
                } else if val < -17.0 {
                    world.data[x][y].tipo = TileType::Water;
                    if x > 0 && y > 0 && x < 99 && y < 99 && val < -20.0 {
                        if ::rand::random(){
                            world.data[x+1][y].tipo = TileType::Water;
                        }
                        if ::rand::random(){
                            world.data[x][y+1].tipo = TileType::Water;
                        }
                        if ::rand::random(){
                            world.data[x-1][y].tipo = TileType::Water;
                        }
                        if ::rand::random(){
                            world.data[x][y-1].tipo = TileType::Water;
                        }
                    }
                }
                y += 1;
            }
            y = 0;
            x += 1;
        }

        x = 30;
        y = 30;
        while x < 38 {
            while y < 37 {
                if x == 37 || x == 30 || y == 36 || y == 30 {
                    world.data[x][y].tipo = TileType::Planks;
                } else {
                    world.data[x][y].tipo = TileType::Boards;
                }
                if x == 34 && y == 36 {
                    world.data[x][y].tipo = TileType::Boards;
                }
                y += 1;
            }
            y = 30;
            x += 1;
        }

        world
    }
}

pub fn screen_to_tiles(screen_coords: (f32, f32)) -> (isize, isize) {
    return (screen_coords.0 as isize / 40, screen_coords.1 as isize / 40)
}

pub fn tiles_to_screen(tile_coords: (usize, usize)) -> (f32, f32) {
    return (tile_coords.0 as f32 * -40.0, tile_coords.1 as f32 * -40.0)
}
