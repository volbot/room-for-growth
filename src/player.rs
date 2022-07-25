use crate::interact::Interaction;
use crate::person::{Person, CanWalk};
use crate::tile::TileType;
use crate::world::{screen_to_tiles, World};
use crate::camera::Camera;

use macroquad::prelude::*;

pub struct Player {
    pub person: Person,
    pub target_id: Option<usize>,

}

impl Player {
    pub fn new(pos: (usize, usize)) -> Player {
        Player {
            person: Person::new(pos, 0),
            target_id: None,
        }
    }

    pub fn think(&mut self, world: &World) -> Result<Interaction, &'static str> {
        self.walk(world);
        if self.target_id.is_none() {
            return Err("no target interactable");
        }
        let person = &world.people.get(self.target_id.unwrap()).unwrap();
        if self.person.entity.distance(&person.entity) <= 1 {
            let result = person.interact;
            self.target_id = None;
            self.person.target = None;
            if result.is_none() {
                return Err("no interactable at person");
            } else {
                return Ok(result.unwrap());
            }
        }
        return Err("no target interactable");
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

pub fn input_player_target(camera: &Camera, player: &mut Player, world: &World) {
    let clicked = is_mouse_button_pressed(MouseButton::Left);
    if clicked {
        let mouse = mouse_position();
        let (x, y) = screen_to_tiles(camera.project(mouse));
        if x < 0 || y < 0 || x as usize >= world.data.len() || y as usize >= world.data[0].len() {
            return
        }
        match world.data[x as usize][y as usize].tipo {
            TileType::Grass | TileType::Boards => {
                player.person.target = Some((x as usize,y as usize))
            }
            _ => {return}
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
