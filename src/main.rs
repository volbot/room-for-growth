use macroquad::prelude::*;
use world::tiles_to_screen;

use crate::world::World;
use crate::tile::TileSet;
use crate::person::Person;
use crate::player::{Player, input_player_target};
use crate::camera::{Camera, input_camera_movement};
use crate::draw::*;
use crate::interact::Interaction;

pub mod tile;
pub mod world;
pub mod entity;
pub mod player;
pub mod person;
pub mod camera;
pub mod draw;
pub mod interact;

#[macroquad::main("Bungo")]
async fn main() {
    let tileset = TileSet::new().await;
    let world = World::new();
    let mut player = Player::new((50,50));
    let mut npc = Person::new((55,55), 1);
    npc.target = Some((34,34));
    npc.interact = Some(Interaction::new(interact::InteractType::Quest));
    let mut cam = Camera::new((800,800),tiles_to_screen((40,40)));
    loop {
        clear_background(GRAY);
        draw_world(&cam, &world, &tileset);
        draw_person(&cam, &npc, &tileset); 
        draw_person(&cam, &player.person, &tileset);
        player.person.walk(&world);
        npc.walk(&world);
        input_player_target(&cam, &mut player, &world);
        input_camera_movement(&mut cam);

        next_frame().await
    }
}
