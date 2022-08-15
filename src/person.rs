use crate::entity::Entity;
use crate::quest::{Quest, QuestType};
use crate::seals::SealType;
use crate::world::World;
use crate::tile::TileType;
use crate::interact::{Interaction, InteractType};
use macroquad::{prelude::*, rand};
use pathfinding::prelude::astar;
use crate::{pathing::*, Game};

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
    pub fn new(name: &str, pos: (usize, usize), tex_id: usize, world: &mut World) -> Person {
        //clear land at person's position
        for n1 in world.neighbors(pos) {
            for n2 in  world.neighbors(n1) {
                world.data[n2.0][n2.1].id = TileType::Grass.id();
            }
        }

        Person {
            target: None,
            entity: Entity::new(name, pos, tex_id),
            last_act: get_time(),
            interact: None,
            quest: None,
            speed: 1.0,
        }
    }

    pub fn set_quest(&mut self, quest: &Quest) {
        self.quest = Some(quest.clone());
        let mut inter = quest.msgs[0].clone();
        inter.name = self.entity.name.clone();
        self.interact = Some(inter);
    }

    pub fn advance_quest(&mut self) {
        let mut quest = self.quest.clone().unwrap();
        quest.status += 1;
        let mut inter = quest.msgs[quest.status].clone();
        inter.name = self.entity.name.clone();
        self.interact = Some(inter);
        self.quest = Some(quest);
    }

    pub fn think(&mut self, world: &World) {
        let time = get_time();
        if self.target.is_some() {
            return
        }
        let mut in_seal: Option<(usize, usize)> = None;
        let seal = world.get_seal(self.entity.pos);
        if seal.is_some() {
            in_seal = Some(seal.clone().unwrap().pos);
            let owner = seal.unwrap().owner.clone();
            if owner.is_some() && owner.unwrap().entity.name == self.entity.name {
                if seal.unwrap().register.is_some() {
                    let mp = seal.unwrap().merchant_pos().unwrap();
                    if world.data[mp.0][mp.1].is_walkable() && self.entity.pos != mp {
                        self.target = Some(mp);
                    }
                    return
                }
            }
        }
        if self.quest.is_some() {
            return
        }
        if time >= self.last_act + 5. && rand::gen_range(0, (time - self.last_act) as i32) < (time - self.last_act - 5.) as i32 {
            let (mut x, mut y) = (self.entity.pos.0 as i32 + rand::gen_range(-3,3), self.entity.pos.1 as i32 + rand::gen_range(-3,3));
            while !world.data[x as usize][y as usize].is_walkable() || 
                if in_seal.is_some() {
                    let out_seal = world.get_seal((x as usize, y as usize));
                    if out_seal.is_some() {
                        out_seal.unwrap().pos != in_seal.unwrap()
                    } else {
                        true
                    }
                } else {
                    false
            }{
                (x,y) = (self.entity.pos.0 as i32 + rand::gen_range(-3,3), self.entity.pos.1 as i32 + rand::gen_range(-3,3));
            }
            self.last_act = time;
            self.target = Some((x as usize,y as usize));
        }
    }

    pub fn update_quests(game: &mut Game) {
        let world_copy = &game.world.clone();
        for person in &mut game.world.people {
            if person.quest.is_none() {
                continue
            }
            match person.interact.clone().unwrap().tipo {
                InteractType::Waiting => {
                    if person.quest.clone().unwrap().is_completable(&game.player) {
                        person.advance_quest();
                    }
                    match person.quest.clone().unwrap().objec.tipo {
                        QuestType::House | QuestType::HouseS => {
                            if world_copy.is_inside(person.entity.pos, &mut Vec::new()) {
                                let seal = world_copy.get_seal(person.entity.pos);
                                let obj_type = match person.quest.clone().unwrap().objec.tipo {
                                     QuestType::House => {SealType::House},
                                     QuestType::HouseS => {SealType::Shop},
                                     _ => {SealType::House}
                                };
                                if seal.is_some() && seal.unwrap().tipo == obj_type && 
                                    seal.unwrap().owner.is_some() && seal.unwrap().owner.clone().unwrap().entity.name == person.entity.name {
                                    person.advance_quest();
                                }
                            }
                        }
                        QuestType::Assign => {
                            if person.target.is_none() {
                                for seal in &mut game.world.seals {
                                    if seal.owner.is_some() &&
                                        seal.owner.clone().unwrap().entity.name == person.entity.name
                                            && seal.register.is_some() && person.entity.pos == seal.merchant_pos().unwrap() {
                                                person.advance_quest();
                                            }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
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
