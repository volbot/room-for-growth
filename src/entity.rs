pub struct Entity {
    pub tipo: EntityType,
    pub pos: (usize, usize),
    pub tex_id: usize,
}

impl Entity {
    pub fn new(pos: (usize, usize), tex_id: usize) -> Entity {
        Entity {
            tipo: EntityType::Player,
            pos, tex_id,
        }
    }
}

pub enum EntityType {
    Player,
}
