#[derive(Debug)]
pub(crate) struct Stack<T> {
    size: usize,
    data: Vec<T>,
}


impl<T> Stack<T> {
    pub(crate) fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub(crate) fn len(&self) -> usize {
        self.size
    }

    pub(crate) fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    pub(crate) fn push(&mut self, value: T) {
        self.data.push(value);
        self.size += 1;
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        if 0 == self.size {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    pub(crate) fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            return None;
        }
        self.data.get(self.size - 1)
    }

    pub(crate) fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }

    pub(crate) fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub(crate) fn iter(&self) -> Iter<T> {
        let mut iterator = Iter {
            stack: Vec::new(),
        };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    pub(crate) fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut {
            stack: Vec::new(),
        };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }

        iterator
    }
}


pub(crate) struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

pub(crate) struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub(crate) struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}