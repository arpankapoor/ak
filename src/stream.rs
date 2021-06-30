pub struct Stream<'a, T> {
    pub slice: &'a [T],
    pub index: Option<usize>,
}

impl<'a, T> Stream<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        Self { slice, index: None }
    }

    pub fn prev(&self) -> Option<T> {
        self.slice.get(self.index.filter(|&i| i != 0)? - 1)
    }

    pub fn curr(&self) -> Option<T> {
        self.slice.get(self.index?)
    }

    pub fn peek(&self) -> Option<T> {
        self.slice
            .get(self.index.map_or(0, |i| i.saturating_add(1)))
    }

    pub fn peek_next(&self) -> Option<T> {
        self.slice
            .get(self.index.map_or(1, |i| i.saturating_add(2)))
    }

    pub fn next_if<F: FnOnce(T) -> bool>(&mut self, func: F) -> Option<T> {
        match self.peek() {
            Some(x) if func(x) => self.next(),
            _ => None,
        }
    }

    pub fn next_if_eq(&mut self, expected: T) -> Option<T>
    where
        T: PartialEq,
    {
        self.next_if(|x| x == expected)
    }

    pub fn consume_while(&mut self, func: fn(&T) -> bool) -> usize {
        let mut count = 0;
        while self.next_if(func).is_some() {
            count += 1;
        }
        count
    }
}

impl<'a, T> Iterator for Stream<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self
            .index
            .map_or(0, |i| i.saturating_add(1).min(self.slice.len()));
        self.index = Some(index);
        self.slice.get(index)
    }
}

#[cfg(test)]
mod test {
    use super::Stream;

    #[test]
    fn bytestream_test() {
        let mut test = Stream::new(b"hello");
        assert_eq!(test.prev(), None);
        assert_eq!(test.curr(), None);
        assert_eq!(test.peek(), Some(&b'h'));
        assert_eq!(test.peek_next(), Some(&b'e'));

        assert_eq!(test.next(), Some(&b'h'));
        assert_eq!(test.prev(), None);
        assert_eq!(test.curr(), Some(&b'h'));
        assert_eq!(test.peek(), Some(&b'e'));
        assert_eq!(test.peek_next(), Some(&b'l'));

        assert_eq!(test.next(), Some(&b'e'));
        assert_eq!(test.prev(), Some(&b'h'));
        assert_eq!(test.curr(), Some(&b'e'));
        assert_eq!(test.peek(), Some(&b'l'));
        assert_eq!(test.peek_next(), Some(&b'l'));

        assert_eq!(test.next(), Some(&b'l'));
        assert_eq!(test.prev(), Some(&b'e'));
        assert_eq!(test.curr(), Some(&b'l'));
        assert_eq!(test.peek(), Some(&b'l'));
        assert_eq!(test.peek_next(), Some(&b'o'));

        assert_eq!(test.next(), Some(&b'l'));
        assert_eq!(test.prev(), Some(&b'l'));
        assert_eq!(test.curr(), Some(&b'l'));
        assert_eq!(test.peek(), Some(&b'o'));
        assert_eq!(test.peek_next(), None);

        assert_eq!(test.next(), Some(&b'o'));
        assert_eq!(test.prev(), Some(&b'l'));
        assert_eq!(test.curr(), Some(&b'o'));
        assert_eq!(test.peek(), None);
        assert_eq!(test.peek_next(), None);

        assert_eq!(test.next(), None);
        assert_eq!(test.prev(), Some(&b'o'));
        assert_eq!(test.curr(), None);
        assert_eq!(test.peek(), None);
        assert_eq!(test.peek_next(), None);

        assert_eq!(test.next(), None);
        assert_eq!(test.prev(), Some(&b'o'));
        assert_eq!(test.curr(), None);
        assert_eq!(test.peek(), None);
        assert_eq!(test.peek_next(), None);
    }

    #[test]
    fn bytestream_test_next_if() {
        let mut test = Stream::new(b"hello");
        assert_eq!(test.next_if(|&x| x == b'h'), Some(&b'h'));
        assert_eq!(test.next_if(|&x| x == b'o'), None);
        assert_eq!(test.next(), Some(&b'e'));
        assert_eq!(test.next_if(|&x| x == b'l'), Some(&b'l'));
        assert_eq!(test.next_if(|&x| x == b'l'), Some(&b'l'));
        assert_eq!(test.next_if(|&x| x == b'o'), Some(&b'o'));
        assert_eq!(test.next_if(|&x| x == b'o'), None);
        assert_eq!(test.next(), None);
    }

    #[test]
    fn bytestream_test_next_if_eq() {
        let mut test = Stream::new(b"hello");
        assert_eq!(test.next_if_eq(&b'h'), Some(&b'h'));
        assert_eq!(test.next_if_eq(&b'o'), None);
        assert_eq!(test.next(), Some(&b'e'));
        assert_eq!(test.next_if_eq(&b'l'), Some(&b'l'));
        assert_eq!(test.next_if_eq(&b'l'), Some(&b'l'));
        assert_eq!(test.next_if_eq(&b'o'), Some(&b'o'));
        assert_eq!(test.next_if_eq(&b'o'), None);
        assert_eq!(test.next(), None);
    }
}
