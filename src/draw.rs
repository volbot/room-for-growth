use crate::interact::Interaction;
use crate::inventory::Inventory;
use crate::{person::Person, world::World, camera::Camera, entity::Entity};
use crate::tile::{TileSet, TileType};
use macroquad::prelude::*;
use macroquad::ui::*;

pub fn draw_entity(cam: &Camera, entity: &Entity, tileset: &TileSet) {
    if !cam.is_tile_visible(entity.pos) {
        return
    }
    draw_texture(tileset.people[entity.tex_id].unwrap(),
    ((entity.pos.0)*40) as f32 * cam.scale + cam.corner.0,
    ((entity.pos.1)*40) as f32 * cam.scale + cam.corner.1, WHITE);
}

pub fn draw_person(cam: &Camera, person: &Person, tileset: &TileSet) {
    if !cam.is_tile_visible(person.entity.pos) {
        return
    }
    draw_entity(cam, &person.entity, tileset);
    if person.interact.is_some() {
        draw_texture(
            person.interact.unwrap().tex(tileset), 
            ((person.entity.pos.0)*40) as f32 * cam.scale + cam.corner.0, 
            ((person.entity.pos.1)*40-40) as f32 * cam.scale + cam.corner.1, 
            WHITE);
    }
}

pub fn draw_world(camera: &Camera, world: &World, tileset: &TileSet) {
    let bounds = camera.bounds();
    if -camera.corner.0 < 0.0 || -camera.corner.1 < 0.0 || bounds.0 > world.data.len() as f32 * 40.0 || bounds.1 > world.data[0].len() as f32 * 40.0 {
        draw_rectangle(0.0,0.0,camera.res.0 as f32,camera.res.1 as f32, DARKGRAY);
    }
    let mut x = 0;
    let mut y = 0;
    while x < world.data.len() {
        while y < world.data[0].len() {
            if !camera.is_tile_visible((x,y)) {
                y+=1;
                continue
            }
            draw_texture(tileset.tiles[match world.data[x][y].tipo {
                TileType::Grass => {
                    0
                }
                TileType::Wall => {
                    1
                }
                TileType::Water => {
                    2
                }
                TileType::Planks => {
                    3
                }
                TileType::Boards => {
                    4
                }
                TileType::Brush => {
                    5
                }
            }].unwrap(),(x * 40) as f32 * camera.scale + camera.corner.0,
                        (y * 40) as f32 * camera.scale + camera.corner.1,WHITE);
            y += 1;
        }
        y = 0;
        x += 1;                                                       
    }
}

pub fn draw_popup(interact: &Interaction, tileset: &TileSet) -> Result<Interaction, &'static str> {
    draw_texture(tileset.windows[1].unwrap(), 150.0, 600.0, WHITE);
    let split = interact.text.split("^^");
    let mut vec = Vec::new();
    for s in split {
        vec.push(s);
    }
    draw_text_ex(vec.get(0).unwrap(), 190.0, 640.0, TextParams {
        font_size: 20,
        font: tileset.font,
        color: BLACK,
        ..Default::default()
    });
    draw_text_ex(vec.get(1).unwrap(), 190.0, 663.0, TextParams {
        font_size: 20,
        font: tileset.font,
        color: BLACK,
        ..Default::default()
    });
    if root_ui().button(Vec2::new(445.0, 690.0), interact.text_button) {
        return Err("window closed")
    }
    return Ok(*interact)
}

pub fn draw_inventory(inventory: &Inventory, tileset: &TileSet) -> Result<Inventory, &'static str> {
    draw_texture(tileset.windows[2].unwrap(), 150.0, 200.0, WHITE);
    draw_text_ex("Inventory", 185.0, 255.0, TextParams {
        font_size: 30,
        font: tileset.font,
        color: BLACK,
        ..Default::default()
    });
    if root_ui().button(Vec2::new(445.0, 215.0), "Done") {
        return Err("window closed")
    }
    return Ok(*inventory)
}
