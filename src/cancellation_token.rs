pub struct CancellationToken {
    cancelled: bool,
}

impl CancellationToken {
    pub fn new() -> CancellationToken {
        Self { cancelled: false }
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}
