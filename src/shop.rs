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
pub enum ShopType {
    Basic
}

impl ShopType {
    pub fn id(&self) -> usize {
        match self {
            ShopType::Basic => {0}
        }
    }
}
