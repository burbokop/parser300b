

#[derive(Clone)]
pub struct AtEnd<I: Iterator, F> {
    pub(crate) iter: I,
    f: Option<F>,
    cache: Option<Vec<I::Item>>
}

impl<I: Iterator, F> AtEnd<I, F> {
    #[inline]
    pub fn new(iter: I, f: F) -> AtEnd<I, F> {
        AtEnd { iter: iter, f: Some(f), cache: Some(Vec::new()) }
    }
}

impl<I, F> Iterator for AtEnd<I, F>
    where
        I: Iterator,
        I::Item: Clone,
        F: FnOnce(Vec<I::Item>),
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => { 
                if let Some(ref mut cache) = self.cache {
                    cache.push(val.clone());
                }
                Some(val)
            },
            None => { (self.f.take().unwrap())(self.cache.take().unwrap()); None },
        }
    }
}

pub trait AtEndabled: Sized + Iterator {
    fn at_end<F>(self, f: F) -> AtEnd<Self, F>
    where 
        F: FnOnce(Vec<Self::Item>);
}

impl<T, I> AtEndabled for I
where
    I: Iterator<Item = T>,
{
    #[inline]
    fn at_end<F>(self, f: F) -> AtEnd<Self, F>
    where 
        F: FnOnce(Vec<Self::Item>) 
    {
        AtEnd::new(self, f)
    }
}