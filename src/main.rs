use buildmenu::BuildMenu;
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use macroquad::ui::*;
use macroquad::rand::srand;
use message::WorldMessage;
use player::PlayerMode;

use crate::storyhooks::start_main_story;
use crate::world::{World, tiles_to_screen};
use crate::tileset::TileSet;
use crate::person::CanWalk;
use crate::player::{Player, input_player_target, input_player_keys};
use crate::camera::{Camera, input_camera_movement};
use crate::draw::*;
use crate::interact::Interaction;
use crate::person::Person;

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
pub mod message;

pub struct Game {
    pub world: World,
    pub player: Player,
    pub camera: Camera,
    pub window_active: Option<Interaction>,
    pub mine_state: i32,
    pub mine_time: f64,
}

#[macroquad::main("Bungo")]
async fn main() {
    clear_background(BLACK);
    let tex = load_texture("assets/ui/loadscreen.png").await.unwrap(); 
    draw_texture(tex,0.,0.,WHITE);
    next_frame().await;
    srand(macroquad::miniquad::date::now() as u64); //seed randomness with current date/time
    let tileset = TileSet::new().await; //generate tileset and wait for it to complete
    let mut w = World::new();           //create world
    let p = Player::new((50, 50), &mut w); //create player at supplies coordinates
    let mut game = Game {               //save gamedata to a single object
        world: w,
        player: p,
        camera: Camera::new((800,800),tiles_to_screen((40,40))), //create camera, 800x800, at tile 40,40
        window_active: None,
        mine_state: -1,
        mine_time: get_time(),
    };
    let mut worldmsg: Vec<WorldMessage> = Vec::new();
    root_ui().push_skin(&tileset.skins[0]); //set default skin
    start_main_story(&mut game.world); //spawn things for main story
    loop {
        clear_background(GRAY);
        draw_world(&game.camera, &game.world, &tileset); //draw gameworld
        let world_copy = &game.world.clone(); //create clone of world for non-mutable references
        for person in &mut game.world.people { //loop through people in the world
            draw_person(&game.camera, &person, &tileset); //draw them
            person.think(world_copy);
            person.walk(world_copy);                    //move them
        }
        Person::update_quests(&mut game);
        draw_person(&game.camera, &game.player.person, &tileset); //draw the player
        Player::think(&mut game);                  //do player data
        if game.window_active.is_some() {                       //if game window exists,
            if game.window_active.as_ref().unwrap().text == "**Inventory" { //check data for special cases
                draw_inventory(&mut game, &tileset);                        //draw inventory
            } else if game.window_active.as_ref().unwrap().text == "**Building" {
                draw_build_menu(&BuildMenu::new(&game.player), &mut game, &tileset); //draw buildmenu
            } else if game.window_active.as_ref().unwrap().text == "**Shop" {
                let seal = world_copy.get_seal(game.player.person.entity.pos); 
                if seal.is_some() && seal.unwrap().register.is_some() {
                    draw_shop_menu(&seal.unwrap(), &mut game, &tileset);
                } else {
                    game.window_active = None;
                }
            } else {
                draw_popup(&game.window_active.clone().unwrap(), &mut game, &tileset); //if no special
                                                                               //   case, draw the
                                                                               //   interaction as-is
            }
        } else { //if no window
            input_camera_movement(&mut game.camera);            //check camera movement
            input_player_target(&mut game, &mut worldmsg); //check player movement
            input_player_keys(&mut game);       //check player keys
            draw_world_msg(&game, &mut worldmsg, &tileset);
            draw_loc_indic(&game, &tileset);

            if game.mine_state >= 0 && game.player.person.target.is_some() {
                let mut t = tiles_to_screen(game.player.person.target.unwrap());
                t.0 -= game.camera.corner.0;
                t.1 -= game.camera.corner.1;
                draw_texture(tileset.mine[game.mine_state as usize].unwrap(), -t.0 as f32, -t.1 as f32, WHITE);
                let time = get_time();
                if time >= game.mine_time + 0.3  {
                    game.mine_state = gen_range(0, 3);
                    game.mine_time = time;
                }
            }
        }

        match game.player.mode {
            PlayerMode::Build => {
                draw_build_ui(&game, &tileset);
            }
            PlayerMode::Assign => {
                draw_assign_ui(&game, &tileset);
            }
            _ => {
                draw_main_ui(&game, &tileset);
            }
        }

        next_frame().await //wait for next frame
    }
}
