use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct HistoryList<T> {
    max_capacity: usize,
    data: VecDeque<T>,
}

impl<T> HistoryList<T> {
    pub fn new(max_capacity: usize) -> Self {
        HistoryList {
            max_capacity,
            data: VecDeque::new(),
        }
    }
    /// push a new element onto the History.
    /// If the history is at max-capacity, remove the oldest element, and return it.
    pub fn push(&mut self, elem: T) -> Option<T> {
        self.data.push_front(elem);
        if self.data.len() > self.max_capacity {
            self.data.pop_back()
        } else {
            None
        }
    }

    pub fn pop_newest(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn newest(&self) -> Option<&T> {
        self.data.get(0)
    }
}
