use std::cmp::Ordering;

pub struct Borrowed<'a, I: Iterator> {
    iter: &'a mut I,
}

impl<'a, I: Iterator> Borrowed<'a, I> {
    pub fn new(iter: &'a mut I) -> Self {
        Self { iter }
    }
}

impl<'a, I: Iterator> Iterator for Borrowed<'a, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'a, I: ExactSizeIterator> ExactSizeIterator for Borrowed<'a, I> {}

pub struct Inserted<'a, T> {
    array: &'a [T],
    item: Option<T>,
    insert_index: usize,
    current_index: usize,
}

impl<'a, T> Inserted<'a, T> {
    pub fn new(array: &'a [T], item: T, insert_index: usize) -> Self {
        Self {
            array,
            item: Some(item),
            insert_index,
            current_index: 0,
        }
    }
}

impl<'a, T: Clone> Iterator for Inserted<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.current_index;

        if idx > self.array.len() {
            return None;
        }

        self.current_index += 1;

        match idx.cmp(&self.insert_index) {
            Ordering::Less => Some(self.array[idx].clone()),
            Ordering::Equal => self.item.take(),
            Ordering::Greater => Some(self.array[idx - 1].clone()),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.array.len() + 1 - self.current_index;
        (size, Some(size))
    }
}

impl<'a, T: Clone> ExactSizeIterator for Inserted<'a, T> {}

pub struct Replaced<'a, T> {
    array: &'a [T],
    item: Option<T>,
    insert_index: usize,
    current_index: usize,
}

impl<'a, T> Replaced<'a, T> {
    pub fn new(array: &'a [T], item: T, insert_index: usize) -> Self {
        Self {
            array,
            item: Some(item),
            insert_index,
            current_index: 0,
        }
    }
}

impl<'a, T: Clone> Iterator for Replaced<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.current_index;

        if idx >= self.array.len() {
            return None;
        }

        self.current_index += 1;

        match idx.cmp(&self.insert_index) {
            Ordering::Less => Some(self.array[idx].clone()),
            Ordering::Equal => self.item.take(),
            Ordering::Greater => Some(self.array[idx].clone()),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.array.len() - self.current_index;
        (size, Some(size))
    }
}

impl<'a, T: Clone> ExactSizeIterator for Replaced<'a, T> {}
