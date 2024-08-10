use core::iter::Peekable;
use core::ops::ControlFlow;

/// An iterator adaptor that maps items while a closure returns successfully.
///
/// See [`.peeking_map()`](crate::Itertools::peeking_map)
/// for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct PeekingMap<'a, I, Accept, Map, Output>
where
    I: Iterator,
    Map: FnOnce(I::Item) -> Output,
    Accept: FnMut(&mut I::Item) -> ControlFlow<Option<Output>, Map>,
{
    iter: &'a mut I,
    accept: Accept,
}

impl<I, Accept, Map, Output> std::fmt::Debug for PeekingMap<'_, I, Accept, Map, Output>
where
    I: Iterator + std::fmt::Debug,
    Map: FnOnce(I::Item) -> Output,
    Accept: FnMut(&mut I::Item) -> ControlFlow<Option<Output>, Map>,
{
    debug_fmt_fields!(PeekingMap, iter);
}

impl<'a, I, Accept, Map, Output> PeekingMap<'a, I, Accept, Map, Output>
where
    I: Iterator,
    Map: FnOnce(I::Item) -> Output,
    Accept: FnMut(&mut I::Item) -> ControlFlow<Option<Output>, Map>,
{
    pub(crate) fn new(iter: &'a mut I, f: Accept) -> Self {
        Self { iter, accept: f }
    }
}

impl<I, Accept, Map, Output> Iterator for PeekingMap<'_, Peekable<I>, Accept, Map, Output>
where
    I: Iterator,
    Map: FnOnce(I::Item) -> Output,
    Accept: FnMut(&mut I::Item) -> ControlFlow<Option<Output>, Map>,
{
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.accept)(self.iter.peek_mut()?) {
            ControlFlow::Continue(map) => Some(map(self.iter.next()?)),
            ControlFlow::Break(res) => res,
        }
    }
}
