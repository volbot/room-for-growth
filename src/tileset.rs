use macroquad::{prelude::*, ui::{Skin, root_ui}};

pub struct TileSet {
    pub tiles: [Option<Texture2D>; 8],
    pub people: [Option<Texture2D>; 2],
    pub icons: [Option<Texture2D>; 3],
    pub windows: [Option<Texture2D>; 5],
    pub items: [Option<Texture2D>; 3],
    pub skins: [Skin; 2],
    pub font: Font,
}

impl TileSet {
    pub async fn new() -> TileSet {
        let font_bytes = include_bytes!("../assets/fonts/JMH Cthulhumbus Arcade.otf");
        let tex1 = load_texture("assets/ui/windows/button_bg.png").await.unwrap().get_texture_data();
        let tex2 = load_texture("assets/ui/windows/garbage_button.png").await.unwrap().get_texture_data();
        let button_style_1 = root_ui().style_builder()
            .background(tex1)
            .background_margin(RectOffset::new(67., 67., 18., 18.))
            .font(font_bytes).unwrap()
            .build();
        let button_style_2 = root_ui().style_builder()
            .background(tex2)
            .background_margin(RectOffset::new(40.,0.,40.,0.))
            .build();
        let skin1 = Skin {
            button_style: button_style_1, ..root_ui().default_skin()
        };
        let skin2 = Skin {
            button_style: button_style_2, ..root_ui().default_skin()
        };
        let skins = [skin1, skin2];
        let mut ts = TileSet{
            tiles: [None; 8],
            people: [None; 2],
            icons: [None; 3],
            windows: [None; 5],
            items: [None; 3],
            skins,
            font: load_ttf_font("assets/fonts/JMH Cthulhumbus Arcade.otf").await.unwrap(),
        };
        ts.tiles[0] = Some(load_texture("assets/tiles/turf.png").await.unwrap());
        ts.tiles[1] = Some(load_texture("assets/tiles/wall.png").await.unwrap());
        ts.tiles[2] = Some(load_texture("assets/tiles/water.png").await.unwrap());
        ts.tiles[3] = Some(load_texture("assets/tiles/woodplank.png").await.unwrap());
        ts.tiles[4] = Some(load_texture("assets/tiles/woodboards.png").await.unwrap());
        ts.tiles[5] = Some(load_texture("assets/tiles/brush.png").await.unwrap());
        ts.tiles[6] = Some(load_texture("assets/tiles/seal.png").await.unwrap());
        ts.tiles[7] = Some(load_texture("assets/tiles/register.png").await.unwrap());

        ts.people[0] = Some(load_texture("assets/entities/people/gunder.png").await.unwrap());
        ts.people[1] = Some(load_texture("assets/entities/people/shortstack.png").await.unwrap());
        
        ts.icons[0] = Some(load_texture("assets/ui/ingame/new_info.png").await.unwrap());
        ts.icons[1] = Some(load_texture("assets/ui/ingame/good_info.png").await.unwrap());
        ts.icons[2] = Some(load_texture("assets/ui/ingame/dec_info.png").await.unwrap());

        ts.windows[0] = Some(load_texture("assets/ui/windows/button_bg.png").await.unwrap());
        ts.windows[1] = Some(load_texture("assets/ui/windows/popup_bg.png").await.unwrap());
        ts.windows[2] = Some(load_texture("assets/ui/windows/inventory_bg.png").await.unwrap());
        ts.windows[3] = Some(load_texture("assets/ui/windows/selected.png").await.unwrap());
        ts.windows[4] = Some(load_texture("assets/ui/windows/garbage_button.png").await.unwrap());

        ts.items[0] = Some(load_texture("assets/items/logs.png").await.unwrap());
        ts.items[1] = Some(load_texture("assets/items/dirt.png").await.unwrap());
        ts.items[2] = Some(load_texture("assets/items/wax.png").await.unwrap());

        ts
    }
}
