#[derive(Clone,Copy,Debug)]
pub struct Entity {
    pub pos: (usize, usize),
    pub tex_id: usize,
}

impl Entity {
    pub fn new(pos: (usize, usize), tex_id: usize) -> Entity {
        Entity {
            pos, tex_id,
        }
    }

    pub fn distance(&self, entity: &Entity) -> isize {
        let dist = (entity.pos.0.abs_diff(self.pos.0) + entity.pos.1.abs_diff(self.pos.1)) as isize;
        return dist
    }
}
