use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static EVENT_LOOP_CREATED: AtomicBool = AtomicBool::new(false);

pub struct EventLoop<T: 'static> {
    pub(crate) event_loop: platform_impl::EventLoop<T>,
    pub(crate) _marker: PhantomData<*mut ()>, // Not Send nor Sync
}

impl EventLoop<()> {
    /// Alias for [`EventLoopBuilder::new().build()`].
    ///
    /// [`EventLoopBuilder::new().build()`]: EventLoopBuilder::build
    #[inline]
    pub fn new() -> Result<EventLoop<()>, EventLoopError> {
        EventLoopBuilder::new().build()
    }
}
