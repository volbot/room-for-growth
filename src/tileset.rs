use macroquad::{prelude::*, ui::{Skin, root_ui}};

pub struct TileSet {
    pub tiles: [Option<Texture2D>; 12],
    pub people: [Option<Texture2D>; 4],
    pub icons: [Option<Texture2D>; 4],
    pub windows: [Option<Texture2D>; 6],
    pub items: [Option<Texture2D>; 5],
    pub mine: [Option<Texture2D>; 4],
    pub skins: [Skin; 2],
    pub textpar: [TextParams; 6],
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
        let header = TextParams {
            font_size: 14,
            font: load_ttf_font_from_bytes(font_bytes).unwrap(),
            color: BLACK,
            ..Default::default()
        };
        let info = TextParams {
            font_size: 12,
            font: load_ttf_font_from_bytes(font_bytes).unwrap(),
            color: BLACK,
            ..Default::default()
        };
        let body = TextParams {
            font_size: 20,
            font: load_ttf_font_from_bytes(font_bytes).unwrap(),
            color: BLACK,
            ..Default::default()
        };
        let bighead = TextParams {
            font_size: 30,
            font: load_ttf_font_from_bytes(font_bytes).unwrap(),
            color: BLACK,
            ..Default::default()
        };
        let uitext = TextParams {
            font_size: 18,
            font: load_ttf_font_from_bytes(font_bytes).unwrap(),
            color: WHITE,
            ..Default::default()
        };
        let msgtext = TextParams {
            font_size: 12,
            font_scale: 1.,
            font: load_ttf_font_from_bytes(font_bytes).unwrap(),
            color: WHITE,
            ..Default::default()
        };
        let textpar = [header, info, body, bighead, uitext, msgtext];
        let skins = [skin1, skin2];
        let mut ts = TileSet{
            tiles: [None; 12],
            people: [None; 4],
            icons: [None; 4],
            windows: [None; 6],
            items: [None; 5],
            mine: [None; 4],
            skins,textpar,
            font: load_ttf_font_from_bytes(font_bytes).unwrap(),
        };
        ts.tiles[0] = Some(load_texture("assets/tiles/turf.png").await.unwrap());
        ts.tiles[1] = Some(load_texture("assets/tiles/wall.png").await.unwrap());
        ts.tiles[2] = Some(load_texture("assets/tiles/water.png").await.unwrap());
        ts.tiles[3] = Some(load_texture("assets/tiles/woodplank.png").await.unwrap());
        ts.tiles[4] = Some(load_texture("assets/tiles/woodboards.png").await.unwrap());
        ts.tiles[5] = Some(load_texture("assets/tiles/brush.png").await.unwrap());
        ts.tiles[6] = Some(load_texture("assets/tiles/shopseal.png").await.unwrap());
        ts.tiles[7] = Some(load_texture("assets/tiles/register.png").await.unwrap());
        ts.tiles[8] = Some(load_texture("assets/tiles/homeseal.png").await.unwrap());
        ts.tiles[9] = Some(load_texture("assets/tiles/path.png").await.unwrap());
        ts.tiles[10] = Some(load_texture("assets/tiles/techreg.png").await.unwrap());
        ts.tiles[11] = Some(load_texture("assets/tiles/beacon.png").await.unwrap());

        ts.people[0] = Some(load_texture("assets/entities/people/gunder.png").await.unwrap());
        ts.people[1] = Some(load_texture("assets/entities/people/shortstack.png").await.unwrap());
        ts.people[2] = Some(load_texture("assets/entities/people/inspector.png").await.unwrap());
        ts.people[3] = Some(load_texture("assets/entities/people/mechy.png").await.unwrap());

        ts.icons[0] = Some(load_texture("assets/ui/ingame/new_info.png").await.unwrap());
        ts.icons[1] = Some(load_texture("assets/ui/ingame/good_info.png").await.unwrap());
        ts.icons[2] = Some(load_texture("assets/ui/ingame/dec_info.png").await.unwrap());
        ts.icons[3] = Some(load_texture("assets/ui/world/indic.png").await.unwrap());

        ts.windows[0] = Some(load_texture("assets/ui/windows/button_bg.png").await.unwrap());
        ts.windows[1] = Some(load_texture("assets/ui/windows/popup_bg.png").await.unwrap());
        ts.windows[2] = Some(load_texture("assets/ui/windows/inventory_bg.png").await.unwrap());
        ts.windows[3] = Some(load_texture("assets/ui/windows/selected.png").await.unwrap());
        ts.windows[4] = Some(load_texture("assets/ui/windows/garbage_button.png").await.unwrap());
        ts.windows[5] = Some(load_texture("assets/ui/windows/shopslot.png").await.unwrap());

        ts.mine[0] = Some(load_texture("assets/ui/world/mine/mine0.png").await.unwrap());
        ts.mine[1] = Some(load_texture("assets/ui/world/mine/mine1.png").await.unwrap());
        ts.mine[2] = Some(load_texture("assets/ui/world/mine/mine2.png").await.unwrap());
        ts.mine[3] = Some(load_texture("assets/ui/world/mine/mine3.png").await.unwrap());

        ts.items[0] = Some(load_texture("assets/items/logs.png").await.unwrap());
        ts.items[1] = Some(load_texture("assets/items/dirt.png").await.unwrap());
        ts.items[2] = Some(load_texture("assets/items/wax.png").await.unwrap());
        ts.items[3] = Some(load_texture("assets/items/chip.png").await.unwrap());
        ts.items[4] = Some(load_texture("assets/items/antenna.png").await.unwrap());

        ts
    }
}
