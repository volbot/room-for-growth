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
}
