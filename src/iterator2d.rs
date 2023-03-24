

struct Next<T> {
    next_line: bool,
    value: T
}

trait Iterator2d {
    type Item;

    fn next(&mut self) -> Option<Next<Self::Item>>;


    fn map<B, F>(self, f: F) -> Map2d<Self, F>
    where
        Self: Sized,
        F: FnMut(impl Iterator<Item = Self::Item>) -> B,
    {
        Map2d::new(self, f)
    }
}


pub struct Map2d<I, F> {
    // Used for `SplitWhitespace` and `SplitAsciiWhitespace` `as_str` methods
    pub(crate) iter: I,
    f: F,
}

impl<I, F> Map2d<I, F> {
    pub fn new(iter: I, f: F) -> Map2d<I, F> {
        Map2d { iter, f }
    }
}

//impl<I: fmt::Debug, F> fmt::Debug for Map<I, F> {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        f.debug_struct("Map").field("iter", &self.iter).finish()
//    }
//}

impl<B, I: Iterator2d, F> Iterator for Map2d<I, F>
where
    F: FnMut(impl Iterator<Item = I::Item>) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn try_fold<Acc, G, R>(&mut self, init: Acc, g: G) -> R
    where
        Self: Sized,
        G: FnMut(Acc, Self::Item) -> R,
        R: Try<Output = Acc>,
    {
        self.iter.try_fold(init, map_try_fold(&mut self.f, g))
    }

    fn fold<Acc, G>(self, init: Acc, g: G) -> Acc
    where
        G: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.fold(init, map_fold(self.f, g))
    }

    #[inline]
    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> B
    where
        Self: TrustedRandomAccessNoCoerce,
    {
        // SAFETY: the caller must uphold the contract for
        // `Iterator::__iterator_get_unchecked`.
        unsafe { (self.f)(try_get_unchecked(&mut self.iter, idx)) }
    }
}