use crate::item::{Item, ItemType};

#[derive(Clone,Copy,Debug)]
pub struct Register {
    pub pos: (usize, usize),
    pub id: usize,
}

impl Register {
    pub fn new(pos: (usize, usize), id: usize) -> Register {
        Register { pos, id }
    }

    pub fn tipo(&self) -> ShopType {
        match self.id {
            0 => {ShopType::Basic}
            _ => {ShopType::Basic}
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub struct ShopItem {
    pub item: Item,
    pub cost: i32,
}

impl ShopItem {
    pub fn new(item: Item, cost: i32) -> ShopItem {
        ShopItem { item, cost }
    }
}

#[derive(Clone,Copy,Debug)]
pub enum ShopType {
    Basic
}

impl ShopType {
    pub fn id(&self) -> usize {
        match self {
            ShopType::Basic => {0}
        }
    }

    pub fn shop_items(&self) -> [ShopItem;3] {
        match self {
            ShopType::Basic => {[
                ShopItem::new(Item::new(ItemType::Wax.id(),1),10),
                ShopItem::new(Item::new(ItemType::Dirt.id(),25),-4),
                ShopItem::new(Item::new(ItemType::Log.id(),10),-4),
            ]}
        }
    }
}
