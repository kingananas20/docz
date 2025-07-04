mod doc;
mod error;

pub use doc::Doc;
pub use error::Error;

// logging for tests
#[cfg(test)]
pub(crate) mod logging {
    use std::sync::Once;

    static INIT_LOGGER: Once = Once::new();

    pub fn init_logger() {
        INIT_LOGGER.call_once(|| {
            env_logger::builder()
                .format_timestamp(None)
                .filter_level(log::LevelFilter::Debug)
                .is_test(true)
                .init();
        });
    }
}
