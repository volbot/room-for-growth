use crate::item::Item;

#[derive(Clone,Copy,Debug)]
pub struct Inventory {
    pub data: [[Option<Item>; 9]; 4],
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            data: [[None; 9]; 4],
        }
    }

    pub fn push(&mut self, item: Item) {
        let mut i = 0;
        let mut j = 0;
        while i < 9 {
            while j < 4 {
                if self.data[j][i].is_some() {
                    let mut curr_item = self.data[j][i].unwrap();
                    if curr_item.id == item.id {
                        curr_item.quant += item.quant;
                    }
                    self.data[j][i] = Some(curr_item);
                    return
                }
                j += 1;
            }
            j = 0;
            i += 1;
        }
        i = 0;
        j = 0;
        while i < 9 {
            while j < 4 {
                if self.data[j][i].is_none() {
                    self.data[j][i] = Some(item);
                    return
                }
                j += 1;
            }
            j = 0;
            i += 1;
        }
    }
}
