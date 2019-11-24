use super::ir::Op;

pub struct PeekableCode<'a, I: std::iter::Iterator<Item = &'a Op>> {
    iter: I,
    peek_val: Option<&'a Op>,
}

impl<'a, I: std::iter::Iterator<Item = &'a Op>> PeekableCode<'a, I> {
    pub fn new(iter: I) -> Self {
        let mut iter = iter;
        let peek = iter.next();
        Self {
            iter,
            peek_val: peek,
        }
    }

    pub fn peek(&mut self) -> Option<&Op> {
        self.peek_val
    }

    pub fn next(&mut self) -> Option<&Op> {
        let val = self.peek_val;
        self.peek_val = self.iter.next();
        val
    }
}
