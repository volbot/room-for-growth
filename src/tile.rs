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
    Planks,
    Boards,
}

pub struct TileSet {
    pub tiles: [Option<Texture2D>; 5],
    pub people: [Option<Texture2D>; 2],
    pub icons: [Option<Texture2D>; 3],
    pub windows: [Option<Texture2D>; 2],
}

impl TileSet {
    pub async fn new() -> TileSet {
        let mut ts = TileSet{
            tiles: [None; 5],
            people: [None; 2],
            icons: [None; 3],
            windows: [None; 2],
        };
        ts.tiles[0] = Some(load_texture("assets/tiles/turf.png").await.unwrap());
        ts.tiles[1] = Some(load_texture("assets/tiles/wall.png").await.unwrap());
        ts.tiles[2] = Some(load_texture("assets/tiles/water.png").await.unwrap());
        ts.tiles[3] = Some(load_texture("assets/tiles/woodplank.png").await.unwrap());
        ts.tiles[4] = Some(load_texture("assets/tiles/woodboards.png").await.unwrap());

        ts.people[0] = Some(load_texture("assets/entities/people/gunder.png").await.unwrap());
        ts.people[1] = Some(load_texture("assets/entities/people/shortstack.png").await.unwrap());
        
        ts.icons[0] = Some(load_texture("assets/ui/ingame/new_info.png").await.unwrap());
        ts.icons[1] = Some(load_texture("assets/ui/ingame/good_info.png").await.unwrap());
        ts.icons[2] = Some(load_texture("assets/ui/ingame/dec_info.png").await.unwrap());
        
        ts.windows[0] = Some(load_texture("assets/ui/windows/button_bg.png").await.unwrap());
        ts.windows[1] = Some(load_texture("assets/ui/windows/popup_bg.png").await.unwrap());

        ts
    }
}

pub fn is_walkable(tile: Tile) -> bool{
    match tile.tipo {
        TileType::Grass | TileType::Boards => {true},
        _ => {false}
    }
}
