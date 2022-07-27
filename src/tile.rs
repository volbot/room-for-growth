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
            _ => {TileType::Grass}
        }
    }
    pub fn resources(&self) -> Item {
        match self.id {
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
            3 | 4 | 5 => {
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
            _ => {"None"}
        }
    }
    pub fn is_walkable(&self) -> bool{
        match self.id {
            0 | 4 | 5 => {true},
            _ => {false}
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
        }
    }
}
