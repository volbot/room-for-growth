use crate::tile::TileSet;

use macroquad::prelude::*;

#[derive(Clone,Copy,Debug)]
pub struct Interaction {
    pub tipo: InteractType,
}

impl Interaction {
    pub fn new(tipo: InteractType) -> Interaction {
        Interaction {
            tipo,
        }
    }

    pub fn tex(&self, tileset: &TileSet) -> Texture2D {
        match self.tipo {
            InteractType::Quest => {
                tileset.icons[0].unwrap()
            }
            InteractType::Gift => {
                tileset.icons[1].unwrap()
            }
            InteractType::Declare => {
                tileset.icons[2].unwrap()
            }
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub enum InteractType {
    Quest,
    Gift,
    Declare,
}
