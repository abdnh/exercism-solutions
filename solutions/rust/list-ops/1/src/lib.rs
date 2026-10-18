/// Yields each item of a and then each item of b
pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator + 'static,
    J: Iterator<Item = I::Item> + 'static,
{
    std::iter::from_fn(move || a.next().or_else(|| b.next()))
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator + 'static,
    I::Item: Iterator + 'static,
{
    let mut nested_iter = nested_iter.peekable();
    std::iter::from_fn(move || {
        loop {
            if let Some(iter) = nested_iter.peek_mut() {
                let item = iter.next();
                if item.is_none() {
                    nested_iter.next();
                } else {
                    return item;
                }
            } else {
                return None;
            }
        }
    })
}

struct FilterIterator<I, F>(I, F);

impl<T, I, F> Iterator for FilterIterator<I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> bool,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(item) = self.0.next() {
                if self.1(&item) {
                    return Some(item);
                }
            } else {
                return None;
            }
        }
    }
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator + 'static,
    F: Fn(&I::Item) -> bool + 'static,
{
    FilterIterator(iter, predicate)
}

pub fn length<I: Iterator>(mut iter: I) -> usize {
    let mut count = 0;
    while let Some(_) = iter.next() {
        count += 1;
    }

    count
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator + 'static,
    F: Fn(I::Item) -> U + 'static,
{
    std::iter::from_fn(move || {
        iter.next().map(|item| function(item))
    })
}

pub fn foldl<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut value = initial;
    while let Some(item) = iter.next() {
        value = function(value, item);
    }
    value
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut value = initial;
    while let Some(item) = iter.next_back() {
        value = function(value, item);
    }
    value
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    std::iter::from_fn(move || iter.next_back())
}
