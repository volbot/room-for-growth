use crate::tile::TileSet;
use crate::person::Person;
use crate::world::screen_to_tiles;

use macroquad::prelude::*;

pub struct Player {
    pub tex_id: usize,
    pub person: Person,
}

impl Player {
    pub fn new() -> Player {
        Player {
            tex_id: 1,
            person: Person::new(),
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

pub fn draw_player(player: &Player, tileset: &TileSet) {
    draw_texture(tileset.imgs[player.tex_id].unwrap(),
                    ((player.person.entity.pos.0)*40) as f32,
                    ((player.person.entity.pos.1)*40) as f32, WHITE);
}
