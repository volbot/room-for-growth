use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::{tile::{Tile, TileType}, person::Person, quest::{get_quests, Quest}};

use noise::{Fbm, NoiseFn};

#[derive(Clone, Debug)]
pub struct World {
    pub data: [[Tile; 100]; 100],
    pub people: Vec<Person>,
    pub quest_list: Vec<Quest>,
    noise: Fbm,
}

impl World {
    pub fn new() -> World {
        let grass_template = Tile::new(TileType::Grass.id());
        let mut world = World {
            data: [[grass_template.clone(); 100]; 100],
            people: Vec::new(),
            quest_list: get_quests(),
            noise: Fbm::new(),
        };
        let mut x = 0;
        let mut y = 0;
        while x < 100 {
            while y < 100 {
                let val = 87.0*world.noise.get([(x) as f64, (y) as f64]);
                if val > 15.0 {
                    world.data[x][y].id = TileType::Brush.id();
                } else if val > 20.0 {
                    world.data[x][y].id = TileType::Wall.id();
                } else if val < -17.0 {
                    world.data[x][y].id = TileType::Water.id();
                    if x > 0 && y > 0 && x < 99 && y < 99 && val < -20.0 {
                        if gen_range(0,2)==1{
                            world.data[x+1][y].id = TileType::Water.id();
                        }
                        if gen_range(0,2)==1{
                            world.data[x][y+1].id = TileType::Water.id();
                        }
                        if gen_range(0,2)==1{
                            world.data[x-1][y].id = TileType::Water.id();
                        }
                        if gen_range(0,2)==1{
                            world.data[x][y-1].id = TileType::Water.id();
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
                    if gen_range(0,3)<2 {
                        world.data[x][y].id = TileType::Planks.id();
                    } else {
                        world.data[x][y].id = TileType::Grass.id();
                    }
                } else {
                    world.data[x][y].id = TileType::Boards.id();
                }
                if x == 34 && y == 36 {
                    world.data[x][y].id = TileType::Boards.id();
                }
                y += 1;
            }
            y = 30;
            x += 1;
        }
        x = 29;
        y = 29;
        while x < 39 {
            while y < 38 {
                if x == 38 || y == 37 || x == 29 || y == 29 {
                    world.data[x][y].id = TileType::Grass.id();
                }
                y += 1;
            }
            y = 29;
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
