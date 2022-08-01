use crate::{person::Person, shop::Register};

#[derive(Clone,Debug)]
pub struct Seal {
    pub pos: (usize, usize),
    pub owner: Option<Person>,
    pub register: Option<Register>,
}

impl Seal {
    pub fn new(pos: (usize, usize)) -> Seal {
        Seal {
            pos,
            owner: None,
            register: None,
        }
    }
}
