use crate::tile::TileSet;

use macroquad::prelude::*;

#[derive(Clone,Copy,Debug)]
pub struct Interaction {
    pub tipo: InteractType,
    pub text: &'static str,
    pub text_button: &'static str,
}

impl Interaction {
    pub fn new(tipo: InteractType, text: &'static str, text_button: &'static str) -> Interaction {
        Interaction {
            tipo, text, text_button
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
            InteractType::Waiting => {
                tileset.icons[2].unwrap()
            }
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub enum InteractType {
    Quest,
    Gift,
    Waiting,
}
