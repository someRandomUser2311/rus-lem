use std::pin::Pin;
use std::task::{Context, Poll};

pub struct YieldCount {
    pending: u32,
    value: u32,
    polls: u32,
}

impl YieldCount {
    pub fn new(pending: u32, value: u32) -> YieldCount {
        YieldCount {
            pending,
            value,
            polls: 0,
        }
    }
}

impl Future for YieldCount {
    type Output = (u32, u32);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.pending > 0 {
            this.polls += 1;
            this.pending -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready((this.value, this.polls))
        }
    }
}

pub async fn count_yields(pending: u32, value: u32) -> (u32, u32) {
    YieldCount::new(pending, value).await
}
