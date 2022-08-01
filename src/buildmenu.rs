use crate::{tile::Tile, player::Player};

#[derive(Clone,Copy,Debug)]
pub struct BuildMenu {
    pub data: [[Option<BuildChoice>; 9]; 4],
}

impl BuildMenu {
    pub fn new(player: &Player) -> BuildMenu {
        let inv = player.inventory;
        let mut menu = BuildMenu {
            data: [[None; 9]; 4],
        };
        let mut i = 0;
        let mut j = 0;
        for recipe in &player.tilerecipes {
            let tile = Tile::new(recipe.id);
            let possible = (inv.item_count(tile.resources().id) as f32/tile.resources().quant as f32).floor() as usize;
            menu.data[i][j] = Some(BuildChoice::new(Tile::new(tile.id),possible as usize));
            j += 1;
            if j == 9 {
                j = 0;
                i += 1;
            }
            if i == 4 {
                return menu
            }
        }
        menu
    }
}

#[derive(Clone,Copy,Debug)]
pub struct BuildChoice {
    pub tile: Tile,
    pub count: usize,
}

impl BuildChoice {
    pub fn new(tile: Tile, count: usize) -> BuildChoice {
        BuildChoice {tile, count}
    }
}
