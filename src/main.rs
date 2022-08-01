use buildmenu::BuildMenu;
use macroquad::prelude::*;
use macroquad::ui::*;
use macroquad::rand::srand;

use crate::storyhooks::start_main_story;
use crate::world::{World, tiles_to_screen};
use crate::tileset::TileSet;
use crate::person::CanWalk;
use crate::player::{Player, input_player_target, input_player_keys};
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
pub mod seals;
pub mod storyhooks;
pub mod shop;
pub mod reward;
pub mod recipe;

pub struct Game {
    pub world: World,
    pub player: Player,
    pub camera: Camera,
    pub window_active: Option<Interaction>,
}

#[macroquad::main("Bungo")]
async fn main() {
    srand(macroquad::miniquad::date::now() as u64); //seed randomness with current date/time
    let tileset = TileSet::new().await; //generate tileset and wait for it to complete
    let mut w = World::new();           //create world
    let p = Player::new((50, 50), &mut w); //create player at supplies coordinates
    let mut game = Game {               //save gamedata to a single object
        world: w,
        player: p,
        camera: Camera::new((800,800),tiles_to_screen((40,40))), //create camera, 800x800, at tile 40,40
        window_active: None
    };
    root_ui().push_skin(&tileset.skins[0]); //set default skin
    start_main_story(&mut game.world); //spawn things for main story
    loop {
        clear_background(GRAY);
        draw_world(&game.camera, &game.world, &tileset); //draw gameworld
        let world_copy = &game.world.clone(); //create clone of world for non-mutable references
        for person in &mut game.world.people { //loop through people in the world
            draw_person(&game.camera, &person, &tileset); //draw them
            person.walk(world_copy);                    //move them
            person.update_quest(&game.player, world_copy); //update their quest info
        }
        draw_person(&game.camera, &game.player.person, &tileset); //draw the player
        match game.player.think(&mut game.world) {                  //do player data
            Ok(interact) => { game.window_active = Some(interact) } //set active window if player
                                                                    //    returns an interaction
            Err(_s) => {}
        }
        if game.window_active.is_some() {                       //if game window exists,
            if game.window_active.unwrap().text == "**Inventory" { //check data for special cases
                draw_inventory(&mut game, &tileset);                        //draw inventory
            } else if game.window_active.unwrap().text == "**Building" {
                draw_build_menu(&BuildMenu::new(&game.player), &mut game, &tileset); //draw 
                                                                                               //   buildmenu
            } else {
                draw_popup(&game.window_active.unwrap(), &mut game, &tileset); //if no special
                                                                               //   case, draw the
                                                                               //   interaction as-is
            }
        } else { //if no window
            input_camera_movement(&mut game.camera);            //check camera movement
            input_player_target(&game.camera, &mut game.player, &game.world); //check player
                                                                              //   movement
            game.window_active = input_player_keys(&mut game.player);       //check player keys
        }

        next_frame().await //wait for next frame
    }
}
