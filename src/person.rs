use crate::entity::Entity;
use crate::world::World;
use crate::tile::{TileType, is_walkable};
use macroquad::prelude::*;
use pathfinding::prelude::astar;

pub struct Person {
    pub target: Option<(usize, usize)>,
    pub entity: Entity,
    pub last_act: f64
}

impl Person {
    pub fn new(pos: (usize, usize), tex_id: usize) -> Person {
        Person {
            target: None,
            entity: Entity::new(pos, tex_id),
            last_act: get_time(),
        }
    }
    pub fn walk(&mut self, world: &World) {
        if self.target.is_none() {
            return
        }
        let time = get_time();
        if time >= self.last_act + 0.25 {
            let goal: (i32, i32) = (self.target.unwrap().0 as i32, self.target.unwrap().1 as i32);
            let curr: (i32, i32) = (self.entity.pos.0 as i32, self.entity.pos.1 as i32);
            if goal == curr {
                self.target = None;
                return
            }
            let result = astar( &(curr),
                            |&(x, y)| successors((x, y), world)
                            .into_iter().map(|p| (p, 1)),
                            |&(x, y)| heuristic((x,y), goal, world),
                            |&p| p == goal);
            if result.is_none() {
                self.target = None;
                return
            }
            let new_pos = *result.unwrap().0.get(1).unwrap();
            self.entity.pos = (new_pos.0 as usize, new_pos.1 as usize);
            self.last_act = time;
        }
    }
}

pub fn heuristic(pos: (i32, i32), goal: (i32, i32), world: &World) -> i32 {
    let tile = world.data[pos.0 as usize][pos.1 as usize];
    match tile.tipo {
        TileType::Grass => {
            ((goal.0.abs_diff(pos.0) + goal.1.abs_diff(pos.1)) / 3) as i32
        }
        _ => {
            -1
        }
    }
}

pub fn successors(pos: (i32, i32), world: &World) -> Vec<(i32, i32)> {
    let x = pos.0;
    let y = pos.1;
    let mut vec = vec![(x+1, y), (x-1, y), (x, y+1), (x, y-1)];
    let mut i = 0;
    while i < vec.len() {
        let curr = vec.get(i).unwrap();
        if curr.0 < world.data.len() as i32 && curr.1 < world.data[0].len() as i32 &&
            curr.0 >= 0 && curr.1 >= 0 &&
            is_walkable(world.data[curr.0 as usize][curr.1 as usize]){
            i+=1;
        } else {
            vec.remove(i);
        }
    }
    vec
}


