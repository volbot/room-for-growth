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
            person: Person::new(pos, 0),
        }
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
            _ => {}
        }
    }
}
