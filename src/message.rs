pub struct WorldMessage {
    pub msg: String,
    pub timer: i32,
    pub limit: i32,
    pub pos: (usize, usize)
}

impl WorldMessage {
    pub fn new(msg: &str, pos: (usize, usize)) -> WorldMessage {
        WorldMessage {
            msg: msg.to_string(),
            timer: 0,
            limit: 135,
            pos,
        }
    }
}
