use std::fmt::Debug;

pub struct Sorting<T: PartialOrd + Clone> {
    pub array: Vec<T>,
}

pub enum Order {
    Ascending,
    Descending,
}

impl Order {
    pub fn comparator<T: PartialOrd>(&self, a: &T, b: &T) -> bool {
        match self {
            Order::Ascending => a < b,
            Order::Descending => a > b,
        }
    }
}

impl<T: PartialOrd + Clone> Sorting<T> {
    pub fn selection(&mut self, order: Order) {
        for i in 0..self.array.len() {
            let mut min_idx = i;
            for j in i + 1..self.array.len() {
                if order.comparator(&self.array[j], &self.array[j + 1]) {
                    min_idx = j;
                }
            }
            self.array.swap(i, min_idx);
        }
    }

    pub fn bubble(&mut self, order: Order) {
        let mut had_swap_in_pass;
        for i in 0..self.array.len() - 1 {
            had_swap_in_pass = false;
            for j in 0..self.array.len() - i - 1 {
                if order.comparator(&self.array[j], &self.array[j + 1]) {
                    self.array.swap(j, j + 1);
                    had_swap_in_pass = true;
                }
            }
            if !had_swap_in_pass {
                break;
            }
        }
    }

    pub fn insertion(&mut self, order: Order) {
        for i in 1..self.array.len() {
            let k = self.array[i].clone();
            let mut j = i - 1;
            while order.comparator(&self.array[i], &self.array[j]) {
                self.array[j + 1] = self.array[j].clone();
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            self.array[j + 1] = k;
        }
    }
}
