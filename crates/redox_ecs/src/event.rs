/// Internal storage for events of type T.
///
/// Uses double buffering to ensure events last for 2 frames.
pub struct Events<T: 'static + Send + Sync> {
    buffers: [Vec<T>; 2],
    current_buffer: usize,
}

impl<T: 'static + Send + Sync> Events<T> {
    /// Creates a new event queue.
    pub fn new() -> Self {
        Self {
            buffers: [Vec::with_capacity(128), Vec::with_capacity(128)],
            current_buffer: 0,
        }
    }

    /// Swaps the buffers and clears the old write buffer.
    /// Should be called once per frame.
    pub fn update(&mut self) {
        let next_buffer = (self.current_buffer + 1) % 2;
        self.buffers[next_buffer].clear();
        self.current_buffer = next_buffer;
    }

    /// Sends an event into the current buffer.
    pub fn send(&mut self, event: T) {
        self.buffers[self.current_buffer].push(event);
    }

    /// Returns an iterator over all pending events (both buffers).
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffers[1 - self.current_buffer].iter().chain(self.buffers[self.current_buffer].iter())
    }
}

/// Reader for events of type T.
///
/// Keeps track of its own read position, allowing multiple readers to consume
/// the same events independently.
pub struct EventReader<'a, T: 'static + Send + Sync> {
    events: &'a Events<T>,
    _last_read_id: usize, // Could be extended to track position per buffer
}

impl<'a, T: 'static + Send + Sync> EventReader<'a, T> {
    /// Creates a new event reader.
    pub fn new(events: &'a Events<T>) -> Self {
        Self { events, _last_read_id: 0 }
    }

    /// Returns an iterator over all pending events.
    /// In a real implementation this should track last-read position.
    /// For MVP we simply iterate over all events (simplified).
    pub fn iter(&mut self) -> impl Iterator<Item = &T> {
        self.events.iter()
    }
}