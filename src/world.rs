use macroquad::prelude::*;
use macroquad::rand::gen_range;
use pathfinding::prelude::astar;

use crate::{tile::{Tile, TileType}, person::Person, quest::{get_quests, Quest}, seals::Seal, pathing::{successors_inside, heuristic}};

use noise::{Fbm, NoiseFn, Seedable};

#[derive(Clone, Debug)]
pub struct World {
    pub data: [[Tile; 100]; 100],
    pub people: Vec<Person>,
    pub seals: Vec<Seal>,
    pub quest_list: Vec<Quest>,
    noise: Fbm,
}

impl World {
    pub fn new() -> World {
        let grass_template = Tile::new(TileType::Grass.id());
        let mut world = World {
            data: [[grass_template.clone(); 100]; 100],
            people: Vec::new(),
            seals: Vec::new(),
            quest_list: get_quests(),
            noise: Fbm::new(),
        };
        world.noise = world.noise.set_seed(gen_range(0,12421242));
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
        world
    }

    pub fn is_inside(&self, pos: (usize, usize), history: &mut Vec<(usize, usize)>) -> bool {
        let tile = self.data[pos.0][pos.1];
        history.push(pos);
        match tile.tipo() {
            TileType::Boards => {
                let neighbors = self.neighbors(pos);
                let mut results: Vec<bool> = Vec::new();
                for neighbor in neighbors {
                    if !history.contains(&neighbor) {
                        results.push(self.is_inside(neighbor, history));
                    }
                }
                let mut sum: i32 = 0;
                let total = results.len() as i32;
                for result in results {
                    sum += result as i32;
                }
                return sum == total;
            }
            TileType::Planks | TileType::Seal | TileType::Register => {
                true
            }
            TileType::Grass => {
                false
            }
            _ => {
                false
            }
        }
    }

    pub fn neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut vec = Vec::new();
        if pos.0 > 0 {
            vec.push((pos.0-1, pos.1));
        }
        if pos.1 > 0 {
            vec.push((pos.0,pos.1-1));
        }
        if pos.0 < self.data.len()-1 {
            vec.push((pos.0+1,pos.1));
        }
        if pos.1 < self.data[0].len()-1 {
            vec.push((pos.0,pos.1+1));
        }
        vec
    }

    pub fn get_seal(&self, pos: (usize, usize)) -> Option<&Seal> {
        let pos_i = (pos.0 as i32, pos.1 as i32);
        for seal in &self.seals {
            let goal = (seal.pos.0 as i32, seal.pos.1 as i32);
            let result = astar( &(pos_i),
                |&(x,y)| successors_inside((x,y), self)
                .into_iter().map(|p| (p, 1)),
                |&(x, y)| heuristic((x,y), goal, self),
                |&p| p == goal);
           if result.is_none() {
               continue
           } else {
               return Some(&seal)
           }
        }
        return None
    }

    pub fn get_seal_mut(&mut self, pos: (usize, usize)) -> Option<&mut Seal> {
        let pos_i = (pos.0 as i32, pos.1 as i32);
        let im_clone = &self.clone();
        for seal in &mut self.seals {
            let goal = (seal.pos.0 as i32, seal.pos.1 as i32);
            let result = astar( &(pos_i),
                |&(x,y)| successors_inside((x,y), im_clone)
                .into_iter().map(|p| (p, 1)),
                |&(x, y)| heuristic((x,y), goal, im_clone),
                |&p| p == goal);
           if result.is_none() {
               continue
           } else {
               return Some(seal)
           }
        }
        return None
    }
}

pub fn screen_to_tiles(screen_coords: (f32, f32)) -> (isize, isize) {
    return (screen_coords.0 as isize / 40, screen_coords.1 as isize / 40)
}

pub fn tiles_to_screen(tile_coords: (usize, usize)) -> (f32, f32) {
    return (tile_coords.0 as f32 * -40.0, tile_coords.1 as f32 * -40.0)
}
