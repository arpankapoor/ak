#[derive(Clone)]
pub struct ByteStream<'a> {
    slice: &'a [u8],
    index: Option<usize>,
}

impl<'a> ByteStream<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self { slice, index: None }
    }

    pub fn next_index(&self) -> usize {
        self.index.map_or(0, |i| self.slice.len().min(i + 1))
    }

    pub fn slice(&self, start: usize) -> &'a [u8] {
        &self.slice[start..self.next_index()]
    }

    pub fn prev(&self) -> Option<u8> {
        self.slice.get(self.index.filter(|&i| i != 0)? - 1).copied()
    }

    pub fn curr(&self) -> Option<u8> {
        self.slice.get(self.index?).copied()
    }

    pub fn peek(&self) -> Option<u8> {
        self.slice.get(self.index.map_or(0, |i| i + 1)).copied()
    }

    pub fn peek_next(&self) -> Option<u8> {
        self.slice.get(self.index.map_or(1, |i| i + 2)).copied()
    }

    pub fn next_if<F: FnOnce(u8) -> bool>(&mut self, func: F) -> Option<u8> {
        match self.peek() {
            Some(x) if func(x) => self.next(),
            _ => None,
        }
    }

    pub fn next_if_eq(&mut self, expected: u8) -> Option<u8> {
        self.next_if(|x| x == expected)
    }

    pub fn consume_while(&mut self, func: fn(u8) -> bool) -> usize {
        let mut count = 0;
        while self.next_if(func).is_some() {
            count += 1;
        }
        count
    }
}

impl<'a> Iterator for ByteStream<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let index = self.next_index();
        self.index = Some(index);
        self.slice.get(index).copied()
    }
}

#[cfg(test)]
mod test {
    use super::ByteStream;

    #[test]
    fn bytestream_test() {
        let mut test = ByteStream::new(b"hello");
        assert_eq!(test.prev(), None);
        assert_eq!(test.curr(), None);
        assert_eq!(test.peek(), Some(b'h'));
        assert_eq!(test.peek_next(), Some(b'e'));

        assert_eq!(test.next(), Some(b'h'));
        assert_eq!(test.prev(), None);
        assert_eq!(test.curr(), Some(b'h'));
        assert_eq!(test.peek(), Some(b'e'));
        assert_eq!(test.peek_next(), Some(b'l'));

        assert_eq!(test.next(), Some(b'e'));
        assert_eq!(test.prev(), Some(b'h'));
        assert_eq!(test.curr(), Some(b'e'));
        assert_eq!(test.peek(), Some(b'l'));
        assert_eq!(test.peek_next(), Some(b'l'));

        assert_eq!(test.next(), Some(b'l'));
        assert_eq!(test.prev(), Some(b'e'));
        assert_eq!(test.curr(), Some(b'l'));
        assert_eq!(test.peek(), Some(b'l'));
        assert_eq!(test.peek_next(), Some(b'o'));

        assert_eq!(test.next(), Some(b'l'));
        assert_eq!(test.prev(), Some(b'l'));
        assert_eq!(test.curr(), Some(b'l'));
        assert_eq!(test.peek(), Some(b'o'));
        assert_eq!(test.peek_next(), None);

        assert_eq!(test.next(), Some(b'o'));
        assert_eq!(test.prev(), Some(b'l'));
        assert_eq!(test.curr(), Some(b'o'));
        assert_eq!(test.peek(), None);
        assert_eq!(test.peek_next(), None);

        assert_eq!(test.next(), None);
        assert_eq!(test.prev(), Some(b'o'));
        assert_eq!(test.curr(), None);
        assert_eq!(test.peek(), None);
        assert_eq!(test.peek_next(), None);

        assert_eq!(test.next(), None);
        assert_eq!(test.prev(), Some(b'o'));
        assert_eq!(test.curr(), None);
        assert_eq!(test.peek(), None);
        assert_eq!(test.peek_next(), None);
    }

    #[test]
    fn bytestream_test_next_if() {
        let mut test = ByteStream::new(b"hello");
        assert_eq!(test.next_if(|x| x == b'h'), Some(b'h'));
        assert_eq!(test.next_if(|x| x == b'o'), None);
        assert_eq!(test.next(), Some(b'e'));
        assert_eq!(test.next_if(|x| x == b'l'), Some(b'l'));
        assert_eq!(test.next_if(|x| x == b'l'), Some(b'l'));
        assert_eq!(test.next_if(|x| x == b'o'), Some(b'o'));
        assert_eq!(test.next_if(|x| x == b'o'), None);
        assert_eq!(test.next(), None);
    }

    #[test]
    fn bytestream_test_next_if_eq() {
        let mut test = ByteStream::new(b"hello");
        assert_eq!(test.next_if_eq(b'h'), Some(b'h'));
        assert_eq!(test.next_if_eq(b'o'), None);
        assert_eq!(test.next(), Some(b'e'));
        assert_eq!(test.next_if_eq(b'l'), Some(b'l'));
        assert_eq!(test.next_if_eq(b'l'), Some(b'l'));
        assert_eq!(test.next_if_eq(b'o'), Some(b'o'));
        assert_eq!(test.next_if_eq(b'o'), None);
        assert_eq!(test.next(), None);
    }
}
