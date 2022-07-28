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
        let mut possible = (inv.item_count(0) as f32/4.).floor() as usize;
        menu.data[0][0] = Some(BuildChoice::new(Tile::new(TileType::Planks.id()),possible as usize));
        possible = (inv.item_count(0) as f32/3.).floor() as usize;
        menu.data[0][1] = Some(BuildChoice::new(Tile::new(TileType::Boards.id()),possible as usize)); 
        possible = (inv.item_count(1) as f32/4.).floor() as usize;
        menu.data[0][2] = Some(BuildChoice::new(Tile::new(TileType::Grass.id()),possible as usize));
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
