use crossbeam_channel::unbounded;
use std::io;
use std::task::Poll;
use std::{future::Future, sync::Arc};
use tokio::sync::oneshot::{Receiver, Sender};
struct UringTask {
    complete: Option<Sender<i32>>,
}

struct UringPoolFuture {
    // ring: Arc<io_uring::IoUring>,
    ring: Arc<io_uring::IoUring>,
    finish: crossbeam_channel::Receiver<()>,
}

struct UringContextInner {}

#[derive(Clone)]
pub struct UringContext {
    inner: Arc<UringContextInner>,
}
