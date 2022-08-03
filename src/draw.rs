use crate::Game;
use crate::buildmenu::BuildMenu;
use crate::interact::Interaction;
use crate::seals::Seal;
use crate::{person::Person, world::World, camera::Camera, entity::Entity};
use crate::tileset::TileSet;
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
            draw_texture(tileset.tiles[world.data[x][y].id].unwrap(),(x * 40) as f32 * camera.scale + camera.corner.0,
            (y * 40) as f32 * camera.scale + camera.corner.1,WHITE);
            y += 1;
        }
        y = 0;
        x += 1;                                                       
    }
}

pub fn draw_popup(interact: &Interaction, game: &mut Game, tileset: &TileSet) {
    draw_texture(tileset.windows[1].unwrap(), 150.0, 600.0, WHITE);
    let split = interact.text.split("^^");
    let mut i = 640.;
    for s in split {
        draw_text_ex(s, 190., i, TextParams {
            font_size: 20,
            font: tileset.font,
            color: BLACK,
            ..Default::default()
        });
        i+=20.
    }
    if root_ui().button(Vec2::new(445.0, 690.0), interact.text_button) {
        game.window_active = None;
    }
}

pub fn draw_inventory(game: &mut Game, tileset: &TileSet) {
    let inventory = &mut game.player.inventory;
    draw_texture(tileset.windows[2].unwrap(), 150.0, 200.0, WHITE);
    draw_text_ex("Inventory", 185.0, 255.0, TextParams {
        font_size: 30,
        font: tileset.font,
        color: BLACK,
        ..Default::default()
    });
    let mut i = 0;
    let mut j = 0;
    while i < 9 {
        while j < 4 {
            let corner = (52.0 * i as f32 + 168.0, 75.0 * j as f32 + 285.0);
            if inventory.data[j][i].is_some() {
                let item = inventory.data[j][i].unwrap();
                let tooltip: String = item.name().to_string();
                let mut quantity: String = " x".to_string();
                quantity.push_str(&item.quant.to_string());
                if inventory.sel.is_some() && inventory.sel.unwrap() == (j, i) {
                    draw_texture(tileset.windows.get(3).unwrap().unwrap(), corner.0-3., corner.1-3., WHITE);
                    root_ui().push_skin(&tileset.skins[1].clone());
                    if root_ui().button(Vec2::new(395., 220.), "") {
                        inventory.pop(inventory.data[j][i].unwrap());
                    }
                    root_ui().pop_skin();
                }
                draw_texture(tileset.items.get(item.id).unwrap().unwrap(), corner.0+3., corner.1+10.0, WHITE);
                draw_text_ex(&tooltip, corner.0, corner.1+59.0, TextParams {
                    font_size: 10,
                    font: tileset.font,
                    color: BLACK,
                    ..Default::default()
                });
                draw_text_ex(&quantity, corner.0, corner.1+69.0, TextParams {
                    font_size: 10,
                    font: tileset.font,
                    color: BLACK,
                    ..Default::default()
                });
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
    if root_ui().button(Vec2::new(460.0, 215.0), "Done") {
        inventory.sel = None;
        game.window_active = None;
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse = mouse_position();
        if mouse.0 > 168.0 && mouse.0 < 636.0 && mouse.1 > 285.0 && mouse.1 < 585.0 {
            let x = (mouse.0 as usize - 168) / 52;
            let y = (mouse.1 as usize - 285) / 75;
            let dat = inventory.data[y][x];
            if dat.is_some() {
                inventory.sel = Some((y, x));
            }
        }
    }
}

pub fn draw_build_menu(menu: &BuildMenu, game: &mut Game, tileset: &TileSet) {
    draw_texture(tileset.windows[2].unwrap(), 150.0, 200.0, WHITE);
    draw_text_ex("Building", 185.0, 255.0, TextParams {
        font_size: 30,
        font: tileset.font,
        color: BLACK,
        ..Default::default()
    });
    let mut i = 0;
    let mut j = 0;
    while i < 9 {
        while j < 4 {
            let corner = (52.0 * i as f32 + 168.0, 75.0 * j as f32 + 285.0);
            if menu.data[j][i].is_some() {
                let item = menu.data[j][i].unwrap();
                let tooltip: String = item.tile.name().to_string();
                let mut quantity: String = " x".to_string();
                quantity.push_str(&item.count.to_string());
                draw_texture(tileset.tiles.get(item.tile.id).unwrap().unwrap(), corner.0+3., corner.1+10.0, WHITE);
                draw_text_ex(&tooltip, corner.0, corner.1+59.0, TextParams {
                    font_size: 10,
                    font: tileset.font,
                    color: BLACK,
                    ..Default::default()
                });
                draw_text_ex(&quantity, corner.0, corner.1+69.0, TextParams {
                    font_size: 10,
                    font: tileset.font,
                    color: BLACK,
                    ..Default::default()
                });
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
    if root_ui().button(Vec2::new(445.0, 215.0), "Done") {
        game.player.target_id = None;
        game.window_active = None;
        return
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse = mouse_position();
        if mouse.0 > 168.0 && mouse.0 < 636.0 && mouse.1 > 285.0 && mouse.1 < 585.0 {
            let x = (mouse.0 as usize - 168) / 52;
            let y = (mouse.1 as usize - 285) / 75;
            let dat = menu.data[y][x];
            if dat.is_some() {
                game.player.target_id = Some(dat.unwrap().tile.id);
                game.window_active = None;
            }
        }
    }
}

pub fn draw_shop_menu(seal: &Seal, game: &mut Game, tileset: &TileSet) {
    draw_texture(tileset.windows[1].unwrap(), 150.0, 600.0, WHITE);

    let header = TextParams {
        font_size: 14,
        font: tileset.font,
        color: BLACK,
        ..Default::default()
    };
    let info = TextParams {
        font_size: 12,
        font: tileset.font,
        color: BLACK,
        ..Default::default()
    };

    let shopitems = seal.register.unwrap().tipo().shop_items();
    let mut purchasable = [false; 3];
    let mut si_type = if shopitems[0].cost >= 0 {
        "BUY"
    } else {
        "SELL"
    };
    let mut color = match si_type {
        "BUY" => {if shopitems[0].cost > game.player.denars as i32 {RED} else {
            purchasable[0] = true;
            GREEN
        }}
        "SELL" => {if game.player.inventory.item_stack_count(shopitems[0].item) > 0 {
            purchasable[0] = true;
            GREEN} else {RED}}
        _ => {WHITE}
    };
    draw_text_ex(si_type, 195.,624.,header);
    draw_texture(tileset.windows[5].unwrap(), 165.0, 630.0, color);
    draw_texture(tileset.items[shopitems[0].item.id].unwrap(), 175.0, 640.0, WHITE);
    draw_text_ex(shopitems[0].item.name(), 230.0, 650.0, info);
    let mut quant_str: String = "x".to_string();
    quant_str.push_str(&shopitems[0].item.quant.to_string());
    draw_text_ex(&quant_str, 280., 650., info);
    let mut cost_str: String = "D$:      ".to_string();
    let cost = if si_type == "BUY" {
        shopitems[0].cost
    } else {
        -shopitems[0].cost
    };
    cost_str.push_str(&cost.to_string());
    draw_text_ex(&cost_str, 230., 670., info); 

    si_type = if shopitems[1].cost >= 0 {
        "BUY"
    } else {
        "SELL"
    };
    color = match si_type {
        "BUY" => {if shopitems[1].cost > game.player.denars as i32 {RED} else {
            purchasable[1] = true;
            GREEN
        }}
        "SELL" => {if game.player.inventory.item_stack_count(shopitems[1].item) > 0 {
            purchasable[1] = true;
            GREEN} else {RED}}
        _ => {WHITE}
    };
    draw_text_ex(si_type, 356.,624.,header);
    draw_texture(tileset.windows[5].unwrap(), 326.0, 630.0, color);
    draw_texture(tileset.items[shopitems[1].item.id].unwrap(), 336.0, 640.0, WHITE);
    draw_text_ex(shopitems[1].item.name(), 391.0, 650.0, info);
    quant_str = "x".to_string();
    quant_str.push_str(&shopitems[1].item.quant.to_string());
    draw_text_ex(&quant_str, 441., 650., info);
    cost_str = "D$:      ".to_string();
    let cost = if si_type == "BUY" {
        shopitems[1].cost
    } else {
        -shopitems[1].cost
    };
    cost_str.push_str(&cost.to_string());
    draw_text_ex(&cost_str, 391., 670., info); 

    si_type = if shopitems[2].cost >= 0 {
        "BUY"
    } else {
        "SELL"
    };
    color = match si_type {
        "BUY" => {if shopitems[2].cost > game.player.denars as i32 {RED} else {
            purchasable[2] = true;
            GREEN
        }}
        "SELL" => {if game.player.inventory.item_stack_count(shopitems[2].item) > 0 {
            purchasable[2] = true;
            GREEN} else {RED}}
        _ => {WHITE}
    };
    draw_text_ex(si_type, 517.,624.,header);
    draw_texture(tileset.windows[5].unwrap(), 487.0, 630.0, color);
    draw_texture(tileset.items[shopitems[2].item.id].unwrap(), 497.0, 640.0, WHITE);
    draw_text_ex(shopitems[2].item.name(), 552.0, 650.0, info);
    quant_str = "x".to_string();
    quant_str.push_str(&shopitems[2].item.quant.to_string());
    draw_text_ex(&quant_str, 602., 650., info);
    cost_str = "D$:      ".to_string();
    let cost = if si_type == "BUY" {
        shopitems[2].cost
    } else {
        -shopitems[2].cost
    };
    cost_str.push_str(&cost.to_string());
    draw_text_ex(&cost_str, 552., 670., info); 

    if is_mouse_button_pressed(MouseButton::Left) {
        let mp = mouse_position();
        if !(mp.0 < 165. || mp.0 > 633. || mp.1 < 630. || mp.1 > 694.) {
            if mp.0 >= 165. && mp.0 < 311. {
                if purchasable[0] {
                    let cost = shopitems[0].cost;
                    let item = shopitems[0].item;
                    game.player.denars = (game.player.denars as i32 - cost) as usize;
                    if cost >= 0 {
                        game.player.inventory.push(item);
                    } else {
                        game.player.inventory.pop(item);
                    }
                }
            } else if mp.0 >= 326. && mp.0 < 472. {
                if purchasable[1] {
                    let cost = shopitems[1].cost;
                    let item = shopitems[1].item;
                    game.player.denars = (game.player.denars as i32 - cost) as usize;
                    if cost >= 0 {
                        game.player.inventory.push(item);
                    } else {
                        game.player.inventory.pop(item);
                    }
                }
            } else if mp.0 >= 487. && mp.0 <633. {
                if purchasable[2] {
                    let cost = shopitems[2].cost;
                    let item = shopitems[2].item;
                    game.player.denars = (game.player.denars as i32 - cost) as usize;
                    if cost >= 0 {
                        game.player.inventory.push(item);
                    } else {
                        game.player.inventory.pop(item);
                    }
                }
            }
        }

    }

    if root_ui().button(Vec2::new(445.0, 700.0), "Done") {
        game.window_active = None;
    }
}
