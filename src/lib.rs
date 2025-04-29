/// Find existing installation directories
pub mod existing_dirs;

/// Reading data from executables
pub mod execlookup;

/// Initializing a new retail game install from the official retail installer. No execution required!
#[cfg(feature = "game_install")]
pub mod installer;
