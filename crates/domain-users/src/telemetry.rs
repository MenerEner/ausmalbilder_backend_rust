#[cfg(feature = "tracing")]
pub use tracing::{debug as ddebug, error as derror, info as dinfo, warn as dwarn};

#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! ddebug {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! dinfo {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! dwarn {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! derror {
    ($($arg:tt)*) => {};
}
