use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub tipo: TileType,
}

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Turf,
    Grass,
}

pub struct TileSet {
    pub imgs: [Option<Texture2D>; 2]
}

impl TileSet {
    pub async fn new() -> TileSet {
        let mut ts = TileSet{
            imgs: [None; 2]
        };
        ts.imgs[0] = Some(load_texture("assets/turf.png").await.unwrap());
        ts
    }
}
