use macroquad::prelude::*;
use macroquad::ui::*;
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
    npc.interact = Some(Interaction::new(interact::InteractType::Quest, "Buzz off, pickloid.", "Ok", Some(0)));
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
        }
        draw_person(&cam, &player.person, &tileset);
        match player.think(&world) {
            Ok(interact) => { window_active = Some(interact) }
            Err(s) => {}
        }
        if window_active.is_some() {
            match draw_popup(&window_active.unwrap(), &tileset) {
                Ok(interact) => { window_active = Some(interact); }
                Err(_s) => { 
                    window_active = None;
                }
            }
        } else {
            input_player_target(&cam, &mut player, &world);
            input_camera_movement(&mut cam);
        }


        next_frame().await
    }
}
