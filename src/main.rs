use buildmenu::BuildMenu;
use interact::InteractType;
use macroquad::prelude::*;
use macroquad::ui::*;
use player::input_player_keys;
use world::tiles_to_screen;

use crate::world::World;
use crate::tile::TileSet;
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

#[macroquad::main("Bungo")]
async fn main() {
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
    root_ui().push_skin(&skin);
    //create necessary variables
    let mut world = World::new();
    let mut player = Player::new((50,50));
    //TEMP---------------
    let mut npc = Person::new((55,55), 1);
    npc.target = Some((34,34));
    npc.set_quest(&world.quest_list.get(0).unwrap());
    world.people.push(npc);
    //END TEMP-----------
    //set aside storage for an interaction to display
    let mut window_active: Option<Interaction> = None;
    //create a camera struct to store data
    let mut cam = Camera::new((800,800),tiles_to_screen((40,40)));
    loop {
        clear_background(GRAY);
        draw_world(&cam, &world, &tileset);
        let world_copy = &world.clone();
        for person in &mut world.people {
            draw_person(&cam, &person, &tileset);
            person.walk(world_copy);
            if person.quest.is_some() {
                match person.interact.unwrap().tipo {
                    InteractType::Waiting => {
                        if person.quest.unwrap().is_completable(world_copy, &player) {
                            person.advance_quest();
                        }
                    }
                    _ => {}
                }
            }
        }
        draw_person(&cam, &player.person, &tileset);
        match player.think(&mut world) {
            Ok(interact) => { window_active = Some(interact) }
            Err(_s) => {}
        }
        if window_active.is_some() {
            if window_active.unwrap().text == "**Inventory" {
                 match draw_inventory(&player.inventory, &tileset) {
                     Ok(_i) => {}
                     Err(_s) => {window_active = None}
                 }
            } else if window_active.unwrap().text == "**Building" {
                match draw_build_menu(&BuildMenu::new(&player.inventory), &tileset) {
                    Ok(i) => {
                        player.target_id = Some(i.tile.id);
                        if i.count != 1000000 {
                            window_active = None;
                        }
                    }
                    Err(_s) => {
                        player.target_id = None;
                        window_active = None;
                    }
                }

            } else {
                match draw_popup(&window_active.unwrap(), &tileset) {
                    Ok(interact) => { window_active = Some(interact); }
                    Err(_s) => {
                        window_active = None;
                    }
                }
            }
        } else {
            input_camera_movement(&mut cam);
            input_player_target(&cam, &mut player, &world);
            window_active = input_player_keys(&mut player);
        }

        next_frame().await
    }
}
