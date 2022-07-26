use crate::tileset::TileSet;

use macroquad::prelude::*;

#[derive(Clone,Debug)]
pub struct Interaction {
    pub tipo: InteractType,
    pub name: String,
    pub text: String,
    pub text_button: String,
    pub data: Option<i32>,
}

impl Interaction {
    pub fn new(tipo: InteractType, text: &str, text_button: &str, data: Option<i32>) -> Interaction {
        Interaction {
            tipo, 
            name: "".to_string(),
            text: text.to_string(),
            text_button: text_button.to_string(), 
            data
        }
    }

    pub fn tex(&self, tileset: &TileSet) -> Texture2D {
        match self.tipo {
            InteractType::Quest => {
                tileset.icons[0].unwrap()
            }
            InteractType::Gift | InteractType::Complete => {
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
    Complete,
}
