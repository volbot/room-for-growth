use crate::person::Person;

#[derive(Clone,Copy,Debug)]
pub struct Seal {
    pub pos: (usize, usize),
    pub owner: Option<Person>,
}

impl Seal {
    pub fn new(pos: (usize, usize)) -> Seal {
        Seal {
            pos,
            owner: None,
        }
    }
}
