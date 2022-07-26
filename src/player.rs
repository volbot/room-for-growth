use crate::interact::{Interaction, InteractType};
use crate::inventory::Inventory;
use crate::person::{Person, CanWalk};
use crate::tile::{is_walkable, TileType};
use crate::world::{screen_to_tiles, World};
use crate::camera::Camera;

use macroquad::prelude::*;

pub struct Player {
    pub person: Person,
    pub target_id: Option<usize>,
    pub mode: PlayerMode,
    pub inventory: Inventory,
}

impl Player {
    pub fn new(pos: (usize, usize)) -> Player {
        Player {
            person: Person::new(pos, 0),
            target_id: None,
            mode: PlayerMode::Talk,
            inventory: Inventory::new(),
        }
    }

    pub fn think(&mut self, world: &mut World) -> Result<Interaction, &'static str> {
        match self.mode {
            PlayerMode::Talk => {
                if self.target_id.is_none() {
                    self.walk(world);
                    return Err("no target interactable");
                }
                let person = &mut world.people.get_mut(self.target_id.unwrap()).unwrap();
                let dist = self.person.entity.distance(&person.entity);
                if dist <= 1 {
                    let result = person.interact;
                    self.target_id = None;
                    self.person.target = None;
                    if result.is_none() {
                        return Err("person not interactable");
                    } else {
                        match result.unwrap().tipo { 
                            InteractType::Quest => {
                                person.advance_quest();
                            }
                            InteractType::Complete => {
                                let next = person.interact.unwrap().data;
                                if next.is_some() {
                                    person.set_quest(world.quest_list.get(next.unwrap() as usize).unwrap());
                                }
                            }
                            InteractType::Gift => {
                                //give item
                            }
                            _ => {}
                        }
                        return Ok(result.unwrap());
                    }
                }
                self.walk(world);
            }
            PlayerMode::Mine => {
                if self.person.target.is_some() && self.person.entity.distance_pos(self.person.target.unwrap()) <= 1 {
                    let time = get_time();
                    if time >= self.person.last_act + 1.0 * self.person.speed {
                        self.person.last_act = time;
                        self.inventory.push(world.data[self.person.target.unwrap().0][self.person.target.unwrap().1].resources());
                        world.data[self.person.target.unwrap().0][self.person.target.unwrap().1].id = TileType::Grass.id();
                        self.person.target = None;
                    }
                } else {
                    self.walk(world);
                }
            }
            PlayerMode::Build => {
                if self.person.target.is_some() && self.person.entity.distance_pos(self.person.target.unwrap()) == 1 {
                    if self.target_id.is_some() {
                        world.data[self.person.target.unwrap().0][self.person.target.unwrap().1].id = self.target_id.unwrap();
                    }
                    self.person.target = None;
                } else {
                    self.person.walk(world);
                }
            }
        }
        return Err("no target interactable");
    }
    pub fn is_minable(&self, tile: TileType) -> bool {
        match tile {
            TileType::Brush | TileType::Planks | TileType::Boards => {
                true
            }
            _ => {
                false
            }
        }
    }
}

impl CanWalk for Player {
    fn walk(&mut self, world: &World) {
        if self.target_id.is_none() {
            self.person.walk(world);
            return
        }
        let person = &world.people.get(self.target_id.unwrap()).unwrap();
        if person.entity.pos != self.person.target.unwrap() {
            self.person.target = Some(person.entity.pos);
        }
        self.person.walk(world);
    }
}

#[derive(Debug)]
pub enum PlayerMode {
    Talk,
    Mine,
    Build, 
}

pub fn input_player_target(camera: &Camera, player: &mut Player, world: &World) {
    let clicked = is_mouse_button_pressed(MouseButton::Left);
    if clicked {
        match player.mode {
            PlayerMode::Build => {
                player.person.target = None;
                let mouse = mouse_position();
                let (x, y) = screen_to_tiles(camera.project(mouse));
                if player.target_id.is_none() || x < 0 || y < 0 || x as usize >= world.data.len() || y as usize >= world.data[0].len() {
                    return
                }
                if is_walkable(world.data[x as usize][y as usize]) {
                    player.person.target = Some((x as usize, y as usize));
                }

            }
            _ => {
                player.mode = PlayerMode::Talk;
                player.person.target = None;
                player.target_id = None;
                let mouse = mouse_position();
                let (x, y) = screen_to_tiles(camera.project(mouse));
                if x < 0 || y < 0 || x as usize >= world.data.len() || y as usize >= world.data[0].len() {
                    return
                }
                if is_walkable(world.data[x as usize][y as usize]) {
                    player.person.target = Some((x as usize, y as usize));
                } else {
                    player.person.target = None;
                    return
                }
                let mut i = 0;
                while i < world.people.len() {
                    if world.people.get(i).unwrap().entity.pos == (x as usize,y as usize) {
                        player.target_id = Some(i);
                        return
                    }
                    i += 1;
                }
            }
        }
    }
    let clicked = is_mouse_button_pressed(MouseButton::Right);
    if clicked {
        player.mode = PlayerMode::Mine;
        player.person.target = None;
        player.target_id = None;
        let mouse = mouse_position();
        let (x, y) = screen_to_tiles(camera.project(mouse));
        if x < 0 || y < 0 || x as usize >= world.data.len() || y as usize >= world.data[0].len() {
            return
        }
        if player.is_minable(world.data[x as usize][y as usize].tipo()) && (
            is_walkable(world.data[x as usize+1][y as usize]) || 
            is_walkable(world.data[x as usize-1][y as usize]) || 
            is_walkable(world.data[x as usize][y as usize+1]) || 
            is_walkable(world.data[x as usize][y as usize-1]) ) {
            player.person.target = Some((x as usize, y as usize));
        } else {
            return
        }
    }
}

pub fn input_player_keys(player: &mut Player) -> Option<Interaction> {
    if is_key_pressed(KeyCode::E) {
        return Some(Interaction::new(InteractType::Complete, "**Inventory", "", None));
    }
    if is_key_pressed(KeyCode::Q) {
        player.mode = PlayerMode::Build;
        return Some(Interaction::new(InteractType::Complete, "**Building", "", None));
    }
    return None
}
