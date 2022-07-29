use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use buildmenu::BuildMenu;
use getrandom::getrandom;
use macroquad::prelude::*;
use macroquad::ui::*;
use macroquad::rand::srand;
use player::input_player_keys;
use world::tiles_to_screen;

use crate::world::World;
use crate::tileset::TileSet;
use crate::person::{Person, CanWalk};
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
pub mod pathing;
pub mod quest;
pub mod inventory;
pub mod item;
pub mod buildmenu;
pub mod tileset;

pub struct Game {
    pub world: World,
    pub player: Player,
    pub camera: Camera,
    pub window_active: Option<Interaction>,
}

#[macroquad::main("Bungo")]
async fn main() {
    let mut hash = DefaultHasher::new();
    let mut dat: [u8;1] = [0];
    if getrandom(&mut dat).is_ok() {
        dat.hash(&mut hash);
        srand(hash.finish());
    }
    //load font as bytes for buttons
    let font_bytes = include_bytes!("../assets/fonts/JMH Cthulhumbus Arcade.otf");
    //generate the tileset from the assets folder
    let tileset = TileSet::new().await;
    //create a button style for menus
    let button_style = root_ui().style_builder()
        .background(tileset.windows[0].unwrap().get_texture_data())
        .background_margin(RectOffset::new(67., 67., 18., 18.))
        .font(font_bytes).unwrap()
        .build();
    let skin = Skin {
        button_style,
        ..root_ui().default_skin()
    };
    let mut w = World::new();
    let p = Player::new((50, 50), &mut w);
    let mut game = Game {
        world: w,
        player: p,
        camera: Camera::new((800,800),tiles_to_screen((40,40))),
        window_active: None
    };
    root_ui().push_skin(&skin);
    //TEMP---------------
    let mut npc = Person::new((55,55), 1);
    npc.target = Some((34,34));
    npc.set_quest(&game.world.quest_list.get(0).unwrap());
    game.world.people.push(npc);
    //END TEMP-----------
    loop {
        clear_background(GRAY);
        draw_world(&game.camera, &game.world, &tileset);
        let world_copy = &game.world.clone();
        for person in &mut game.world.people {
            draw_person(&game.camera, &person, &tileset);
            person.walk(world_copy);
            person.update_quest(&game.player, world_copy);
        }
        draw_person(&game.camera, &game.player.person, &tileset);
        match game.player.think(&mut game.world) {
            Ok(interact) => { game.window_active = Some(interact) }
            Err(_s) => {}
        }
        if game.window_active.is_some() {
            if game.window_active.unwrap().text == "**Inventory" {
                draw_inventory(&mut game, &tileset);
            } else if game.window_active.unwrap().text == "**Building" {
                draw_build_menu(&BuildMenu::new(&game.player.inventory), &mut game, &tileset);
            } else {
                draw_popup(&game.window_active.unwrap(), &mut game, &tileset); 
            }
        } else {
            input_camera_movement(&mut game.camera);
            input_player_target(&game.camera, &mut game.player, &game.world);
            game.window_active = input_player_keys(&mut game.player);
        }

        next_frame().await
    }
}
