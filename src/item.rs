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
}
