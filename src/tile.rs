use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub tipo: TileType,
}

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Grass,
    Wall,
}

pub struct TileSet {
    pub imgs: [Option<Texture2D>; 3]
}

impl TileSet {
    pub async fn new() -> TileSet {
        let mut ts = TileSet{
            imgs: [None; 3]
        };
        ts.imgs[0] = Some(load_texture("assets/turf.png").await.unwrap());
        ts.imgs[1] = Some(load_texture("assets/gunder.png").await.unwrap());
        ts.imgs[2] = Some(load_texture("assets/wall.png").await.unwrap());
        ts
    }
}

pub fn is_walkable(tile: Tile) -> bool{
    match tile.tipo {
        TileType::Grass => {true},
        _ => {false}
    }
}
