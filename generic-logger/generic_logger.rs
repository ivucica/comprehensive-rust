pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, verbosity: u8, message: &str) {
        println!("verbosity={verbosity}: {message}");
    }
}

struct VerbosityFilter {
    max_verbosity: u8,
    inner: Box<dyn Logger>,
}
impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message)
        }
    }
}


fn main() {
    let logger = VerbosityFilter { max_verbosity: 3, inner: Box::new(StdoutLogger) };
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}