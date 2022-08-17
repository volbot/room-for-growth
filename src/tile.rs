use macroquad::prelude::*;

use crate::item::Item;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub id: usize,
}

impl Tile {
    pub fn new(id: usize) -> Tile {
        Tile {
            id
        }
    }
    pub fn tipo(&self) -> TileType {
        match self.id {
            0 => {TileType::Grass}
            1 => {TileType::Wall}
            2 => {TileType::Water}
            3 => {TileType::Planks}
            4 => {TileType::Boards}
            5 => {TileType::Brush}
            6 => {TileType::ShopSeal}
            7 => {TileType::Register}
            8 => {TileType::HomeSeal}
            9 => {TileType::Path}
            10 => {TileType::TechReg}
            11 => {TileType::Beacon}
            _ => {TileType::Grass}
        }
    }
    pub fn resources(&self) -> Item {
        match self.id {
            0 => {
                Item::new(1,4)
            }
            9 => {
                Item::new(0,0)
            }
            7 | 10 => {
                Item::new(0,10)
            }
            6 | 8 => {
                Item::new(2,1)
            }
            5 => {
                Item::new(0,5)
            }
            3 => {
                Item::new(0,4)
            }
            4 => {
                Item::new(0,3)
            }
            _ => {
                Item::new(0,0)
            }
        }
    }
    pub fn is_mineable(&self) -> bool {
        match self.id {
            0 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => {
                true
            }
            _ => {
                false
            }
        }
    }
    pub fn name(&self) -> &str {
        match self.id {
            0 => {"Grass"}
            1 => {"Wall"}
            2 => {"Water"}
            3 => {"Planks"}
            4 => {"Boards"}
            5 => {"Brush"}
            6 => {"Shop Seal"}
            7 => {"Register"}
            8 => {"Home Seal"}
            9 => {"Path"}
            10 => {"Tech Shop"}
            11 => {"Beacon"}
            _ => {"None"}
        }
    }
    pub fn is_walkable(&self) -> bool{
        match self.id {
            0 | 4 | 5 | 6 | 8 | 9 => {true},
            _ => {false}
        }
    }
    pub fn under_id(&self) -> usize {
        match self.id {
            0 => {TileType::Water.id()}
            6 | 7 | 8 | 10 => {TileType::Boards.id()}
            _ => {TileType::Grass.id()}
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Grass,
    Wall,
    Water,
    Planks,
    Boards,
    Brush,
    ShopSeal,
    HomeSeal,
    Register,
    Path,
    TechReg,
    Beacon,
}

impl TileType {
    pub fn id(&self) -> usize {
        match self {
            TileType::Grass => {0}
            TileType::Wall => {1}
            TileType::Water => {2}
            TileType::Planks => {3}
            TileType::Boards => {4}
            TileType::Brush => {5}
            TileType::ShopSeal => {6}
            TileType::Register => {7}
            TileType::HomeSeal => {8}
            TileType::Path => {9}
            TileType::TechReg => {10}
            TileType::Beacon => {11}
        }
    }
}
