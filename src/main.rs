use macroquad::prelude::*;

use crate::world::{World, draw_world};
use crate::tile::TileSet;
use crate::entity::draw_entities;
use crate::player::{Player, draw_player, input_player_target};
use crate::camera::{Camera, input_camera_movement};

pub mod tile;
pub mod world;
pub mod entity;
pub mod player;
pub mod person;
pub mod camera;

#[macroquad::main("Bungo")]
async fn main() {
    let tileset = TileSet::new().await;
    let world = World::new();
    let mut player = Player::new((10,10));
    let mut cam = Camera::new((600,600),(0.0,0.0));
    let entities = Vec::new();
    loop {
        clear_background(GRAY);
        draw_world(&cam, &world, &tileset);
        draw_entities(&entities, &tileset);
        draw_player(&cam, &player, &tileset);
        player.person.walk();
        input_player_target(&mut player);
        input_camera_movement(&mut cam);

        next_frame().await
    }
}
