use crate::tile::TileSet;
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

pub fn input_player_target(player: &mut Player) {
    let clicked = is_mouse_button_pressed(MouseButton::Left);
    if clicked {
        let (x, y) = screen_to_tiles(mouse_position());
        player.person.target = Some((x,y))
    }
}

pub fn draw_player(camera: &Camera, player: &Player, tileset: &TileSet) {
    if !camera.is_tile_visible(player.person.entity.pos) {
        return
    }
    draw_texture(tileset.imgs[player.person.entity.tex_id].unwrap(),
                    ((player.person.entity.pos.0)*40) as f32 + camera.corner.0,
                    ((player.person.entity.pos.1)*40) as f32 + camera.corner.1, WHITE);
}
