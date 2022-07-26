use crate::{inventory::Inventory, tile::{Tile, TileType}};

#[derive(Clone,Copy,Debug)]
pub struct BuildMenu {
    pub data: [[Option<BuildChoice>; 9]; 4],
}

impl BuildMenu {
    pub fn new(inv: &Inventory) -> BuildMenu {
        let mut menu = BuildMenu {
            data: [[None; 9]; 4],
        };
        menu.data[0][0] = Some(BuildChoice::new(Tile::new(TileType::Planks.id()),1));
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
