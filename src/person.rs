use crate::entity::Entity;
use crate::player::Player;
use crate::quest::{Quest, QuestType};
use crate::world::World;
use crate::tile::TileType;
use crate::interact::{Interaction, InteractType};
use macroquad::{prelude::*, rand};
use pathfinding::prelude::astar;
use crate::pathing::*;

pub trait CanWalk {
    fn walk(&mut self, world: &World);
}

#[derive(Clone, Debug)]
pub struct Person {
    pub target: Option<(usize, usize)>,
    pub entity: Entity,
    pub last_act: f64,
    pub interact: Option<Interaction>,
    pub quest: Option<Quest>,
    pub speed: f64,
}

impl Person {
    pub fn new(pos: (usize, usize), tex_id: usize, world: &mut World) -> Person {
        //clear land at person's position
        for n1 in world.neighbors(pos) {
            for n2 in  world.neighbors(n1) {
                world.data[n2.0][n2.1].id = TileType::Grass.id();
            }
        }

        Person {
            target: None,
            entity: Entity::new(pos, tex_id),
            last_act: get_time(),
            interact: None,
            quest: None,
            speed: 1.0,
        }
    }

    pub fn set_quest(&mut self, quest: &Quest) {
        self.quest = Some(quest.clone());
        self.interact = Some(quest.msgs[0]);
    }

    pub fn advance_quest(&mut self) {
        let mut quest = self.quest.clone().unwrap();
        quest.status += 1;
        self.interact = Some(quest.msgs[quest.status]);
        self.quest = Some(quest);
    }

    pub fn update_quest(&mut self, player: &Player, world: &World) {
        if self.quest.is_none() {
            return
        }
        match self.interact.unwrap().tipo {
            InteractType::Waiting => {
                if self.quest.clone().unwrap().is_completable(player) {
                    self.advance_quest();
                }
                match self.quest.clone().unwrap().objec.tipo {
                    QuestType::House => {
                        if self.target.is_none() {
                        for seal in &mut world.seals.clone() {
                            if seal.owner.is_some() {
                                continue
                            }
                            for n in world.neighbors(seal.pos) {
                                if world.is_inside(n, &mut Vec::new()) && world.data[n.0][n.1].is_walkable() {
                                    self.target = Some(n);
                                    seal.owner=Some(self.clone());
                                }
                            }
                        }
                        }
                        if world.is_inside(self.entity.pos, &mut Vec::new()) {
                            self.advance_quest();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn think(&mut self, world: &World) {
        let time = get_time();
        let seal = world.get_seal(self.entity.pos);
        if time >= self.last_act + 5. && rand::gen_range(0, (time - self.last_act) as i32) < (time - self.last_act - 5.) as i32 {
            let (mut x, mut y) = (self.entity.pos.0 as i32 + rand::gen_range(-3,3), self.entity.pos.1 as i32 + rand::gen_range(-3,3));
            while !world.data[x as usize][y as usize].is_walkable() {
                (x,y) = (self.entity.pos.0 as i32 + rand::gen_range(-3,3), self.entity.pos.1 as i32 + rand::gen_range(-3,3));
            }
            let mut new_seal = world.get_seal((x as usize,y as usize));
            while !world.data[x as usize][y as usize].is_walkable() || (seal.is_some() && ((new_seal.is_some() && seal.unwrap().pos != new_seal.unwrap().pos)||new_seal.is_none())) {
                (x,y) = (self.entity.pos.0 as i32 + rand::gen_range(-3,3), self.entity.pos.1 as i32 + rand::gen_range(-3,3));
                new_seal = world.get_seal((x as usize,y as usize));
            }
            self.last_act = time;
            self.target = Some((x as usize,y as usize));
        }
    }
}

impl CanWalk for Person {
    fn walk(&mut self, world: &World) {
        if self.target.is_none() {
            return
        }
        let time = get_time();
        let mult = match world.data[self.entity.pos.0][self.entity.pos.1].tipo() {
            TileType::Brush => {2.0}
            _ => {1.0}
        };
        if time >= self.last_act + 0.25 * self.speed * mult {
            let goal: (i32, i32) = (self.target.unwrap().0 as i32, self.target.unwrap().1 as i32);
            let curr: (i32, i32) = (self.entity.pos.0 as i32, self.entity.pos.1 as i32);
            if goal == curr {
                self.target = None;
                return
            }
            let result = astar( &(curr),
                            |&(x, y)| successors((x,y), goal, world)
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
