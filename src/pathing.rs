use crate::world::World;
use crate::tile::TileType;

pub fn heuristic(pos: (i32, i32), goal: (i32, i32), world: &World) -> i32 {
    let tile = world.data[pos.0 as usize][pos.1 as usize];
    if tile.is_walkable() {
        let h = ((goal.0.abs_diff(pos.0) + goal.1.abs_diff(pos.1)) / 3) as i32;
        match tile.tipo() {
            TileType::Brush => {
                h*2+4
            }
            TileType::Path => {
                (h/2-4).max(0)
            }
            _ => {
                h
            }
        }
    } else {
        1000000
    }
}

pub fn successors(pos: (i32, i32), goal: (i32, i32), world: &World) -> Vec<(i32, i32)> {
    let x = pos.0;
    let y = pos.1;
    let mut vec = vec![(x+1, y), (x-1, y), (x, y+1), (x, y-1)];
    let mut i = 0;
    while i < vec.len() {
        let curr = vec.get(i).unwrap();
        if curr.0 < world.data.len() as i32 && curr.1 < world.data[0].len() as i32 && //WorldLim
            curr.0 >= 0 && curr.1 >= 0 &&                                       //WorldLim
                (world.data[curr.0 as usize][curr.1 as usize].is_walkable() || //Walkable, or
                curr.0 == goal.0 && curr.1 == goal.1) {                     //Last in path
                    i+=1;
                } else {
                    vec.remove(i);
                }
    }
    vec
}

pub fn successors_inside(pos: (i32, i32), world: &World) -> Vec<(i32, i32)> {
    let x = pos.0;
    let y = pos.1;
    let mut vec = vec![(x+1, y), (x-1, y), (x, y+1), (x, y-1)];
    let mut i = 0;
    while i < vec.len() {
        let curr = vec.get(i).unwrap();
        if curr.0 < world.data.len() as i32 && curr.1 < world.data[0].len() as i32 && //WorldLim
            curr.0 >= 0 && curr.1 >= 0 { //WorldLim
                    let id = world.data[curr.0 as usize][curr.1 as usize].id;
                    if id == TileType::Boards.id() || id == TileType::ShopSeal.id() || id == TileType::HomeSeal.id() {
                        i += 1;
                        continue
                    }
        }
        vec.remove(i);
    }
    vec
}
