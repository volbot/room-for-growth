use crate::entity::Entity;
use macroquad::prelude::*;

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
            println!("{} {}", self.entity.pos.0, self.entity.pos.1);
            println!("{} {}", self.target.unwrap().0, self.target.unwrap().1);
            let xdist: isize = self.entity.pos.0 as isize - self.target.unwrap().0 as isize;
            let ydist: isize = self.entity.pos.1 as isize - self.target.unwrap().1 as isize;
            if xdist.abs() >= ydist.abs() && xdist != 0 {
                let change: isize = if xdist > 0 {-1} else {1};
                if change.is_positive() {
                    self.entity.pos.0 += change as usize;
                } else {
                    self.entity.pos.0 -= change.abs() as usize;
                }
            } else if ydist != 0 {
                let change: isize = if ydist > 0 {-1} else {1};
                if change.is_positive() {
                    self.entity.pos.1 += change as usize;
                } else {
                    self.entity.pos.1 -= change.abs() as usize;
                }
            }
            self.last_act = time;
        }
    }
}
