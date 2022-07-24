use crate::{world::World, camera::Camera, player::Player};
use crate::tile::{TileSet, TileType};
use macroquad::prelude::*;

pub fn draw_player(cam: &Camera, player: &Player, tileset: &TileSet) {
    if !cam.is_tile_visible(player.person.entity.pos) {
        return
    }
    draw_texture(tileset.imgs[player.person.entity.tex_id].unwrap(),
    ((player.person.entity.pos.0)*40) as f32 + cam.corner.0,
    ((player.person.entity.pos.1)*40) as f32 + cam.corner.1, WHITE);
}

pub fn draw_world(camera: &Camera, world: &World, tileset: &TileSet) {
    let bounds = camera.bounds();
    if -camera.corner.0 < 0.0 || -camera.corner.1 < 0.0 || bounds.0 > world.data.len() as f32 * 40.0 || bounds.1 > world.data[0].len() as f32 * 40.0 {
        draw_rectangle(0.0,0.0,camera.res.0 as f32,camera.res.1 as f32, DARKGRAY);
    }
    let mut x = 0;
    let mut y = 0;
    while x < 20 {
        while y < 20 {
            if !camera.is_tile_visible((x,y)) {
                y+=1;
                continue
            }
            draw_texture(tileset.imgs[match world.data[x][y].tipo {
                TileType::Turf => {
                    0
                }
                _ => {
                    0
                }
            }].unwrap(),(x * 40) as f32 + camera.corner.0,
                        (y * 40) as f32 + camera.corner.1,WHITE);
            y += 1;
        }
        y = 0;
        x += 1;                                                       
    }
}
