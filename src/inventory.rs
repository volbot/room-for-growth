use crate::item::Item;

#[derive(Clone,Copy,Debug)]
pub struct Inventory {
    pub data: [[Option<Item>; 9]; 4],
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            data: [[None; 9]; 4],
        }
    }
}
