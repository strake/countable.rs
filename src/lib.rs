#![no_std]

extern crate either;
#[doc(hidden)]
pub extern crate itertools;
extern crate void;

use core::iter;
use either::Either;
use itertools::Itertools;
use void::Void;

pub trait Countable: Sized {
    type Counter: Iterator<Item = Self>;
    fn count() -> Self::Counter;
}

impl Countable for Void {
    type Counter = iter::Empty<Void>;
    #[inline]
    fn count() -> iter::Empty<Void> { iter::empty() }
}

impl Countable for () {
    type Counter = iter::Once<()>;
    #[inline]
    fn count() -> iter::Once<()> { iter::once(()) }
}

impl<A: Countable> Countable for Option<A> {
    type Counter = iter::Chain<iter::Once<Self>, iter::Map<A::Counter, fn(A) -> Self>>;
    #[inline]
    fn count() -> Self::Counter { iter::once(None).chain(A::count().map(Some as _)) }
}

impl<A: Countable, B: Countable> Countable for Either<A, B> {
    type Counter = itertools::Interleave<iter::Map<A::Counter, fn(A) -> Either<A, B>>, iter::Map<B::Counter, fn(B) -> Either<A, B>>>;
    #[inline]
    fn count() -> Self::Counter {
        itertools::interleave(A::count().map(Either::Left as _), B::count().map(Either::Right as _))
    }
}

impl<A: Clone + Countable, B: Countable> Countable for (A, B)
  where B::Counter: Clone {
    type Counter = itertools::Product<A::Counter, B::Counter>;
    #[inline]
    fn count() -> itertools::Product<A::Counter, B::Counter> {
        Itertools::cartesian_product(A::count(), B::count())
    }
}

pub trait EndlessIterator: Iterator {
    fn next(&mut self) -> Self::Item;
}

impl<As: Iterator, Bs: EndlessIterator<Item = As::Item>> EndlessIterator for iter::Chain<As, Bs> {
    #[inline]
    fn next(&mut self) -> Self::Item { Iterator::next(self).unwrap() }
}

impl<As: EndlessIterator, B, F: FnMut(As::Item) -> B> EndlessIterator for iter::Map<As, F> {
    #[inline]
    fn next(&mut self) -> Self::Item { Iterator::next(self).unwrap() }
}
