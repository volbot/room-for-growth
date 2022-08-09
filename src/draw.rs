use crate::Game;
use crate::buildmenu::BuildMenu;
use crate::interact::Interaction;
use crate::message::WorldMessage;
use crate::seals::Seal;
use crate::shop::ShopItem;
use crate::tile::Tile;
use crate::world::tiles_to_screen;
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
        draw_text_ex(s, 190., i, tileset.textpar[2]);
        i+=20.
    }
    if root_ui().button(Vec2::new(445.0, 690.0), interact.text_button) {
        game.window_active = None;
    }
}

pub fn draw_inventory(game: &mut Game, tileset: &TileSet) {
    let inventory = &mut game.player.inventory;
    draw_texture(tileset.windows[2].unwrap(), 150.0, 200.0, WHITE);
    draw_text_ex("Inventory", 185.0, 255.0, tileset.textpar[3]);
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
                draw_text_ex(&tooltip, corner.0, corner.1+59.0, tileset.textpar[1]);
                draw_text_ex(&quantity, corner.0, corner.1+71.0, tileset.textpar[1]);
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
    draw_text_ex("Building", 185.0, 255.0, tileset.textpar[3]);
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
                draw_text_ex(&tooltip, corner.0, corner.1+59.0, tileset.textpar[1]);
                draw_text_ex(&quantity, corner.0, corner.1+69.0, tileset.textpar[1]);
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

    let shopitems = seal.register.unwrap().tipo().shop_items();
    let mut purchasable = [false; 3];

    let mut i = 0;
    while i < shopitems.len() {
        purchasable[i] = draw_shop_item(&shopitems[i], i, game, tileset);
        i+=1;
    }

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

pub fn draw_shop_item(shopitem: &ShopItem, win_pos: usize, game: &Game, tileset: &TileSet) -> bool {
    let mut res = false;
    let origin = match win_pos {
        0 => {(165.,630.)}
        1 => {(326.,630.)}
        2 | _ => {(487.,630.)}
    };
    let si_type = if shopitem.cost >= 0 {
        "BUY"
    } else {
        "SELL"
    };
    let color = match si_type {
        "BUY" => {if shopitem.cost > game.player.denars as i32 {RED} else {
            res = true;
            GREEN
        }}
        "SELL" => {if game.player.inventory.item_stack_count(shopitem.item) > 0 {
            res = true;
            GREEN} else {RED}}
        _ => {WHITE}
    };
    draw_text_ex(si_type, origin.0 + 30.,origin.1 - 6.,tileset.textpar[0]);
    draw_texture(tileset.windows[5].unwrap(), origin.0, origin.1, color);
    draw_texture(tileset.items[shopitem.item.id].unwrap(), origin.0 + 10., origin.1 + 10., WHITE);
    draw_text_ex(shopitem.item.name(), origin.0 + 65., origin.1 + 20., tileset.textpar[0]);
    let mut quant_str = "x".to_string();
    quant_str.push_str(&shopitem.item.quant.to_string());
    draw_text_ex(&quant_str, origin.0 + 115., origin.1 + 20., tileset.textpar[1]);
    let mut cost_str = "D$:      ".to_string();
    let cost = if si_type == "BUY" {
        shopitem.cost
    } else {
        -shopitem.cost
    };
    cost_str.push_str(&cost.to_string());
    draw_text_ex(&cost_str, origin.0 + 65., origin.1 + 40., tileset.textpar[1]); 
    return res;
}

pub fn draw_main_ui(game: &Game, tileset: &TileSet) {
    let mut moneytext = "Denars: $".to_string();
    moneytext.push_str(&game.player.denars.to_string());
    draw_text_ex(&moneytext, 2., 20., tileset.textpar[4]);
}
pub fn draw_build_ui(game: &Game, tileset: &TileSet) {
    draw_text_ex("BUILD MODE", 2., 20., tileset.textpar[4]);
    let mut seltext = "Building: ".to_string();
    let mut availtext = "Remaining: ".to_string();
    let tid = game.player.target_id;
    let mut avail = -1;
    if tid.is_some() {
        let tile = Tile::new(tid.unwrap());
        seltext.push_str(tile.name());
        avail = game.player.inventory.item_stack_count(tile.resources());
        availtext.push_str(&avail.to_string());
    } else {
        seltext.push_str("None");
    }
    draw_text_ex(&seltext, 2., 40., tileset.textpar[4]);
    if avail == -1 {
        availtext.push_str("_");
    }
    draw_text_ex(&availtext, 2., 60., tileset.textpar[4]);
}

pub fn draw_world_msg(game: &Game, worldmsg: &mut Vec<WorldMessage>, tileset: &TileSet) {
    let mut i = 0;
    while i < worldmsg.len() {
        let mut msg = worldmsg.get_mut(i).unwrap();
        let mut pos = tiles_to_screen((msg.pos.0, msg.pos.1));
        pos.0 -= game.camera.corner.0;
        pos.1 -= game.camera.corner.1;
        let dims = measure_text(&msg.msg, Some(tileset.font), 12, 1.);
        draw_text_ex(&msg.msg, -pos.0 as f32-(dims.width/2.), -pos.1 as f32+20., tileset.textpar[5]);
        msg.timer += 1;
        if msg.timer >= msg.limit {
            worldmsg.remove(i);
        } else {
            i += 1;
        }
    }
}
