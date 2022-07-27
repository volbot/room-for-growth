use macroquad::prelude::*;

pub struct TileSet {
    pub tiles: [Option<Texture2D>; 6],
    pub people: [Option<Texture2D>; 2],
    pub icons: [Option<Texture2D>; 3],
    pub windows: [Option<Texture2D>; 3],
    pub font: Font,
}

impl TileSet {
    pub async fn new() -> TileSet {
        let mut ts = TileSet{
            tiles: [None; 6],
            people: [None; 2],
            icons: [None; 3],
            windows: [None; 3],
            font: load_ttf_font("assets/fonts/JMH Cthulhumbus Arcade.otf").await.unwrap(),
        };
        ts.tiles[0] = Some(load_texture("assets/tiles/turf.png").await.unwrap());
        ts.tiles[1] = Some(load_texture("assets/tiles/wall.png").await.unwrap());
        ts.tiles[2] = Some(load_texture("assets/tiles/water.png").await.unwrap());
        ts.tiles[3] = Some(load_texture("assets/tiles/woodplank.png").await.unwrap());
        ts.tiles[4] = Some(load_texture("assets/tiles/woodboards.png").await.unwrap());
        ts.tiles[5] = Some(load_texture("assets/tiles/brush.png").await.unwrap());

        ts.people[0] = Some(load_texture("assets/entities/people/gunder.png").await.unwrap());
        ts.people[1] = Some(load_texture("assets/entities/people/shortstack.png").await.unwrap());
        
        ts.icons[0] = Some(load_texture("assets/ui/ingame/new_info.png").await.unwrap());
        ts.icons[1] = Some(load_texture("assets/ui/ingame/good_info.png").await.unwrap());
        ts.icons[2] = Some(load_texture("assets/ui/ingame/dec_info.png").await.unwrap());

        ts.windows[0] = Some(load_texture("assets/ui/windows/button_bg.png").await.unwrap());
        ts.windows[1] = Some(load_texture("assets/ui/windows/popup_bg.png").await.unwrap());
        ts.windows[2] = Some(load_texture("assets/ui/windows/inventory_bg.png").await.unwrap());

        ts
    }
}
