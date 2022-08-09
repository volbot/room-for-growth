use crate::message::WorldMessage;
use crate::Game;
use crate::interact::{Interaction, InteractType};
use crate::inventory::Inventory;
use crate::person::{Person, CanWalk};
use crate::recipe::TileRecipe;
use crate::reward::Reward;
use crate::seals::Seal;
use crate::shop::Register;
use crate::tile::{TileType, Tile};
use crate::world::{screen_to_tiles, World};
use crate::camera::Camera;

use macroquad::prelude::*;

pub struct Player {
    pub person: Person,
    pub target_id: Option<usize>,
    pub mode: PlayerMode,
    pub inventory: Inventory,
    pub tilerecipes: Vec<TileRecipe>,
    pub denars: usize,
}

impl Player {
    pub fn new(pos: (usize, usize), world: &mut World) -> Player {
        let p = Player {
            person: Person::new("Player", pos, 0, world),
            target_id: None,
            mode: PlayerMode::Talk,
            inventory: Inventory::new(),
            tilerecipes: vec![TileRecipe::new(TileType::Grass.id())],
            denars: 0,
        };
        //p.inventory.push(Item::new(0,200));
        p
    }

    pub fn accept_reward(&mut self, reward: &Reward) {
        for item in &reward.items {
            self.inventory.push(*item);
        }
        for recipe in &reward.tilerecipes {
            self.tilerecipes.push(*recipe);
        }
    }

    pub fn think(game: &mut Game) {
        match game.player.mode {
            PlayerMode::Talk => {
                if game.player.target_id.is_none() {
                    if game.player.person.target.is_none() {
                        game.player.walk(&game.world);
                    } else {
                        let tile = game.world.data[game.player.person.target.unwrap().0][game.player.person.target.unwrap().1];
                        if tile.id == TileType::Register.id() {
                            let dist = game.player.person.entity.distance_pos(game.player.person.target.unwrap());
                            if dist <= 1 {
                                game.player.target_id = None;
                                game.player.person.target = None;
                                game.window_active = Some(Interaction::new(InteractType::Complete, "**Shop", "", None));
                            }
                        }
                    }
                    game.player.walk(&game.world);
                    return
                }
                let person = &mut game.world.people.get_mut(game.player.target_id.unwrap()).unwrap();
                let dist = game.player.person.entity.distance(&person.entity);
                if dist <= 1 {
                    let result = person.interact;
                    game.player.target_id = None;
                    game.player.person.target = None;
                    if result.is_some() {
                        match result.unwrap().tipo { 
                            InteractType::Quest => {
                                person.advance_quest();
                            }
                            InteractType::Complete => {
                                let reward = &person.quest.clone().unwrap().reward;
                                if reward.is_some(){
                                    game.player.accept_reward(&reward.clone().unwrap());
                                }
                                let next = person.interact.unwrap().data;
                                if next.is_some() {
                                    person.set_quest(game.world.quest_list.get(next.unwrap() as usize).unwrap());
                                } else {
                                    person.quest = None;
                                    person.interact = None;
                                }
                            }
                            InteractType::Gift => {
                                //give item
                            }
                            _ => {}
                        }
                        game.window_active = result;
                    }
                }
                game.player.walk(&game.world);
            }
            PlayerMode::Mine => {
                if game.player.person.target.is_some() && game.player.person.entity.distance_pos(game.player.person.target.unwrap()) <= 1 {
                    for person in &game.world.people {
                        if person.entity.pos == game.player.person.target.unwrap() {
                            game.player.person.target = None;
                            return
                        }
                    }
                    let time = get_time();
                    if time >= game.player.person.last_act + 1.0 * game.player.person.speed {
                        game.player.person.last_act = time;
                        let target = game.player.person.target.unwrap().clone();
                        game.player.inventory.push(game.world.data[target.0][target.1].resources());
                        game.world.data[target.0][target.1].id = game.world.data[target.0][target.1].under_id();
                        game.player.person.target = None;
                    }
                } else {
                    game.player.walk(&game.world);
                }
            }
            PlayerMode::Build => {
                if game.player.person.target.is_some() && game.player.person.entity.distance_pos(game.player.person.target.unwrap()) == 1 {
                    if game.player.target_id.is_some() {
                        let tile = Tile::new(game.player.target_id.unwrap());
                        if game.player.inventory.item_count(tile.resources().id) >= tile.resources().quant as isize{
                            game.player.inventory.pop(tile.resources());
                            let target = game.player.person.target.unwrap();
                            game.world.data[target.0][target.1].id = tile.id;
                            match tile.tipo() {
                                TileType::Seal => {
                                    game.world.seals.push(Seal::new((target.0,target.1)));
                                }
                                TileType::Register => {
                                    let seal = game.world.get_seal_mut(target);
                                    if seal.is_some() {
                                        seal.unwrap().register = Some(Register::new(target, 0));
                                    }
                                }
                                _ => {}
                            }
                        } else {
                            game.player.target_id = None;
                        }
                        game.player.person.target = None;
                    }
                } else {
                    game.player.person.walk(&game.world);
                }
            }
        }
    }
}

impl CanWalk for Player {
    fn walk(&mut self, world: &World) {
        if self.target_id.is_none() {
            self.person.walk(world);
            return
        }
        let person = &world.people.get(self.target_id.unwrap()).unwrap();
        if person.entity.pos != self.person.target.unwrap() {
            self.person.target = Some(person.entity.pos);
        }
        self.person.walk(world);
    }
}

#[derive(Debug)]
pub enum PlayerMode {
    Talk,
    Mine,
    Build, 
}

pub fn input_player_target(camera: &Camera, player: &mut Player, world: &World, worldmsg: &mut Vec<WorldMessage>) {
    let clicked = is_mouse_button_pressed(MouseButton::Left);
    if clicked {
        match player.mode {
            PlayerMode::Build => {
                player.person.target = None;
                let mouse = mouse_position();
                let (x, y) = screen_to_tiles(camera.project(mouse));
                if x < 0 || y < 0 || x as usize >= world.data.len() || y as usize >= world.data[0].len() {
                    return
                }
                let mut i = 0;
                while i < world.people.len() {
                    if world.people.get(i).unwrap().entity.pos == (x as usize,y as usize) {
                        player.mode = PlayerMode::Talk;
                        input_player_target(camera, player, world, worldmsg);
                        return
                    }
                    i += 1;
                }
                if player.target_id.is_some() {
                    if world.data[x as usize][y as usize].id == Tile::new(player.target_id.unwrap()).under_id() {
                        player.person.target = Some((x as usize, y as usize));
                    } else {
                        let mut errtext = "".to_string();
                        let tile = Tile::new(player.target_id.unwrap());
                        errtext.push_str(tile.name());
                        errtext.push_str(" goes on ");
                        errtext.push_str(Tile::new(tile.under_id()).name());
                        worldmsg.push(WorldMessage::new(&errtext,(x as usize, y as usize)));
                    }
                } else {
                    player.mode = PlayerMode::Talk;
                    input_player_target(camera, player, world, worldmsg);
                }
            }
            _ => {
                player.mode = PlayerMode::Talk;
                player.person.target = None;
                player.target_id = None;
                let mouse = mouse_position();
                let (x, y) = screen_to_tiles(camera.project(mouse));
                if x < 0 || y < 0 || x as usize >= world.data.len() || y as usize >= world.data[0].len() {
                    return
                }
                let tile = world.data[x as usize][y as usize];
                if tile.is_walkable() || tile.id == TileType::Register.id() {
                    player.person.target = Some((x as usize, y as usize));
                } else {
                    player.person.target = None;
                    return
                }
                let mut i = 0;
                while i < world.people.len() {
                    if world.people.get(i).unwrap().entity.pos == (x as usize,y as usize) {
                        player.target_id = Some(i);
                        return
                    }
                    i += 1;
                }
            }
        }
    }
    let clicked = is_mouse_button_pressed(MouseButton::Right);
    if clicked {
        player.mode = PlayerMode::Mine;
        player.person.target = None;
        player.target_id = None;
        let mouse = mouse_position();
        let (x, y) = screen_to_tiles(camera.project(mouse));
        if x < 0 || y < 0 || x as usize >= world.data.len() || y as usize >= world.data[0].len() {
            return
        }
        if world.data[x as usize][y as usize].is_mineable() && (
            world.data[x as usize+1][y as usize].is_walkable() || 
            world.data[x as usize-1][y as usize].is_walkable() || 
            world.data[x as usize][y as usize+1].is_walkable() || 
            world.data[x as usize][y as usize-1].is_walkable() ) {
            player.person.target = Some((x as usize, y as usize));
        } else {
            return
        }
    }
}

pub fn input_player_keys(game: &mut Game) {
    if is_key_pressed(KeyCode::E) {
        game.window_active = Some(Interaction::new(InteractType::Complete, "**Inventory", "", None));
    }
    if is_key_pressed(KeyCode::Q) {
        game.player.mode = PlayerMode::Build;
        game.window_active = Some(Interaction::new(InteractType::Complete, "**Building", "", None));
    }
}
