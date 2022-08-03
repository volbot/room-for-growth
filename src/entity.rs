#[derive(Clone,Debug)]
pub struct Entity {
    pub pos: (usize, usize),
    pub tex_id: usize,
    pub name: String,
}

impl Entity {
    pub fn new(name: &str, pos: (usize, usize), tex_id: usize) -> Entity {
        Entity {
            pos, tex_id, 
            name: name.to_string()
        }
    }

    pub fn distance(&self, entity: &Entity) -> isize {
        return self.distance_pos(entity.pos)
    }

    pub fn distance_pos(&self, pos: (usize, usize)) -> isize {
        let dist = (pos.0.abs_diff(self.pos.0) + pos.1.abs_diff(self.pos.1)) as isize;
        dist
    }
}
