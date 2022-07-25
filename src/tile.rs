use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub tipo: TileType,
}

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Grass,
    Wall,
    Water,
}

pub struct TileSet {
    pub imgs: [Option<Texture2D>; 5]
}

impl TileSet {
    pub async fn new() -> TileSet {
        let mut ts = TileSet{
            imgs: [None; 5]
        };
        ts.imgs[0] = Some(load_texture("assets/turf.png").await.unwrap());
        ts.imgs[1] = Some(load_texture("assets/gunder.png").await.unwrap());
        ts.imgs[2] = Some(load_texture("assets/wall.png").await.unwrap());
        ts.imgs[3] = Some(load_texture("assets/shortstack.png").await.unwrap());
        ts.imgs[4] = Some(load_texture("assets/water.png").await.unwrap());
        ts
    }
}

pub fn is_walkable(tile: Tile) -> bool{
    match tile.tipo {
        TileType::Grass => {true},
        _ => {false}
    }
}
