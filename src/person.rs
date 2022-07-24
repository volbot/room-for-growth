use crate::entity::Entity;
use macroquad::prelude::*;
use pathfinding::prelude::astar;

pub struct Person {
    pub target: Option<(usize, usize)>,
    pub entity: Entity,
    pub last_act: f64
}

impl Person {
    pub fn new() -> Person {
        Person {
            target: None,
            entity: Entity::new(),
            last_act: get_time(),
        }
    }
    pub fn walk(&mut self) {
        if self.target.is_none() {
            return
        }
        let time = get_time();
        if time >= self.last_act + 0.3 {
            let goal: (i32, i32) = (self.target.unwrap().0 as i32, self.target.unwrap().1 as i32);
            let curr: (i32, i32) = (self.entity.pos.0 as i32, self.entity.pos.1 as i32);
            if goal == curr {
                self.target = None;
                return
            }
            let result = astar( &(curr),
                            |&(x, y)| vec! [(x+1, y), (x-1, y), (x, y+1), (x, y-1)]
                            .into_iter().map(|p| (p, 1)),
                            |&(x, y)| (goal.0.abs_diff(x) + goal.1.abs_diff(y)) / 3,
                            |&p| p == goal);
            let new_pos = *result.unwrap().0.get(1).unwrap();
            self.entity.pos = (new_pos.0 as usize, new_pos.1 as usize);
            self.last_act = time;
        }
    }
}
