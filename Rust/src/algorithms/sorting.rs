pub struct Sorting<T: PartialOrd> {
    pub array: Vec<T>,
}

pub enum Order {
    Ascending,
    Descending,
}

impl<T: PartialOrd> Sorting<T> {
    pub fn selection(&mut self, order: Order) {
        for i in 0..self.array.len() {
            let mut min_idx = i;
            for j in i + 1..self.array.len() {
                match order {
                    Order::Ascending => {
                        if self.array[j] < self.array[j + 1] {
                            min_idx = j;
                        }
                    }
                    Order::Descending => {
                        if self.array[j] > self.array[j + 1] {
                            min_idx = j;
                        }
                    }
                }
            }
            self.array.swap(i, min_idx);
        }
    }
}
