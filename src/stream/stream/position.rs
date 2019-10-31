use std::pin::Pin;

use crate::future::Future;
use crate::stream::Stream;
use crate::task::{Context, Poll};

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct PositionFuture<'a, S, P> {
    stream: &'a mut S,
    predicate: P,
    pos: usize,
}

impl<'a, S, P> PositionFuture<'a, S, P> {
    pub(super) fn new(stream: &'a mut S, predicate: P) -> Self {
        Self {
            stream,
            predicate,
            pos: 0,
        }
    }
}

impl<S: Unpin, P> Unpin for PositionFuture<'_, S, P> {}

impl<'a, S, P> Future for PositionFuture<'a, S, P>
    where
        S: Stream + Sized + Unpin,
        P: FnMut(&S::Item) -> bool,
{
    type Output = Option<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let item = futures_core::ready!(Pin::new(&mut *self.stream).poll_next(cx));

        match item {
            Some(v) if (&mut self.predicate)(&v) => Poll::Ready(Some(self.pos)),
            Some(_) => {
                cx.waker().wake_by_ref();
                self.pos += 1;
                Poll::Pending
            }
            None => Poll::Ready(None),
        }
    }
}
