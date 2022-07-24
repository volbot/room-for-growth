use macroquad::prelude::*;

use crate::world::{World, draw_world};
use crate::tile::TileSet;
use crate::entity::draw_entities;
use crate::player::{Player, draw_player, input_player_target};

pub mod tile;
pub mod world;
pub mod entity;
pub mod player;
pub mod person;

#[macroquad::main("Bungo")]
async fn main() {
    let tileset = TileSet::new().await;
    let world = World::new();
    let mut player = Player::new();
    let entities = Vec::new();
    loop {
        clear_background(RED);
        draw_world(&world, &tileset);
        draw_entities(&entities, &tileset);
        draw_player(&player, &tileset);
        player.person.walk();
        input_player_target(&mut player);

        next_frame().await
    }
}
