use crate::{item::Item, recipe::TileRecipe};

#[derive(Clone,Debug)]
pub struct Reward {
    pub items: Vec<Item>,
    pub tilerecipes: Vec<TileRecipe>,
}

impl Reward {
    pub fn new (items: Vec<Item>, tilerecipes: Vec<TileRecipe>) -> Reward {
        Reward {items, tilerecipes}
    }
}
