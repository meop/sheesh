#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use unix::{alert, spawn_target};
#[cfg(windows)]
pub use windows::{alert, spawn_target};
