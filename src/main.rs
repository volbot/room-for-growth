use macroquad::prelude::*;

use crate::world::{World, draw_world};
use tile::TileSet;

pub mod tile;
pub mod world;

#[macroquad::main("Bungo")]
async fn main() {
    loop {
        let tileset = TileSet::new().await;
        let world = World::new();
        clear_background(RED);
        draw_world(&world, &tileset);
        next_frame().await
    }
}

