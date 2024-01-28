#[derive(Debug)]
pub struct ErrorReporter {
    happened: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self { happened: false }
    }
    pub fn clear(&mut self) {
        self.happened = false
    }

    pub fn report(&mut self) {
        self.happened = true
    }

    pub fn happened(&mut self) -> bool {
        self.happened
    }

}
