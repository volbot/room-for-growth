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

    pub fn pop(&mut self, item: Item) {
        let mut work_item = item.clone();
        let mut i = 0;
        let mut j = 0;
        while i < 9 {
            while j < 4 {
                if self.data[j][i].is_some() {
                    let mut curr_item = self.data[j][i].unwrap();
                    if curr_item.id == item.id {
                        if curr_item.quant >= work_item.quant {
                            curr_item.quant -= work_item.quant;
                            self.data[j][i] = Some(curr_item);
                            return
                        } else {
                            work_item.quant -= curr_item.quant;
                            self.data[j][i] = None;
                        }
                    }
                    return
                }
                j += 1;
            }
            j = 0;
            i += 1;
        }
    }

    pub fn item_count(&self, id: usize) -> isize {
        let mut total = 0;
        for arr in self.data {
            for slot in arr {
                if slot.is_some() {
                    let item = slot.unwrap();
                    if item.id == id {
                        total += item.quant;
                    }
                }
            }
        }
        total as isize
    }
}
