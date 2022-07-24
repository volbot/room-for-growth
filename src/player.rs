use crate::person::Person;
use crate::tile::TileType;
use crate::world::{screen_to_tiles, World};
use crate::camera::Camera;

use macroquad::prelude::*;

pub struct Player {
    pub person: Person,
}

impl Player {
    pub fn new(pos: (usize, usize)) -> Player {
        Player {
            person: Person::new(pos, 1),
        }
    }
}

pub fn input_player_target(camera: &Camera, player: &mut Player, world: &World) {
    let clicked = is_mouse_button_pressed(MouseButton::Left);
    if clicked {
        let (x, y) = screen_to_tiles(camera.project(mouse_position()));
        match world.data[x][y].tipo {
            TileType::Grass => {
                player.person.target = Some((x,y))
            }
            _ => {}
        }
    }
}
