#[derive(Clone,Copy,Debug)]
pub struct TileRecipe {
    pub id: usize,
}

impl TileRecipe {
    pub fn new(id: usize) -> TileRecipe {
        TileRecipe{id}
    }
}
