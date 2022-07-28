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
        let mut first_none: Option<(usize, usize)> = None;
        while i < 4 {
            while j < 9 {
                if self.data[i][j].is_some() {
                    let mut curr_item = self.data[i][j].unwrap();
                    if curr_item.id == item.id {
                        curr_item.quant += item.quant;
                        self.data[i][j] = Some(curr_item);
                        return
                    }
                } else {
                    if first_none.is_none() {
                        first_none = Some((i, j));
                    }
                }
                j += 1;
            }
            j = 0;
            i += 1;
        }
        if first_none.is_some() {
            let coords = first_none.unwrap();
            self.data[coords.0][coords.1]  = Some(item);
        }
    }

    pub fn pop(&mut self, item: Item) {
        let mut work_item = item.clone();
        let mut i = 0;
        let mut j = 0;
        while i < 4 {
            while j < 9 {
                if self.data[i][j].is_some() {
                    let mut curr_item = self.data[i][j].unwrap();
                    if curr_item.id == item.id {
                        if curr_item.quant >= work_item.quant {
                            curr_item.quant -= work_item.quant;
                            self.data[i][j] = Some(curr_item);
                            return
                        } else {
                            work_item.quant -= curr_item.quant;
                            self.data[i][j] = None;
                        }
                    }
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
