#[derive(Clone,Copy,Debug)]
pub struct Item {
    pub id: usize,
    pub quant: usize,
}

impl Item {
    pub fn new(id: usize, quant: usize) -> Item {
        Item {
            id, quant
        }
    }

    pub fn name(&self) -> &str {
        match self.id {
            0 => {"Log"}
            1 => {"Dirt"}
            2 => {"Wax"}
            _ => {"None"}
        }
    }

    pub fn tipo (&self) -> ItemType {
        match self.id {
            0 => {ItemType::Log}
            1 => {ItemType::Dirt}
            2 => {ItemType::Wax}
            _ => {ItemType::Log}
        }
    }
}

pub enum ItemType {
    Log,
    Dirt,
    Wax,
}

impl ItemType {
    pub fn id(&self) -> usize {
        match self {
            ItemType::Log => {0}
            ItemType::Dirt => {1}
            ItemType::Wax => {2}
        }
    }
}
