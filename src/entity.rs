use macroquad::prelude::*;

use crate::tile::TileSet;

pub struct Entity {
    pub tipo: EntityType,
    pub pos: (usize, usize),
    pub tex_id: usize,
}

impl Entity {
    pub fn new(pos: (usize, usize), tex_id: usize) -> Entity {
        Entity {
            tipo: EntityType::Player,
            pos, tex_id,
        }
    }
}

pub enum EntityType {
    Player,
}

pub fn draw_entities(entities: &Vec<Entity>, tileset: &TileSet) {
    for entity in entities{
        draw_texture(tileset.imgs[match entity.tipo {
            EntityType::Player => {
                1
            }
            _ => {
                1
            }
        }].unwrap(),((entity.pos.0 - 1) * 40) as f32,((entity.pos.1-1) * 40) as f32,WHITE);
    }
}
