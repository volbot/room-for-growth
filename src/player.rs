use crate::person::Person;
use crate::world::screen_to_tiles;
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

pub fn input_player_target(camera: &Camera, player: &mut Player) {
    let clicked = is_mouse_button_pressed(MouseButton::Left);
    if clicked {
        let (x, y) = screen_to_tiles(camera.project(mouse_position()));
        player.person.target = Some((x,y))
    }
}
