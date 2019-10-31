use std::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct Copied<S> {
        #[pin]
        stream: S,
    }
}

impl<S> Copied<S> {
    pub(crate) fn new(stream: S) -> Self {
        Self {
            stream,
        }
    }
}

impl<S> Stream for Copied<S>
where
    S: Stream + Sized,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let next = futures_core::ready!(this.stream.poll_next(cx));
        Poll::Ready(next)
    }
}
