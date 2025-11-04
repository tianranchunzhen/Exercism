pub use append::append;
pub use concat::concat;
pub use filter::filter;
pub use foldl::foldl;
pub use foldr::foldr;
pub use length::length;
pub use map::map;
pub use reverse::reverse;

mod append {
    /// Yields each item of a and then each item of b
    pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
    where
        I: Iterator,
        J: Iterator<Item = I::Item>,
    {
        Append {
            first: a,
            second: b,
        }
    }

    struct Append<I, J> {
        first: I,
        second: J,
    }

    impl<I, J> Iterator for Append<I, J>
    where
        I: Iterator,
        J: Iterator<Item = I::Item>,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.first.next().or_else(|| self.second.next())
        }
    }
}

mod concat {
    /// Combines all items in all nested iterators inside into one flattened iterator
    pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
    where
        I: Iterator,
        I::Item: Iterator,
    {
        Concat {
            nested_iter,
            current_iter: None,
        }
    }

    struct Concat<I>
    where
        I: Iterator,
        I::Item: Iterator,
    {
        nested_iter: I,
        current_iter: Option<I::Item>,
    }

    impl<I> Iterator for Concat<I>
    where
        I: Iterator,
        I::Item: Iterator,
    {
        type Item = <I::Item as Iterator>::Item;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                if let Some(currenct_iter) = &mut self.current_iter {
                    if let Some(item) = currenct_iter.next() {
                        return Some(item);
                    } else {
                        self.current_iter = None;
                    }
                } else if let Some(next_iter) = self.nested_iter.next() {
                    self.current_iter = Some(next_iter);
                } else {
                    return None;
                }
            }
        }
    }
}

mod filter {
    /// Returns an iterator of all items in iter for which `predicate(item)` is true
    pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
    where
        I: Iterator,
        F: Fn(&I::Item) -> bool,
    {
        Filter { iter, predicate }
    }

    struct Filter<I, F> {
        iter: I,
        predicate: F,
    }

    impl<I, F> Iterator for Filter<I, F>
    where
        I: Iterator,
        F: Fn(&I::Item) -> bool,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            for next in self.iter.by_ref() {
                if (self.predicate)(&next) {
                    return Some(next);
                } else {
                    continue;
                }
            }
            None
        }
    }
}

mod length {
    pub fn length<I: Iterator>(iter: I) -> usize {
        let mut length = 0;
        for _ in iter {
            length += 1;
        }
        length
    }
}

mod map {
    /// Returns an iterator of the results of applying `function(item)` on all iter items
    pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
    where
        I: Iterator,
        F: Fn(I::Item) -> U,
    {
        Map { iter, function }
    }

    struct Map<I, F> {
        iter: I,
        function: F,
    }

    impl<I, F, U> Iterator for Map<I, F>
    where
        I: Iterator,
        F: Fn(I::Item) -> U,
    {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            Some((self.function)(self.iter.next()?))
        }
    }
}

mod foldl {
    pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
    where
        I: Iterator,
        F: Fn(U, I::Item) -> U,
    {
        let mut res = initial;
        for item in iter {
            res = function(res, item);
        }
        res
    }
}

mod foldr {
    pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
    where
        I: DoubleEndedIterator,
        F: Fn(U, I::Item) -> U,
    {
        let mut res = initial;
        while let Some(item) = iter.next_back() {
            res = function(res, item);
        }
        res
    }
}

mod reverse {
    /// Returns an iterator with all the original items, but in reverse order
    pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
        // this empty iterator silences a compiler complaint that
        // () doesn't implement Iterator
        Rev(iter)
    }

    struct Rev<I>(I);

    impl<I: DoubleEndedIterator> Iterator for Rev<I> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next_back()
        }
    }
}
