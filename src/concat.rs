use std::borrow::Borrow;

pub trait IteratorConcatExt<U>: Iterator where Self: Iterator {
    fn concat(self) -> U;
    fn join(self, sep: &str) -> U;
}

impl<I> IteratorConcatExt<String> for I where I: Iterator, I::Item: Borrow<str> {
    fn concat(self) -> String {
        self.collect::<Vec<_>>().concat()
    }

    fn join(self, sep: &str) -> String {
        self.collect::<Vec<_>>().join(sep)
    }
}

