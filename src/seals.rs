use crate::{person::Person, shop::Register};

#[derive(Clone,Debug)]
pub struct Seal {
    pub pos: (usize, usize),
    pub tipo: SealType,
    pub owner: Option<Person>,
    pub register: Option<Register>,
}

impl Seal {
    pub fn new(pos: (usize, usize), tipo: SealType) -> Seal {
        Seal {
            pos, tipo,
            owner: None,
            register: None,
        }
    }

    pub fn merchant_pos(&self) -> Option<(usize, usize)> {
        if self.register.is_none() {
            return None
        }
        let reg = self.register.unwrap();
        let xdist = reg.pos.0 as i32 - self.pos.0 as i32;
        let ydist = reg.pos.1 as i32 - self.pos.1 as i32;
        if xdist.abs() > ydist.abs() {
            return Some((if xdist >  0 {
                reg.pos.0 + 1
            } else {
                reg.pos.0 - 1
            }, reg.pos.1))
        } else {
            return Some((reg.pos.0, if ydist > 0 {
                reg.pos.1 + 1
            } else {
                reg.pos.1 - 1
            }))
        }
    }
}

#[derive(Clone,Debug)]
pub enum SealType {
    Shop,
    House,
}
