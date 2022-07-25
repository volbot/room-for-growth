use crate::person::{Person, CanWalk};
use crate::tile::{TileType, Tile};
use crate::world::{screen_to_tiles, World};
use crate::camera::Camera;

use macroquad::prelude::*;

pub struct Player {
    pub person: Person,
    pub interact_target: Option<Person>,
}

impl Player {
    pub fn new(pos: (usize, usize)) -> Player {
        Player {
            person: Person::new(pos, 0),
            interact_target: None,
        }
    }
}

impl CanWalk for Player {
    fn walk(&mut self, world: &[[Tile; 100]; 100]) {
        if self.interact_target.is_none() {
            self.person.walk(world);
            return
        }
        if self.person.entity.distance(&self.interact_target.unwrap().entity) <= 1 {
            self.interact_target = None;
            self.person.target = None;
        } else if self.interact_target.unwrap().entity.pos != self.person.target.unwrap() {
            self.person.target = Some(self.interact_target.unwrap().entity.pos);
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
        for person in &world.people {
            if person.entity.pos == (x as usize,y as usize) {
                player.interact_target = Some(*person);
            }
        }
    }
}
