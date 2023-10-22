pub trait LogOnError {
    fn log_on_err(self) -> Self;
}

impl<T, E: std::fmt::Debug> LogOnError for Result<T, E> {
    fn log_on_err(self) -> Self {
        if let Err(ref e) = self {
            log::error!("Error occurred: {:?}", e);
        }
        self
    }
}